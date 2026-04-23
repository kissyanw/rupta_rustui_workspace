use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;

use super::constants::*;
use crate::expand::ExpandCtxt;
use crate::syntax::class::*;

impl Class {
    pub(super) fn expand_mixin(&self) -> TokenStream {
        let cx = ExpandCtxt::new(self);

        let mod_name = cx.mod_name();
        let name = &self.name;
        let trait_name = cx.trait_name();
        let vis = &self.vis;

        let class_impl = self.items.expand_mixin_class_impl(&cx);
        let data = self.items.expand_mixin_data(&cx);
        let trait_ = self.items.expand_mixin_trait(&cx);
        let vtable = self.items.methods.expand_mixin_vtable(&cx);
        let fields = self.items.expand_field_accessors(&cx);
        let helpers: Vec<_> = self.expand_helper_impls(&cx).collect();
        let static_items = self.items.expand_static_items();
        let eq_hash_impls = self.expand_eq_hash_impls(&cx);
        let format_impls = self.expand_format_impls(&cx);

        quote! {
            #[allow(unused_imports)]
            #vis use #mod_name::{#name, #trait_name};
            #[allow(non_snake_case)]
            #[allow(private_interfaces)]
            mod #mod_name {
                #[allow(unused_imports)]
                use super::*;
                #[allow(unused_imports)]
                use #_crate::prelude::*;
                const __TYPE_NAME: &str = concat!(module_path!(), "::", stringify!(#name));

                #class_impl

                #data

                #trait_

                #vtable

                #( #helpers )*

                #fields

                impl #name {
                    #(#static_items)*
                }

                #eq_hash_impls
                #format_impls
            }
        }
    }
}

impl ClassItems {
    fn expand_mixin_class_impl(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let class = cx.class;
        let name = &class.name;
        let class_impl = cx.class_impl_name();
        let data = cx.data_name();
        let vtable_instance = cx.instance_vtable_name();

        // Get trait bounds from on(...) clause
        let trait_bounds = class.class_attrs.expand_trait_bounds(cx);

        let vtable_init = self.methods.expand_mixin_vtable_init(cx);
        let vtable_overrides = self.methods.expand_vtable_override(cx);
        let items = self.expand_item_defs(cx);

        // Generate RcRef impl
        let rcref_impl = quote! {
            #[automatically_derived]
            unsafe impl<__Super: #_Class, __Self: #_ClassConcrete + #_RcRef> #_RcRef for #class_impl<__Super, __Self> {}
        };

        let downcast_impl = self.expand_downcast_impl(cx);

        // Generate __mixin constructor
        let mixin_ctor = self.expand_mixin_ctor(cx);

        quote! {
            #[repr(C)]
            pub struct #class_impl<__Super: #_HasData, __Self: #_ClassConcrete> {
                __super: __Super,
                __self: #_PhantomData<__Self>,
                data: #data,
            }

            #[automatically_derived]
            impl<__Super: #_HasData, __Self: #_ClassConcrete> #_MixinDataOffset for #class_impl<__Super, __Self> {
                const OFFSET: usize = #_offset_of!(#class_impl<__Super, __Self>, data);
            }

            #[automatically_derived]
            impl<__Super: #_HasData, __Self: #_ClassConcrete> #_HasData for #class_impl<__Super, __Self> {
                type Data = #data;
            }

            #[automatically_derived]
            impl<__Super: #_HasData, __Self: #_ClassConcrete + #trait_bounds> #_MixinHasData for #class_impl<__Super, __Self> {}

            #[automatically_derived]
            impl<__Super: #_HasVtable + #_HasData, __Self: #_ClassConcrete> #_HasVtable for #class_impl<__Super, __Self> {
                type Vtable = #vtable_instance<<__Super as #_HasVtable>::Vtable, __Self>;
            }

            #[automatically_derived]
            impl<__Super: #_HasVtable + #_HasData, __Self: #_ClassConcrete> #_HasSuperVtable for #class_impl<__Super, __Self> {
                type SuperVtable = <__Super as #_HasVtable>::Vtable;
            }

            #[automatically_derived]
            impl<__Super: #_HasVtableImpl + #_HasData, __Self: #_ClassConcrete + #trait_bounds> #_HasVtableImpl for #class_impl<__Super, __Self> {
                const VTABLE: Self::Vtable = #class_impl::<__Super, __Self>::__VTABLE;
            }

            #[automatically_derived]
            impl<__Super: #_HasVtableImpl + #_HasData, __Self: #_ClassConcrete + #trait_bounds> #_HasSuperVtableImpl for #class_impl<__Super, __Self> {
                const SUPER_VTABLE: Self::SuperVtable = <__Super as #_HasVtableImpl>::VTABLE;
            }

            #[automatically_derived]
            impl<__Super: #_HasData, __Self: #_ClassConcrete> #_Deref for #class_impl<__Super, __Self> {
                type Target = __Super;
                fn deref(&self) -> &Self::Target {
                    &self.__super
                }
            }

            #[automatically_derived]
            impl<__Super: #_HasData, __Self: #_ClassConcrete> #_HasSuper for #class_impl<__Super, __Self> {
                type Super = __Super;
            }

            #[automatically_derived]
            impl<__Super: #_HasData, __Self: #_ClassConcrete> #_MixinHasDyn for #class_impl<__Super, __Self> {
                type Dyn = #name;
            }

            #rcref_impl

            #[automatically_derived]
            impl<__Super: #_HasVtableImpl + #_HasData, __Self: #_ClassConcrete + #trait_bounds> #class_impl<__Super, __Self> {
                const __VTABLE: #vtable_instance<<__Super as #_HasVtable>::Vtable, __Self> = {
                    let mut vtable = #vtable_init;
                    Self::__override(&mut vtable);
                    vtable
                };

                #[allow(unused_variables)]
                const fn __override(vtable: &mut #vtable_instance<<__Super as #_HasVtable>::Vtable, __Self>) {
                    #vtable_overrides
                    // Override __downcast to include mixin check
                    #_write_vtable!((vtable as Downcast).__downcast = Self::__downcast);
                }

                #mixin_ctor

                #downcast_impl

                #(#items)*
            }
        }
    }

    fn expand_mixin_ctor(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let data = cx.data_name();
        let fields = self.fields.iter().map(|field| syn::FieldValue {
            attrs: Vec::new(),
            member: syn::Member::Named(field.field.clone()),
            colon_token: Some(field.tk_colon.clone()),
            expr: field.expand_init(None),
        });

        quote! {
            pub fn __mixin(mut __self: #_CRcUninit<__Self, Self>) -> #_CRcUninit<__Self, __Super> {
                let __data = #data {
                    #(#fields,)*
                };
                unsafe {
                    #_core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, #_PhantomData);
                    #_core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
                    __self.assume_init_except()
                }
            }
        }
    }

    fn expand_mixin_data(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let data = cx.data_name();
        let fields: Punctuated<_, syn::Token![,]> =
            self.fields.iter().map(|field| field.expand_def()).collect();

        quote! {
            pub struct #data {
                #fields
            }

            #[automatically_derived]
            impl #_ClassOrMixinData for #data {
                const TYPE: #_DataTypeInfo<'static> = #_DataTypeInfo::Mixin(&#_MixinDataTypeInfo::new(#_TypeName));
            }

            #[automatically_derived]
            impl #_MixinData for #data {}
        }
    }

    fn expand_mixin_trait(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let class = cx.class;
        let name = &class.name;
        let trait_ = cx.trait_name();
        let trait_bounds = class.class_attrs.expand_trait_bounds(cx);
        let data = cx.data_name();
        let class_impl = cx.class_impl_name();
        let vtable = cx.vtable_name();

        // Generate method signatures for the trait
        let method_defs = self
            .methods
            .owns
            .iter()
            .map(|method| method.expand_def_in_trait());

        // Trait definition
        let trait_def = quote! {
            #[allow(dead_code)]
            pub trait #trait_: #trait_bounds {
                fn __offset(&self) -> usize;
                #(#method_defs)*
            }
        };

        // Type alias for the trait object
        let type_alias = quote! {
            pub type #name = dyn #trait_;
            type __Class = #name;
        };

        // Method impls
        let method_impls = self
            .methods
            .owns
            .iter()
            .map(|method| method.expand_impl_in_trait(cx));

        // Impl of trait
        let not_subclass_msg = format!("not a subclass of {}", name);
        let trait_impl = quote! {
            #[automatically_derived]
            impl<__Self: #_ClassConcrete + #trait_bounds> #trait_ for __Self {
                fn __offset(&self) -> usize {
                    const {
                        <<__Self as #_HasData>::Data as #_ClassData>::CLASS_TYPE
                            .offset_of_mixin(<#data as #_MixinData>::MIXIN_TYPE)
                            .expect(#not_subclass_msg)
                    }
                }
                #(#method_impls)*
            }
        };

        // ClassConcreteOrDyn impl
        let concrete_or_dyn_impl = quote! {
            #[automatically_derived]
            impl #_ClassConcreteOrDyn for #name {
                const TYPE: &#_ClassConcreteOrDynTypeInfo = &#_ClassConcreteOrDynTypeInfo::new_mixin(#_TypeName);
            }
        };

        // MixinHasData, HasData, DynHasVtable impls
        let mixin_has_impls = quote! {
            #[automatically_derived]
            impl #_MixinHasData for #name {}

            #[automatically_derived]
            impl #_HasData for #name {
                type Data = #data;
            }

            #[automatically_derived]
            impl #_DynHasVtable for #name {
                type Vtable<__Self: #_ClassConcrete> = #vtable<__Self>;
            }
        };

        // MixinDyn impl
        let mixin_dyn_impl = quote! {
            #[automatically_derived]
            impl #_MixinDyn for #name {
                type Class<__Super: #_HasData, __Self: #_ClassConcrete> = #class_impl<__Super, __Self>;
            }
        };

        quote! {
            #trait_def

            #type_alias

            #trait_impl

            #concrete_or_dyn_impl

            #mixin_has_impls

            #mixin_dyn_impl
        }
    }
}

/// Typed or untyped mixin trait
pub(super) enum MixinTraitKind {
    /// Untyped where `Super` is a type parameter
    Untyped,
    /// Typed where `Super` is given as an asoociated type
    Typed,
}
