use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;

use super::constants::*;
use crate::expand::ExpandCtxt;
use crate::expand::PathExt;
use crate::syntax::class::*;
use crate::syntax::method::*;

impl Methods {
    pub(super) fn expand_vtable(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let vtable = cx.vtable_name();
        let class = cx.class;
        let class_impl = cx.class_impl_name();

        let with_super = if let Some(_withs) = &class.class_attrs.withs {
            // Has with - use HasSuperVtable to get the mixin instance vtable
            // Note: This requires the Vtable impl to have the same bounds as HasSuperVtable
            Some(quote!(.with_super(
                <<#class_impl<__Self> as #_HasSuperVtable>::SuperVtable as #_Vtable>::TYPE
            )))
        } else if let Some(extends) = &class.class_attrs.extends {
            // Has extends but no with - directly use the parent class vtable
            // This doesn't require extra bounds beyond ClassConcrete
            let superclass = &extends.classes;
            Some(quote!(.with_super(<#_CVtable<#superclass, __Self> as #_Vtable>::TYPE)))
        } else {
            None
        };
        let with_interface = class
            .class_attrs
            .expand_interface_vtable_entries()
            .map(|interfaces| quote!(.with_interfaces(&[#interfaces])));

        // Generate Vtable impl for vtable
        let vtable_impl = quote! {
            impl<__Self: #_ClassConcrete> #_Vtable for #vtable<__Self> {
                const TYPE: #_VtableTypeInfo<'static> = #_VtableTypeInfo::Class(
                    &(#_ClassVtableTypeInfo::new(#_TypeName)
                        #with_super
                        #with_interface)
                );
            }

            impl<__Self: #_ClassConcrete> #_ClassVtable for #vtable<__Self> {}
        };

        // Generate super field if there's a parent class
        let super_field = class.class_attrs.expand_vtable_super_field(cx);

        // Generate method function pointer fields
        let method_fields = self.owns.iter().map(|method| {
            let ident = &method.ident;
            let fn_ptr = method.expand_fn_ptr(cx);
            quote! {
                pub #ident: #_Option<#fn_ptr>,
            }
        });

        // Generate interface fields for implements
        let interface_fields = if let Some(implements) = &class.class_attrs.implements {
            implements
                .classes
                .iter()
                .map(|interface| {
                    let ident = interface.interface_field();
                    quote!(#ident: #_CVtable<#interface, __Self>,)
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        let vtable_copy_bounds = quote!(<(__Self: #_ClassConcrete)> #vtable<__Self>);

        quote! {
            #[repr(C)]
            pub struct #vtable<__Self: #_ClassConcrete> {
                #super_field
                #(#interface_fields)*
                __self: #_PhantomData<__Self>,
                #(#method_fields)*
            }

            #_crate::impl_vtable_copy!(#vtable_copy_bounds);

            #vtable_impl
        }
    }

    pub(super) fn expand_vtable_init(&self, cx: &ExpandCtxt<'_>) -> syn::Expr {
        let vtable = cx.vtable_name();
        let class = cx.class;
        let class_impl = cx.class_impl_name();

        // Initialize super vtable (includes superclass and mixins if present)
        let super_init = if class.class_attrs.has_extends_or_withs() {
            quote! { __super: <#class_impl<__Self> as #_HasSuperVtableImpl>::SUPER_VTABLE, }
        } else {
            quote! {}
        };

        // Initialize method fields to None
        let method_inits = self.owns.iter().map(|method| {
            let ident = &method.ident;
            quote! { #ident: None }
        });

        // Initialize interface fields
        let interface_inits = if let Some(implements) = &class.class_attrs.implements {
            implements
                .classes
                .iter()
                .map(|interface| {
                    let ident = interface.interface_field();
                    quote! { #ident: <#_CClass<#interface, __Self> as #_HasVtableImpl>::VTABLE }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        // Initialize mixin fields
        syn::parse_quote! {
            #vtable {
                #super_init
                __self: #_PhantomData,
                #(#interface_inits,)*
                #(#method_inits,)*
            }
        }
    }

    pub(super) fn expand_vtable_override(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let overrides = self
            .overrides
            .iter()
            .filter(|method| method.method.body.body().is_some())
            .flat_map(|method| {
                let ident = &method.method.ident;
                method.r#override.classes.iter().map(move |superclass| {
                    quote!(#_write_vtable!((vtable as #superclass).#ident = Self::#ident);)
                })
            });
        let owns = self
            .owns
            .iter()
            .filter(|method| method.body.body().is_some())
            .map(|method| {
                let ident = &method.ident;
                if cx.class.kind.is_mixin() {
                    let mixin = &cx.class.name;
                    quote!(#_write_vtable!((vtable as #mixin).#ident = Self::#ident);)
                } else {
                    quote!(#_write_vtable!(vtable.#ident = Self::#ident);)
                }
            });

        // Only generate __downcast override for non-mixin classes
        let downcast = if matches!(cx.class.kind, ClassKind::Mixin(_)) {
            quote! {}
        } else {
            quote! {
                #_write_vtable!((vtable as Downcast).ty = Self::ty);
                #_write_vtable!((vtable as Downcast).__downcast = Self::__downcast);
            }
        };

        quote! {
            #(#overrides)*
            #(#owns)*
            #downcast
        }
    }
}

impl Methods {
    pub(super) fn expand_mixin_vtable_init(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let vtable = cx.vtable_name();
        let vtable_instance = cx.instance_vtable_name();
        let class = cx.class;

        // Only include implements interfaces, not on(...) interfaces
        let interface_inits = class.class_attrs.implements.iter().flat_map(|implements| {
            implements.classes.iter().map(|interface| {
                let field_name = interface.interface_field();
                quote!(#field_name: <#_CClass<#interface, __Self> as #_HasVtableImpl>::VTABLE)
            })
        });

        // Initialize method fields to None
        let method_inits = self.owns.iter().map(|method| {
            let ident = &method.ident;
            quote! { #ident: None }
        });

        quote! {
            #vtable_instance {
                __super: <__Super as #_HasVtableImpl>::VTABLE,
                __self: #vtable {
                    #(#interface_inits,)*
                    __self: #_PhantomData,
                    #(#method_inits,)*
                },
            }
        }
    }

    pub(super) fn expand_mixin_vtable(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let vtable = cx.vtable_name();
        let vtable_instance = cx.instance_vtable_name();

        // Generate method function pointer fields
        let method_fields = self.owns.iter().map(|method| {
            let ident = &method.ident;
            let fn_ptr = method.expand_fn_ptr(cx);
            quote!(pub #ident: #_Option<#fn_ptr>,)
        });

        // Only include implements interfaces, not on(...) interfaces
        let interface_fields = cx
            .class
            .class_attrs
            .implements
            .iter()
            .flat_map(|implements| {
                implements.classes.iter().map(|interface| {
                    let field_name = interface.interface_field();
                    quote!(#field_name: #_CVtable<#interface, __Self>,)
                })
            });

        // Generate interface entries for Vtable TYPE (only implements)
        let interface_entries = cx
            .class
            .class_attrs
            .implements
            .iter()
            .flat_map(|implements| {
                implements.classes.iter().map(|interface| {
                    let field_name = interface.interface_field();
                    quote! {
                        #_VtableOffsetEntry::new(
                            <#_CVtable<#interface, __Self> as #_Vtable>::TYPE.as_interface(),
                            #_offset_of!(Self, #field_name),
                        )
                    }
                })
            });

        quote! {
            #[repr(C)]
            pub struct #vtable_instance<__SuperVtable: #_Vtable, __Self: #_ClassConcrete> {
                __super: __SuperVtable,
                __self: #vtable<__Self>,
            }

            #[allow(non_snake_case)]
            #[repr(C)]
            pub struct #vtable<__Self: #_ClassConcrete> {
                #(#interface_fields)*
                __self: #_PhantomData<__Self>,
                #(#method_fields)*
            }

            #_crate::impl_vtable_copy!(<(__Self: #_ClassConcrete)> #vtable<__Self>);
            #_crate::impl_vtable_copy!(<(__SuperVtable: #_Vtable, __Self: #_ClassConcrete)> #vtable_instance<__SuperVtable, __Self>);

            #[automatically_derived]
            impl<__SuperVtable: #_Vtable, __Self: #_ClassConcrete> #_Vtable for #vtable_instance<__SuperVtable, __Self> {
                const TYPE: #_VtableTypeInfo<'static> = #_VtableTypeInfo::MixinInstance(
                    &#_MixinInstanceVtableTypeInfo::new(
                        <__SuperVtable as #_Vtable>::TYPE,
                        <#vtable<__Self> as #_MixinVtable>::MIXIN_TYPE,
                        #_offset_of!(Self, __self),
                    )
                );
            }

            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_Vtable for #vtable<__Self> {
                const TYPE: #_VtableTypeInfo<'static> = #_VtableTypeInfo::Mixin(
                    &#_MixinVtableTypeInfo::new(#_TypeName)
                        .with_interfaces(&[#(#interface_entries),*])
                );
            }

            #[automatically_derived]
            impl<__Super: #_Vtable, __Self: #_ClassConcrete> #_MixinInstanceVtable for #vtable_instance<__Super, __Self> {}

            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_MixinVtable for #vtable<__Self> {}
        }
    }
}

impl ClassAttrs {
    fn expand_vtable_super_field(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        if !self.has_extends_or_withs() {
            return quote! {};
        }

        let class_impl = cx.class_impl_name();
        quote! { __super: <#class_impl<__Self> as #_HasSuperVtable>::SuperVtable, }
    }

    fn expand_interface_vtable_entries(&self) -> Option<Punctuated<syn::Expr, syn::Token![,]>> {
        let Implements { classes, .. } = self.implements.as_ref()?;
        Some(
            classes
                .iter()
                .map(|interface| -> syn::Expr {
                    let ident = interface.interface_field();
                    syn::parse_quote! {
                        #_VtableOffsetEntry::new(
                            <#_CVtable<#interface, __Self> as #_Vtable>::TYPE.as_interface(),
                            #_offset_of!(Self, #ident),
                        )
                    }
                })
                .collect(),
        )
    }
}
