use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;

use super::constants::*;
use crate::expand::VisibilityExt;
use crate::{expand::ExpandCtxt, syntax::class::*};

impl Class {
    pub fn expand(&self) -> TokenStream {
        match &self.kind {
            ClassKind::Mixin(_) => self.expand_mixin(),
            _ => self.expand_class(),
        }
    }

    fn expand_class(&self) -> TokenStream {
        let cx = ExpandCtxt::new(self);

        let mod_name = cx.mod_name();
        let name = &self.name;
        let trait_name = cx.trait_name();
        let vis = &self.vis;

        let class_impl = self.items.expand_class_impl(&cx);
        let data = self.items.expand_data(&cx);
        let vtable = self.items.methods.expand_vtable(&cx);
        let trait_ = self.items.expand_trait(&cx);
        let concrete = self.items.expand_concrete(&cx);
        let fields = self.items.expand_field_accessors(&cx);
        let eq_hash_impls = self.expand_eq_hash_impls(&cx);
        let format_impls = self.expand_format_impls(&cx);
        let helpers: Vec<_> = self.expand_helper_impls(&cx).collect();
        let ctor_data_helpers: Vec<_> = self.expand_ctor_data_helpers(&cx).collect();
        let static_items = self.items.expand_static_items();
        let final_method_defs: Vec<syn::ImplItemFn> = self
            .items
            .methods
            .finals
            .iter()
            .filter_map(|method| method.expand_def_for_dyn(&cx))
            .collect();

        let dummy_concrete = concrete.is_none().then(|| quote!(type __Self = #_Dummy;));

        quote! {
            #[allow(unused_imports)]
            #vis use #mod_name::{#name, #trait_name};
            #[allow(non_snake_case)]
            #[allow(private_interfaces)]
            mod #mod_name {
                #[allow(unused_imports)]
                use #_crate::prelude::*;
                #[allow(unused_imports)]
                use super::*;
                const __TYPE_NAME: &str = concat!(module_path!(), "::", stringify!(#name));

                #class_impl

                #data

                #trait_

                #vtable

                #concrete
                #dummy_concrete

                impl #name {
                    #(#static_items)*
                    #(#final_method_defs)*
                }

                #eq_hash_impls

                #fields

                #format_impls

                #(#helpers)*

                #(#ctor_data_helpers)*
            }
        }
    }

    pub(super) fn expand_eq_hash_impls(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        if !self.has_eq_hash() {
            return quote!();
        }

        let name = &self.name;
        let concrete = cx.concrete_name();
        let is_abstract = self.is_abstract();

        let dyn_impls = quote! {
            #[automatically_derived]
            impl ::core::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    let self_: &#_EqHash = self;
                    let other: &#_EqHash = other;
                    #_IEqHash::eq(self_, other)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for #name {}
            #[automatically_derived]
            impl ::core::hash::Hash for #name {
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                    let self_: &#_EqHash = self;
                    #_IEqHash::hash(self_, state)
                }
            }
        };

        let concrete_impls = if !is_abstract {
            quote! {
                #[automatically_derived]
                impl ::core::cmp::PartialEq for #concrete {
                    fn eq(&self, other: &Self) -> bool {
                        let self_: &#_EqHash = self;
                        let other: &#_EqHash = other;
                        #_IEqHash::eq(self_, other)
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for #concrete {}
                #[automatically_derived]
                impl ::core::hash::Hash for #concrete {
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                        #_IEqHash::hash(self, state)
                    }
                }
            }
        } else {
            quote!()
        };

        quote! {
            #dyn_impls
            #concrete_impls
        }
    }

    pub(super) fn expand_format_impls(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        if !self.has_format() {
            return quote!();
        }

        let name = &self.name;
        let concrete = cx.concrete_name();
        let is_abstract = self.is_abstract();

        let dyn_impls = quote! {
            #[automatically_derived]
            impl ::core::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #_IFormat::fmt_debug(self, f)
                }
            }
        };

        let concrete_impls = if !is_abstract {
            quote! {
                #[automatically_derived]
                impl ::core::fmt::Debug for #concrete {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        #_IFormat::fmt_debug(self, f)
                    }
                }
            }
        } else {
            quote!()
        };

        quote! {
            #dyn_impls
            #concrete_impls
        }
    }
}

impl ClassItems {
    fn expand_class_impl(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let class = cx.class;
        let name = &class.name;
        let class_impl = cx.class_impl_name();
        let data = cx.data_name();
        let vtable = cx.vtable_name();

        let super_field = class.class_attrs.expand_class_impl_super_field();
        let class_impl_bounds = class.class_attrs.expand_concrete_with_trait_bounds(cx);
        let vtable_init = self.methods.expand_vtable_init(cx);
        let vtable_overrides = self.methods.expand_vtable_override(cx);
        let items = self.expand_item_defs(cx);

        // Generate HasSuper impl if there's a super class
        let has_super_and_deref_impl = class.class_attrs.expand_class_impl_has_super_and_deref(cx);
        // let typed_mixin_impls = cx.class.class_attrs.expand_typed_mixin_impls(cx);

        // Generate RcRef impl
        let rcref_impl = quote! {
            #[automatically_derived]
            unsafe impl<__Self: #_ClassConcrete + #_RcRef> #_RcRef for #class_impl<__Self> {}
        };

        let downcast_impl = self.expand_downcast_impl(cx);

        quote! {
            #[repr(C)]
            pub struct #class_impl<__Self: #_ClassConcrete> {
                #super_field
                __self: #_PhantomData<__Self>,
                data: #data,
            }

            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_HasData for #class_impl<__Self> {
                type Data = #data;
            }
            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_ClassHasData for #class_impl<__Self> {}
            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_HasVtable for #class_impl<__Self> {
                type Vtable = #vtable<__Self>;
            }
            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_ClassHasVtable for #class_impl<__Self> {}
            #[automatically_derived]
            impl<__Self: #class_impl_bounds> #_HasVtableImpl for #class_impl<__Self> {
                const VTABLE: Self::Vtable = #class_impl::<__Self>::__VTABLE;
            }
            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_ClassHasDyn for #class_impl<__Self> {
                type Dyn = #name;
            }

            #rcref_impl

            #(#has_super_and_deref_impl)*
            // #(#typed_mixin_impls)*

            #[automatically_derived]
            impl<__Self: #class_impl_bounds> #class_impl<__Self> {
                const __VTABLE: #vtable<__Self> = {
                    let mut vtable = #vtable_init;
                    Self::__override(&mut vtable);
                    vtable
                };

                #[allow(unused_variables)]
                const fn __override(vtable: &mut #vtable<__Self>) {
                    #vtable_overrides
                }

                #(#items)*

                #downcast_impl
            }
        }
    }

    fn expand_concrete(&self, cx: &ExpandCtxt<'_>) -> Option<TokenStream> {
        if cx.class.is_abstract() {
            return None;
        }
        let concrete = cx.concrete_name();
        let vtable = cx.vtable_name();
        let name = &cx.class.name;
        let class = cx.class_impl_name();
        let data = cx.data_name();

        let with_super =
            if cx.class.class_attrs.extends.is_some() || cx.class.class_attrs.withs.is_some() {
                quote! { .with_super::<Self>() }
            } else {
                quote! {}
            };

        let has_super = cx.class.class_attrs.expand_concrete_has_super(cx);

        let ctor_impls = self.ctors.iter().map(|ctor| ctor.expand_impl(cx));
        let method_impls = self
            .methods
            .iter()
            .filter_map(|method| method.expand_impl_in_concrete(cx));

        Some(quote! {
            type __Self = #concrete;

            #[repr(transparent)]
            pub struct #concrete(#class<__Self>);

            #[automatically_derived]
            impl #_Deref for #concrete {
                type Target = #name;
                fn deref(&self) -> &Self::Target {
                    self
                }
            }

            #[automatically_derived]
            impl #_ClassHasDyn for #concrete {
                type Dyn = #name;
            }
            #[automatically_derived]
            impl #_HasData for #concrete {
                type Data = #data;
            }
            #[automatically_derived]
            impl #_ClassHasData for #concrete {}
            #[automatically_derived]
            impl #_ClassConcreteOrDyn for #concrete {
                const TYPE: &#_ClassConcreteOrDynTypeInfo =
                    &#_ClassConcreteOrDynTypeInfo::new_concrete(#_TypeName) #with_super;
            }
            #[automatically_derived]
            impl #_ClassConcrete for #concrete {}
            #[automatically_derived]
            impl #_HasVtable for #concrete {
                type Vtable = #vtable<__Self>;
            }
            #[automatically_derived]
            impl #_HasVtableImpl for #concrete {
                const VTABLE: Self::Vtable = #class::<__Self>::__VTABLE;
            }
            #[automatically_derived]
            impl #_ClassHasVtable for #concrete {}

            #has_super

            #[automatically_derived]
            unsafe impl #_RcRef for #concrete {}

            impl #name {
                #(#ctor_impls)*
            }
            impl #concrete {
                #(#method_impls)*
            }
        })
    }

    pub(super) fn expand_static_items(&self) -> Vec<syn::ImplItem> {
        let mut items = Vec::new();
        items.extend(self.consts.iter().map(|item| {
            let mut item = item.clone();
            item.attrs.push(syn::parse_quote!(#[allow(dead_code)]));
            item.vis = item.vis.expand_for_super();
            syn::ImplItem::Const(item)
        }));
        items.extend(self.functions.iter().map(|item| {
            let mut item = item.clone();
            item.attrs.push(syn::parse_quote!(#[allow(dead_code)]));
            item.attrs.retain(|attr| !attr.path().is_ident("helper"));
            item.sig.inputs.iter_mut().for_each(|input| match input {
                syn::FnArg::Typed(pat_ty) => {
                    pat_ty.attrs.retain(|attr| !attr.path().is_ident("default"))
                }
                syn::FnArg::Receiver(_) => panic!("expected typed argument in helper"),
            });
            item.vis = item.vis.expand_for_super();
            syn::ImplItem::Fn(item)
        }));
        items
    }

    pub(super) fn expand_item_defs(&self, cx: &ExpandCtxt<'_>) -> Vec<syn::ImplItem> {
        let mut items = self.expand_static_items();
        items.extend(self.ctors.iter().map(|ctor| ctor.expand_def(cx)));
        items.extend(
            self.methods
                .owns
                .iter()
                .filter_map(|method| method.expand_def(cx, &[])),
        );
        items.extend(
            self.methods
                .overrides
                .iter()
                .filter_map(|method| method.expand_def(cx)),
        );
        items.extend(
            self.methods
                .finals
                .iter()
                .filter_map(|method| method.expand_def(cx, &[])),
        );
        items
    }

    pub(super) fn expand_downcast_impl(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let class = cx.class;
        let name = &class.name;

        let interfaces = class
            .class_attrs
            .implements
            .iter()
            .flat_map(|implements| implements.classes.iter())
            .map(|interface| quote!{
                if let #_Some(__vtable) = #_read_vtable!(try <#interface as #_Downcast>::__downcast)
                    && let #_Some(__super) = __vtable(__self, ty)
                {
                    return #_Some(__super);
                }
            });

        let downcast_impl_super =
            (class.kind.is_mixin() || class.class_attrs.has_extends_or_withs()).then(|| {
                quote! {
                    if let #_Some(__vtable) = #_read_vtable!(try <Super as #_Downcast>::__downcast)
                        && let #_Some(__super) = __vtable(__self, ty)
                    {
                        return #_Some(__super);
                    }
                }
            });

        quote! {
            fn ty(__self: &__Self) -> #_Type {
                #_Type::of::<#name>()
            }
            fn __downcast(__self: &__Self, ty: #_Type) -> Option<#_ErasedPtr> {
                if ty == #_Type::of::<#name>() {
                    return #_Some(unsafe { #_core::mem::transmute(__self as &#name) });
                }
                #(#interfaces)*
                #downcast_impl_super
                #_None
            }
        }
    }
}

impl ClassAttrs {
    pub(super) fn has_extends_or_withs(&self) -> bool {
        self.extends.is_some() || self.withs.is_some()
    }
    pub(super) fn extends_and_withs(
        &self,
    ) -> Option<(&dyn quote::ToTokens, &Punctuated<syn::Path, syn::Token![,]>)> {
        struct SyncWrapper(Punctuated<syn::Path, syn::Token![,]>);
        unsafe impl Sync for SyncWrapper {}
        static EMPTY: SyncWrapper = SyncWrapper(Punctuated::new());
        Some(match (&self.extends, &self.withs) {
            (None, None) => return None,
            (Some(extends), Some(withs)) => (&extends.classes, &withs.classes),
            (Some(extends), None) => (&extends.classes, &EMPTY.0),
            (None, Some(withs)) => {
                match self.implements.as_ref().and_then(Implements::get_downcast) {
                    None => (&_Dummy, &withs.classes),
                    Some(downcast) => (downcast, &withs.classes),
                }
            }
        })
    }
    pub(super) fn expand_super_class_type(&self) -> Option<syn::Type> {
        let (superclass, withs) = self.extends_and_withs()?;
        Some(withs.iter().fold(
            syn::parse_quote!(#_CClass<#superclass, __Self>),
            |super_type, mixin| syn::parse_quote!(#_MClass<#super_type, #mixin, __Self>),
        ))
    }

    pub(super) fn expand_super_data_type(
        &self,
    ) -> Option<(syn::Type, Punctuated<syn::Expr, syn::Token![,]>)> {
        let (superclass, withs) = self.extends_and_withs()?;

        Some((
            syn::parse_quote!(#_CData<#superclass>),
            withs
                .iter()
                .scan(
                    quote!(#_CClass<#superclass, __Self>),
                    |super_type, mixin| -> Option<syn::Expr> {
                        *super_type = syn::parse_quote!(#_MClass<#super_type, #mixin, __Self>);
                        Some(syn::parse_quote!(
                            #_DataOffsetEntry::new(
                                <#_CData<#mixin> as #_MixinData>::MIXIN_TYPE,
                                <#super_type as #_MixinDataOffset>::OFFSET,
                            )
                        ))
                    },
                )
                .collect(),
        ))
    }

    fn expand_class_impl_super_field(&self) -> Option<TokenStream> {
        let supertype = self.expand_super_class_type()?;
        Some(quote!(__super: #supertype,))
    }
    fn expand_class_impl_has_super_and_deref(&self, cx: &ExpandCtxt<'_>) -> Vec<syn::ItemImpl> {
        // Build super type with MClass wrappers if mixins are present
        let Some(supertype) = self.expand_super_class_type() else {
            return Vec::new();
        };

        let class = cx.class_impl_name();
        let class_impl_bounds = self.expand_concrete_with_trait_bounds(cx);

        let mut ret = vec![syn::parse_quote! {
            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_HasSuper for #class<__Self> {
                type Super = #supertype;
            }
        }];

        // Add HasSuperVtable and HasSuperVtableImpl
        // HasSuperVtable only defines type alias, needs minimal bounds
        ret.push(syn::parse_quote! {
            #[automatically_derived]
            impl<__Self: #_ClassConcrete> #_HasSuperVtable for #class<__Self> {
                type SuperVtable = <#supertype as #_HasVtable>::Vtable;
            }
        });

        ret.push(syn::parse_quote! {
            #[automatically_derived]
            impl<__Self: #class_impl_bounds> #_HasSuperVtableImpl for #class<__Self> {
                const SUPER_VTABLE: Self::SuperVtable = <#supertype as #_HasVtableImpl>::VTABLE;
            }
        });

        // special case: `Object` implements `Downcast`
        // but does not deref to `Downcast`
        if self.has_extends_or_withs() {
            ret.push(syn::parse_quote! {
                #[automatically_derived]
                impl<__Self: #_ClassConcrete> #_Deref for #class<__Self> {
                    type Target = #supertype;
                    fn deref(&self) -> &Self::Target {
                        &self.__super
                    }
                }
            });
        }
        ret
    }

    pub(super) fn expand_data_class_data_type(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let class_impl = cx.class_impl_name();
        if let Some((super_data_type, mixins)) = self.expand_super_data_type() {
            quote! {
                &#_ClassDataTypeInfo::new(#_TypeName).with_super(
                    <#super_data_type as #_ClassData>::CLASS_TYPE,
                    #_offset_of!(#class_impl<#_Dummy>, data)
                ).with_mixins(&[#mixins])
            }
        } else {
            quote! { &#_ClassDataTypeInfo::new(#_TypeName) }
        }
    }

    pub(super) fn expand_concrete_has_super(&self, cx: &ExpandCtxt<'_>) -> Option<syn::ItemImpl> {
        let concrete = cx.concrete_name();
        let supertype = match &self.withs {
            // When using mixins, HasSuper points to the last mixin
            Some(withs) => withs.classes.last().expect("`with` must be non-empty"),
            // No mixins, HasSuper points to superclass
            None => &self.extends.as_ref()?.classes,
        };
        Some(syn::parse_quote! {
            #[automatically_derived]
            impl #_HasSuper for #concrete {
                type Super = #supertype;
            }
        })
    }
}
