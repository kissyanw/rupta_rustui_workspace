use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::check::CheckCtxt;
use crate::errors::CheckError;
use crate::syntax::class::*;

impl Class {
    pub fn check(&self) -> TokenStream {
        let mut cx = CheckCtxt::new(self);

        self.class_attrs.check_attrs(&mut cx);

        let errors = cx.into_errors().map(syn::Error::into_compile_error);
        quote!(#(#errors)*)
    }
}

impl ClassAttrs {
    fn check_attrs(&self, cx: &mut CheckCtxt<'_>) {
        match cx.class.kind {
            ClassKind::Class(_) => self.check_class_attrs(cx),
            ClassKind::Interface(_) => self.check_interface_attrs(cx),
            ClassKind::Mixin(_) => self.check_mixin_attrs(cx),
        }
    }
    fn check_class_attrs(&self, cx: &mut CheckCtxt<'_>) {
        if let Some(ons) = &self.ons {
            cx.errors.push(syn::Error::new(
                ons.kw.span,
                CheckError::UnexpectedClassAttrOn,
            ));
        }
        if let Some(implements) = &self.implements
            && let Some(downcast) = implements.get_downcast()
            && self.extends.is_some()
        {
            cx.errors.push(syn::Error::new(
                downcast.span(),
                CheckError::ExtendsAndImplsDowncast,
            ));
        }
        self.check_implements(cx);
    }
    fn check_interface_attrs(&self, cx: &mut CheckCtxt<'_>) {
        if let Some(extends) = &self.extends {
            cx.errors.push(syn::Error::new(
                extends.kw.span,
                CheckError::UnexpectedInterfaceAttrExtends,
            ));
        }
        if let Some(kw_abstract) = &self.kw_abstract {
            cx.errors.push(syn::Error::new(
                kw_abstract.span,
                CheckError::UnexpectedInterfaceAttrAbstract,
            ));
        }
        if let Some(kw_final) = &self.kw_final {
            cx.errors.push(syn::Error::new(
                kw_final.span,
                CheckError::UnexpectedInterfaceAttrFinal,
            ));
        }
        self.check_implements(cx);
    }
    fn check_mixin_attrs(&self, cx: &mut CheckCtxt<'_>) {
        if let Some(extends) = &self.extends {
            cx.errors.push(syn::Error::new(
                extends.kw.span,
                CheckError::UnexpectedMixinAttrExtends,
            ));
        }
        if let Some(with) = &self.withs {
            cx.errors.push(syn::Error::new(
                with.kw.span,
                CheckError::UnexpectedMixinAttrWith,
            ));
        }
        if let Some(kw_abstract) = &self.kw_abstract {
            cx.errors.push(syn::Error::new(
                kw_abstract.span,
                CheckError::UnexpectedMixinAttrAbstract,
            ));
        }
        if let Some(kw_final) = &self.kw_final {
            cx.errors.push(syn::Error::new(
                kw_final.span,
                CheckError::UnexpectedMixinAttrFinal,
            ));
        }
        if let Some(implements) = &self.implements
            && let Some(downcast) = implements.get_downcast()
            && self.extends.is_some()
        {
            cx.errors.push(syn::Error::new(
                downcast.span(),
                CheckError::ExtendsAndImplsDowncast,
            ));
        }
        self.check_implements(cx);
    }
    fn check_implements(&self, cx: &mut CheckCtxt<'_>) {
        if let Some(implements) = &self.implements {
            if let Some(idx) = implements
                .classes
                .iter()
                .position(|path| path.is_ident("Downcast"))
                && idx > 0
            {
                cx.errors.push(syn::Error::new(
                    implements.classes[idx].span(),
                    CheckError::ImplDowncastMustBeFirst,
                ));
            }
        }
    }
}
