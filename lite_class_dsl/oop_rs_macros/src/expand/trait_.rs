use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;

use super::constants::*;
use crate::expand::ExpandCtxt;
use crate::expand::mixin::MixinTraitKind;
use crate::syntax::class::*;

impl ClassItems {
    pub(super) fn expand_trait(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let class = cx.class;
        let name = &class.name;
        let trait_ = cx.trait_name();
        let trait_bounds = class.class_attrs.expand_trait_bounds(cx);
        let data = cx.data_name();

        // Generate method signatures for the trait
        let method_defs = self
            .methods
            .owns
            .iter()
            .map(|method| method.expand_def_in_trait());

        // Generate type alias for the trait object
        let type_alias = quote! {
            pub type #name = dyn #trait_;
            type __Class = #name;
        };

        // Trait definition with bounds
        let trait_def = quote! {
            #[allow(dead_code)]
            pub trait #trait_: #trait_bounds {
                #_assert_subclass!(self, #name);
                #(#method_defs)*
            }
        };

        let method_impls = self
            .methods
            .owns
            .iter()
            .map(|method| method.expand_impl_in_trait(cx));

        let trait_bounds = class.class_attrs.expand_concrete_with_trait_bounds(cx);
        let trait_impl = quote! {
            #[automatically_derived]
            impl<__Self: #_ClassConcrete + #trait_bounds> #trait_ for __Self {
                #(#method_impls)*
            }
        };

        // HasData, HasVtable, ClassConcreteOrDyn, ClassDyn implementations for dyn trait
        let vtable = cx.vtable_name();
        let class_impl = cx.class_impl_name();

        let with_super = cx
            .class
            .class_attrs
            .extends
            .as_ref()
            .map(|_| quote!(.with_super::<Self>()));

        // HasSuper and Deref implementations if there's a super class
        let has_super_and_deref = class.class_attrs.expand_class_dyn_has_super_and_deref(cx);

        let dyn_impls = quote! {
            #[automatically_derived]
            impl #_ClassHasDyn for #name {
                type Dyn = #name;
            }

            #[automatically_derived]
            impl #_HasData for #name {
                type Data = #data;
            }

            #[automatically_derived]
            impl #_DynHasVtable for #name {
                type Vtable<__Self: #_ClassConcrete> = #vtable<__Self>;
            }

            #[automatically_derived]
            impl #_ClassConcreteOrDyn for #name {
                const TYPE: &#_ClassConcreteOrDynTypeInfo = &(#_ClassConcreteOrDynTypeInfo::new_dyn(#_TypeName)
                    #with_super);
            }

            #[automatically_derived]
            impl #_ClassDyn for #name {
                type Class<__Self: #_ClassConcrete> = #class_impl<__Self>;
            }

            #(#has_super_and_deref)*
        };

        quote! {
            #trait_def

            #type_alias

            #trait_impl

            #dyn_impls
        }
    }
}

impl ClassAttrs {
    fn append_trait_bounds(
        &self,
        _cx: &ExpandCtxt<'_>,
        bounds: &mut Punctuated<syn::Path, syn::Token![+]>,
        kind: MixinTraitKind,
    ) {
        if let Some(Extends { classes, .. }) = &self.extends {
            bounds.push(ClassTo(super::trait_name).transform()(classes));
        }
        if let Some(Withs { classes, .. }) = &self.withs {
            // Always add mixin traits to bounds
            // This is necessary because when a class uses with(M), it needs to have
            // all the trait bounds that M has, including the interfaces that M implements
            bounds.extend(classes.iter().map(ClassTo(super::trait_name).transform()))
        }
        if let Some(Ons { classes, .. }) = &self.ons {
            for class_ref in classes.iter() {
                if class_ref.is_mixin {
                    // For mixins in 'on' clause, don't add to untyped bounds to avoid __offset conflict
                    // Only add to typed bounds
                    // Note: We don't specify the Super type here, it will be inferred from the application context
                    match kind {
                        MixinTraitKind::Untyped => {
                            // Skip mixins in untyped mode
                        }
                        MixinTraitKind::Typed => {
                            // Mixins don't have typed traits, so we don't add them to bounds
                        }
                    }
                } else {
                    // For classes in 'on' clause, add to both untyped and typed bounds
                    bounds.push(ClassTo(super::trait_name).transform()(&class_ref.path));
                }
            }
        }
        if let Some(Implements { classes, .. }) = &self.implements {
            bounds.extend(classes.iter().map(ClassTo(super::trait_name).transform()));
        }
        bounds.push(syn::parse_quote!(#_RcRef));
    }
    pub(super) fn expand_trait_bounds(
        &self,
        cx: &ExpandCtxt<'_>,
    ) -> Punctuated<syn::Path, syn::Token![+]> {
        let mut bounds = Punctuated::new();
        self.append_trait_bounds(cx, &mut bounds, MixinTraitKind::Untyped);
        bounds
    }
    pub(super) fn expand_concrete_with_trait_bounds(
        &self,
        cx: &ExpandCtxt<'_>,
    ) -> Punctuated<syn::Path, syn::Token![+]> {
        let mut bounds = Punctuated::new();
        bounds.push(syn::parse_quote!(#_ClassConcrete));

        self.append_trait_bounds(cx, &mut bounds, MixinTraitKind::Typed);
        bounds
    }
    fn expand_class_dyn_has_super_and_deref(&self, cx: &ExpandCtxt<'_>) -> Vec<syn::ItemImpl> {
        let Some(superclass) = self
            .extends
            .as_ref()
            .map(|Extends { classes, .. }| classes)
            .or_else(|| self.implements.as_ref().and_then(Implements::get_downcast))
            .or_else(|| self.implements.as_ref().and_then(|i| i.classes.first()))
        else {
            return Vec::new();
        };

        let name = &cx.class.name;
        vec![
            syn::parse_quote! {
                #[automatically_derived]
                impl #_HasSuper for #name {
                    type Super = #superclass;
                }
            },
            syn::parse_quote! {
                #[automatically_derived]
                impl #_Deref for #name {
                    type Target = #superclass;
                    fn deref(&self) -> &Self::Target {
                        self
                    }
                }
            },
        ]
    }
}

pub(super) struct ClassTo<F>(F);

impl<F: Fn(&syn::Ident) -> syn::Ident> VisitMut for ClassTo<F> {
    fn visit_path_mut(&mut self, i: &mut syn::Path) {
        if let Some(seg) = i.segments.last_mut() {
            seg.ident = (self.0)(&seg.ident);
        }
    }
}

impl<F: Fn(&syn::Ident) -> syn::Ident> ClassTo<F> {
    pub(super) fn transform(mut self) -> impl FnMut(&syn::Path) -> syn::Path {
        move |class| {
            let mut path = class.clone();
            self.visit_path_mut(&mut path);
            path
        }
    }
}
