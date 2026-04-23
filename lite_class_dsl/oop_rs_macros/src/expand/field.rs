use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::TokenStreamExt;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

use super::ExpandCtxt;
use super::constants::*;
use crate::expand::VisibilityExt;
use crate::expand::method::SelfExprVisitor;
use crate::syntax::class::*;
use crate::syntax::field::*;
use crate::syntax::method::*;
use Mutability::*;

#[derive(Debug, Clone, Copy)]
enum RcTy {
    Rc,
    OptionRc,
    Weak,
    OptionWeak,
    RcLike,
}

#[derive(Debug, Clone, Copy)]
enum Ty {
    Copy,
    Rc(RcTy),
}

#[derive(Debug, Clone, Copy)]
enum Mutability {
    Not,
    Mut,
}

#[derive(Debug, Clone, Copy)]
enum Ref {
    No(Ty),
    Yes,
}

#[derive(Debug, Clone, Copy)]
enum Late {
    No,
    Yes,
}

#[derive(Debug, Clone, Copy)]
struct FieldKind(Late, Ref, Mutability);

impl ClassItems {
    pub(super) fn expand_field_accessors(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let data = cx.data_name();
        let name = &cx.class.name;
        let accessor = cx.accessor_name();
        let trait_ = cx.trait_name();

        let offset = match cx.class.kind {
            ClassKind::Interface(_) => return quote!(),
            ClassKind::Class(_) => quote!(const { <#data as #_ClassData>::CLASS_TYPE.offset() }),
            ClassKind::Mixin(_) => quote!(#trait_::__offset(self)),
        };

        let accessor_def = self.expand_accessor_def(cx);

        // Generate getter and setter methods for dyn trait (delegates to __data())
        let accessors = AccessMode::MODES
            .iter()
            .map(|&mode| self.expand_field_accessors_for_mode(cx, mode));

        quote! {
            impl #name {
                fn __data(&self) -> &#data {
                    let offset = #offset;
                    unsafe {
                        &*#_core::ptr::from_ref(self)
                            .byte_add(offset)
                            .cast()
                    }
                }
            }

            #accessor_def

            #[automatically_derived]
            unsafe impl #_Access for #name {
                type Accessor<__M: #_AccessMode> = #accessor<__M>;
            }

            #(#accessors)*
        }
    }

    fn expand_accessor_def(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let accessor = cx.accessor_name();
        let class = &cx.class.name;
        if cx.class.kind.is_mixin() {
            quote! {
                #[repr(transparent)]
                pub struct #accessor<
                    __M: #_AccessMode,
                    __Upcast: #_AccessorUpcast<__T> = #_Dummy,
                    __T: ?Sized = #class,
                >(#_PhantomData<__M>, #_PhantomData<__Upcast>, __T);

                unsafe impl #_MixinAccessor for #class {
                    type Accessor<__M: #_AccessMode, __Upcast: #_AccessorUpcast<__T>, __T: ?Sized> =
                        #accessor<__M, __Upcast, __T>;
                }

                impl<__M: #_AccessMode, __Upcast: #_AccessorUpcast<__T>, __T: ?Sized> #_Deref
                    for #accessor<__M, __Upcast, __T>
                {
                    type Target = <__Upcast as #_AccessorUpcast<__T>>::SuperAccessor<__M>;

                    fn deref(&self) -> &Self::Target {
                        __Upcast::upcast(&self.2)
                    }
                }
            }
        } else {
            let deref = self.expand_accessor_deref(cx);
            quote! {
                #[repr(transparent)]
                pub struct #accessor<__M: #_AccessMode>(#_PhantomData<__M>, #class);

                #deref
            }
        }
    }

    fn expand_accessor_deref(&self, cx: &ExpandCtxt<'_>) -> Option<TokenStream> {
        let accessor = cx.accessor_name();
        let class = &cx.class.name;

        Some(
            match (&cx.class.class_attrs.extends, &cx.class.class_attrs.withs) {
                (None, None) => return None,
                (Some(Extends { classes, .. }), None) => quote! {
                    #[automatically_derived]
                    impl<__M: #_AccessMode> #_Deref for #accessor<__M> {
                        type Target = <#classes as #_Access>::Accessor<__M>;
                        fn deref(&self) -> &Self::Target {
                            <#classes as #_Access>::__access::<__M>(&self.1)
                        }
                    }
                },
                (_, Some(Withs { classes, .. })) => {
                    let (super_accessor, upcast) = match &cx.class.class_attrs.extends {
                        Some(Extends { classes, .. }) => (
                            quote!(<#classes as #_Access>::Accessor<__M>),
                            quote!((__self as &#classes).__access::<__M>()),
                        ),
                        None => (quote!(#_Dummy), quote!(&#_Dummy)),
                    };

                    let impls = classes.iter().enumerate().scan((super_accessor, None), |(super_accessor, last_mixin), (i, mixin)| {
                        let upcast = match i.checked_sub(1) {
                            Some(i_minus_1) => quote! {
                                <#last_mixin as #_MixinAccessor>::__access::<__M, __Accessors<#i_minus_1>, #class>(__self)
                            },
                            None => quote!(#upcast),
                        };
                        let impl_ = quote! {
                            #[automatically_derived]
                            impl #_AccessorUpcast<#class> for __Accessors<#i> {
                                type SuperAccessor<__M: #_AccessMode> = #super_accessor;
                                fn upcast<__M: #_AccessMode>(__self: &#class) -> &Self::SuperAccessor<__M> {
                                    #upcast
                                }
                            }

                            #[automatically_derived]
                            impl #_Borrow<#mixin> for #class {
                                fn borrow(&self) -> &#mixin {
                                    self
                                }
                            }
                        };
                        *super_accessor = quote!(<#mixin as #_MixinAccessor>::Accessor<__M, __Accessors<#i>, #class>);
                        *last_mixin = Some(mixin);
                        Some(impl_)
                    });

                    let last_mixin = classes.last().expect("at least one mixin");
                    let num_mixins = classes.len() - 1;
                    quote! {
                        pub struct __Accessors<const N: usize>;

                        #(#impls)*

                        #[automatically_derived]
                        impl<__M: #_AccessMode> #_Deref for #accessor<__M> {
                            type Target = <#last_mixin as #_MixinAccessor>::Accessor<__M, __Accessors<#num_mixins>, #class>;
                            fn deref(&self) -> &Self::Target {
                                <#last_mixin as #_MixinAccessor>::__access::<__M, __Accessors<#num_mixins>, #class>(&self.1)
                            }
                        }
                    }
                }
            },
        )
    }

    fn expand_field_accessors_for_mode(
        &self,
        cx: &ExpandCtxt<'_>,
        mode: AccessMode,
    ) -> syn::ItemImpl {
        let accessor = cx.accessor_name();
        let class = &cx.class.name;
        let fields = self
            .fields
            .iter()
            .filter_map(|field| field.expand_accessor(cx, mode));
        if cx.class.kind.is_mixin() {
            syn::parse_quote! {
                impl<__Upcast: #_AccessorUpcast<__T>, __T: #_Borrow<#class> + ?Sized> #accessor<#mode, __Upcast, __T> {
                    #(#fields)*
                }
            }
        } else {
            syn::parse_quote! {
                impl #accessor<#mode> {
                    #(#fields)*
                }
            }
        }
    }
}

impl CtorSelfExpr {
    pub(super) fn expand_struct_expr_fields(
        &self,
        cx: &ExpandCtxt<'_>,
    ) -> Punctuated<syn::FieldValue, syn::Token![,]> {
        let mut output = Punctuated::new();
        let mut seen_fields = HashSet::new();

        // fill in existing fields
        for field in self.fields.pairs().map(|pair| pair.into_value()) {
            let field_ident = &field.member;
            let expr = &field.expr;
            seen_fields.insert(field_ident);
            let expr = cx
                .get_field(field_ident)
                .map_or_else(|| expr.clone(), |field| field.expand_init(Some(expr)));
            let colon_token = if let syn::Expr::Path(path) = &expr
                && path.qself.is_none()
                && let Some(ident) = path.path.get_ident()
                && ident == field_ident
            {
                None
            } else {
                Some(<syn::Token![:]>::default())
            };

            output.push(syn::FieldValue {
                attrs: field.attrs.clone(),
                member: syn::Member::Named(field_ident.clone()),
                colon_token,
                expr,
            });
        }

        // fill in missing fields
        output.extend(
            cx.class
                .items
                .fields
                .iter()
                .filter(|field| !seen_fields.contains(&field.field))
                .map(|field| {
                    let expr = field.expand_init(None);
                    syn::FieldValue {
                        attrs: Vec::new(),
                        member: syn::Member::Named(field.field.clone()),
                        colon_token: Some(field.tk_colon.clone()),
                        expr,
                    }
                }),
        );
        output
    }
}

impl ClassItems {
    pub(super) fn expand_data(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let data = cx.data_name();
        let fields: Punctuated<_, syn::Token![,]> =
            self.fields.iter().map(|field| field.expand_def()).collect();

        let class_data_type_info = cx.class.class_attrs.expand_data_class_data_type(cx);

        // Generate ClassOrMixinData and ClassData impls
        let class_or_mixin_data_impl = quote! {
            #[automatically_derived]
            impl #_ClassOrMixinData for #data {
                const TYPE: #_DataTypeInfo<'static> = #_DataTypeInfo::Class(#class_data_type_info);
            }
        };

        let class_data_impl = quote! {
            #[automatically_derived]
            impl #_ClassData for #data {}
        };

        quote! {
            pub struct #data {
                #fields
            }


            #class_or_mixin_data_impl

            #class_data_impl
        }
    }
}

impl Field {
    pub(super) fn expand_def(&self) -> syn::Field {
        let Self {
            attrs,
            field,
            tk_colon,
            ..
        } = self;
        let vis = quote!(pub(super)); // TODO
        let ty = self.kind().ty(&self.ty);
        syn::parse_quote! {
            #(#attrs)*
            #vis #field #tk_colon #ty
        }
    }

    fn kind(&self) -> FieldKind {
        let late = match self.kw_late {
            Some(_) => Late::Yes,
            None => Late::No,
        };
        let is_ref = match self.tk_ref {
            Some(_) => Ref::Yes,
            None => Ref::No(self.ty.ty_kind()),
        };
        let mutbl = match self.tk_mut {
            Some(_) => Mut,
            None => Not,
        };
        FieldKind(late, is_ref, mutbl)
    }

    pub(super) fn expand_init(&self, expr: Option<&syn::Expr>) -> syn::Expr {
        let FieldKind(late, ref_, mut_) = self.kind();
        let field_span = self.field.span();
        match (late, expr, self.init.as_ref()) {
            (Late::No, None, None) => parse_quote_spanned!(field_span=> #_Missing::missing()),
            (Late::No, Some(expr), _) | (Late::No, None, Some(FieldInit { expr, .. })) => {
                let expr = if let Ref::No(Ty::Rc(RcTy::RcLike)) = ref_ {
                    parse_quote_spanned!(expr.span()=> #_RcLike::new(#expr))
                } else {
                    expr.clone()
                };
                match (ref_, mut_) {
                    (_, Not) => expr.clone(),
                    (Ref::No(_), Mut) => parse_quote_spanned!(expr.span()=> #_Cell::new(#expr)),
                    (Ref::Yes, Mut) => parse_quote_spanned!(expr.span()=> #_RefCell::new(#expr)),
                }
            }
            (Late::Yes, ..) => match (ref_, mut_) {
                (_, Not) => parse_quote_spanned!(field_span=> #_OnceCell::new()),
                (Ref::No(_), Mut) => parse_quote_spanned!(field_span=> #_Cell::new(#_None)),
                (Ref::Yes, Mut) => parse_quote_spanned!(field_span=> #_RefCell::new(#_None)),
            },
        }
    }

    fn expand_self_data_field(&self, cx: &ExpandCtxt<'_>) -> TokenStream {
        let class = &cx.class.name;
        let field = &self.field;
        if cx.class.kind.is_mixin() {
            quote!(#_Borrow::<#class>::borrow(&self.2).__data().#field)
        } else {
            quote!(self.1.__data().#field)
        }
    }

    fn expand_self(&self, cx: &ExpandCtxt<'_>) -> syn::Expr {
        let class = &cx.class.name;
        if cx.class.kind.is_mixin() {
            syn::parse_quote!(#_Borrow::<#class>::borrow(&self.2))
        } else {
            syn::parse_quote!(self.1)
        }
    }

    fn expand_accessor(&self, cx: &ExpandCtxt<'_>, mode: AccessMode) -> Option<syn::ImplItemFn> {
        use AccessMode::*;
        let kind = self.kind();

        let field = &self.field;
        let vis = self.vis.expand_for_super();
        let ty = &self.ty;
        let wrapped_ty = kind.ty(ty);
        let field_trait = kind.field_trait();

        let attrs = quote!(#_inline #_track_caller);
        let self_field = self.expand_self_data_field(cx);

        Some(match mode {
            Raw => {
                syn::parse_quote! { #attrs #vis fn #field(&self) -> &#wrapped_ty { &#self_field } }
            }
            Get => {
                let ty_get = kind.ty_get(ty);
                let body: syn::Expr = match self.expand_init_closure_for_late(cx) {
                    Some(closure) => {
                        syn::parse_quote!(#field_trait::get_or_init(&#self_field, #closure))
                    }
                    None => syn::parse_quote!(#field_trait::get(&#self_field)),
                };
                syn::parse_quote! { #attrs #vis fn #field(&self) -> #ty_get { #body } }
            }
            GetMut => {
                let ty_get_mut = kind.ty_get_mut(ty)?;
                let body: syn::Expr = match self.expand_init_closure_for_late(cx) {
                    Some(closure) => {
                        syn::parse_quote!(#field_trait::get_mut_or_init(&#self_field, #closure))
                    }
                    None => syn::parse_quote!(#field_trait::get_mut(&#self_field)),
                };
                syn::parse_quote! { #attrs #vis fn #field(&self) -> #ty_get_mut { #body } }
            }
            Set => {
                let ty_set = kind.ty_set(ty)?;
                syn::parse_quote! {
                    #attrs #vis fn #field(&self, #field: #ty_set) {
                        #field_trait::set(&#self_field, #field)
                    }
                }
            }
            Replace => {
                let ty_get = kind.ty_get_for_replace(ty)?;
                let ty_set = kind.ty_set(ty)?;
                syn::parse_quote! {
                    #attrs #vis fn #field(&self, #field: #ty_set) -> #ty_get {
                        #field_trait::replace(&#self_field, #field)
                    }
                }
            }
            Update => {
                let closure_ty = kind.closure_ty_update(ty)?;
                syn::parse_quote! {
                    #attrs #vis fn #field(&self, f: #closure_ty) {
                        #field_trait::update(&#self_field, f)
                    }
                }
            }
            ReplaceWith => {
                let closure_ty = kind.closure_ty_replace_with(ty)?;
                let ty_get = kind.ty_get_for_replace_with(ty)?;
                syn::parse_quote! {
                    #attrs #vis fn #field(&self, f: #closure_ty) -> #ty_get {
                        #field_trait::replace_with(&#self_field, f)
                    }
                }
            }
        })
    }
    fn expand_init_closure_for_late(&self, cx: &ExpandCtxt<'_>) -> Option<TokenStream> {
        _ = self.kw_late?;
        let FieldInit { expr, .. } = self.init.as_ref()?;
        let mut expr = expr.clone();
        SelfExprVisitor::new().visit_expr_mut(&mut expr);
        let __self = self.expand_self(cx);
        let block = quote::quote_spanned!(expr.span()=> {
            let __self = &#__self;
            let __self_dyn = &#__self;
            #expr
        });
        Some(quote::quote_spanned!(expr.span()=> #[inline(never)] #[cold] || #block))
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum AccessMode {
    Get,
    GetMut,
    Set,
    Update,
    Replace,
    ReplaceWith,
    Raw,
}

impl AccessMode {
    const MODES: [Self; 7] = [
        Self::Get,
        Self::GetMut,
        Self::Set,
        Self::Update,
        Self::Replace,
        Self::ReplaceWith,
        Self::Raw,
    ];
    pub(super) const MODE_FN_NAMES: [&str; 7] = [
        "get",
        "get_mut",
        "set",
        "update",
        "replace",
        "replace_with",
        "raw",
    ];
}

impl ToTokens for AccessMode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use AccessMode::*;
        match self {
            Get => tokens.append_all(quote! { #_Get }),
            GetMut => tokens.append_all(quote! { #_GetMut }),
            Set => tokens.append_all(quote! { #_Set }),
            Update => tokens.append_all(quote! { #_Update }),
            Replace => tokens.append_all(quote! { #_Replace }),
            ReplaceWith => tokens.append_all(quote! { #_ReplaceWith }),
            Raw => tokens.append_all(quote! { #_Raw }),
        }
    }
}

impl FieldKind {
    fn field_trait(self) -> syn::Path {
        use Ty::*;
        match self {
            Self(Late::No, Ref::No(Copy), Not) => syn::parse_quote!(#_FieldCopy),
            Self(Late::No, Ref::No(Rc(_)), Not) => syn::parse_quote!(#_FieldRc),
            Self(Late::No, Ref::No(Copy), Mut) => syn::parse_quote!(#_FieldMutCopy),
            Self(Late::No, Ref::No(Rc(_)), Mut) => syn::parse_quote!(#_FieldMutRc),
            Self(Late::No, Ref::Yes, Not) => syn::parse_quote!(#_FieldRef),
            Self(Late::No, Ref::Yes, Mut) => syn::parse_quote!(#_FieldRefMut),
            Self(Late::Yes, Ref::No(Copy), Not) => syn::parse_quote!(#_FieldLateCopy),
            Self(Late::Yes, Ref::No(Rc(_)), Not) => syn::parse_quote!(#_FieldLateRc),
            Self(Late::Yes, Ref::No(Copy), Mut) => syn::parse_quote!(#_FieldLateMutCopy),
            Self(Late::Yes, Ref::No(Rc(_)), Mut) => syn::parse_quote!(#_FieldLateMutRc),
            Self(Late::Yes, Ref::Yes, Not) => syn::parse_quote!(#_FieldLateRef),
            Self(Late::Yes, Ref::Yes, Mut) => syn::parse_quote!(#_FieldLateRefMut),
        }
    }
    fn ty(self, ty: &syn::Type) -> syn::Type {
        match self {
            Self(Late::No, _, Not) => ty.clone(),
            Self(Late::No, Ref::No(_), Mut) => parse_quote_spanned!(ty.span()=> #_Cell<#ty>),
            Self(Late::No, Ref::Yes, Mut) => parse_quote_spanned!(ty.span()=> #_RefCell<#ty>),
            Self(Late::Yes, _, Not) => parse_quote_spanned!(ty.span()=> #_OnceCell<#ty>),
            Self(Late::Yes, Ref::No(_), Mut) => {
                parse_quote_spanned!(ty.span()=> #_Cell<#_Option<#ty>>)
            }
            Self(Late::Yes, Ref::Yes, Mut) => {
                parse_quote_spanned!(ty.span()=> #_RefCell<#_Option<#ty>>)
            }
        }
    }

    fn ty_get(self, ty: &syn::Type) -> syn::Type {
        use Ty::*;
        match self {
            Self(_, Ref::No(Copy), _) => syn::parse_quote!(#ty),
            Self(_, Ref::No(Rc(_)), _) => syn::parse_quote!(<#ty as #_FieldRc>::Get),
            Self(_, Ref::Yes, Not) => syn::parse_quote!(&#ty),
            Self(_, Ref::Yes, Mut) => syn::parse_quote!(#_Ref<'_, #ty>),
        }
    }
    fn ty_get_for_replace(self, ty: &syn::Type) -> Option<syn::Type> {
        use Ty::*;
        Some(match self {
            Self(.., Not) => return None,
            Self(Late::No, Ref::No(Copy) | Ref::Yes, Mut) => syn::parse_quote!(#ty),
            Self(Late::Yes, Ref::No(Copy) | Ref::Yes, Mut) => syn::parse_quote!(#_Option<#ty>),
            Self(Late::No, Ref::No(Rc(_)), Mut) => syn::parse_quote!(<#ty as #_FieldRc>::Get),
            Self(Late::Yes, Ref::No(Rc(_)), Mut) => {
                syn::parse_quote!(<#ty as #_FieldLateMutRc>::GetOption)
            }
        })
    }
    fn closure_ty_update(self, ty: &syn::Type) -> Option<syn::TypeImplTrait> {
        let ty_get = self.ty_get_for_update(ty)?;
        let ty_set = self.ty_set(ty)?;
        Some(syn::parse_quote!(impl #_FnOnce(#ty_get) -> #ty_set))
    }
    fn closure_ty_replace_with(self, ty: &syn::Type) -> Option<syn::TypeImplTrait> {
        let Self(_, Ref::Yes, Mut) = self else {
            return None;
        };
        self.closure_ty_update(ty)
    }
    fn ty_get_for_update(self, ty: &syn::Type) -> Option<syn::Type> {
        use Ty::*;
        Some(match self {
            Self(.., Not) => return None,
            Self(Late::No, Ref::No(Copy), Mut) => syn::parse_quote!(#ty),
            Self(Late::Yes, Ref::No(Copy), Mut) => syn::parse_quote!(#_Option<#ty>),
            Self(Late::No, Ref::No(Rc(_)), Mut) => syn::parse_quote!(<#ty as #_FieldRc>::Get),
            Self(Late::Yes, Ref::No(Rc(_)), Mut) => {
                syn::parse_quote!(<#ty as #_FieldLateMutRc>::GetOption)
            }
            Self(Late::No, Ref::Yes, Mut) => syn::parse_quote!(&mut #ty),
            Self(Late::Yes, Ref::Yes, Mut) => syn::parse_quote!(&mut #_Option<#ty>),
        })
    }
    fn ty_get_for_replace_with(self, ty: &syn::Type) -> Option<syn::Type> {
        let Self(_, Ref::Yes, Mut) = self else {
            return None;
        };
        self.ty_get_for_replace(ty)
    }
    fn ty_set(self, ty: &syn::Type) -> Option<syn::Type> {
        use Ty::*;
        Some(match self {
            Self(Late::No, _, Not) => return None,
            Self(_, Ref::No(Rc(_)), _) => syn::parse_quote!(<#ty as #_FieldRc>::Set),
            Self(..) => syn::parse_quote!(#ty),
        })
    }
    fn ty_get_mut(self, ty: &syn::Type) -> Option<syn::Type> {
        Some(match self {
            Self(_, Ref::Yes, Mut) => syn::parse_quote!(#_RefMut<'_, #ty>),
            Self(..) => return None,
        })
    }
}

trait TypeExt {
    // Returns the inner `T` if it matches `outer_path<T>`
    fn inner_ty(&self, outer_path: &[&str]) -> Option<&Self>;
    fn ty_kind(&self) -> Ty {
        match self.option_ty() {
            Some(inner) if inner.is_rc() => Ty::Rc(RcTy::OptionRc),
            Some(inner) if inner.is_weak() => Ty::Rc(RcTy::OptionWeak),
            None if self.is_rc() => Ty::Rc(RcTy::Rc),
            None if self.is_weak() => Ty::Rc(RcTy::Weak),
            None if self.is_rc_like() => Ty::Rc(RcTy::RcLike),
            _ => Ty::Copy,
        }
    }
    fn option_ty(&self) -> Option<&Self> {
        self.inner_ty(&["core", "option", "Option"])
    }
    fn is_rc(&self) -> bool {
        self.inner_ty(&["oop_rs", "rc", "CRc"]).is_some()
            || self.inner_ty(&["alloc", "rc", "Rc"]).is_some()
    }
    fn is_weak(&self) -> bool {
        self.inner_ty(&["oop_rs", "rc", "CWeak"]).is_some()
            || self.inner_ty(&["alloc", "rc", "Weak"]).is_some()
    }
    fn is_rc_like(&self) -> bool {
        self.inner_ty(&["oop_rs", "rc", "RcLike"]).is_some()
    }
}

impl TypeExt for syn::Type {
    fn inner_ty(&self, outer_path: &[&str]) -> Option<&Self> {
        let syn::Type::Path(syn::TypePath { path, qself: None }) = self else {
            return None;
        };
        let [prefix @ .., last] = outer_path else {
            return None;
        };
        fn match_crate(krate: &str, path: &syn::PathSegment) -> bool {
            path.arguments.is_none()
                && match krate {
                    "core" | "alloc" => path.ident == krate || path.ident == "std",
                    _ => path.ident == krate,
                }
        }
        fn match_module_path<'a>(
            prefix: &[&str],
            path: &'a Punctuated<syn::PathSegment, syn::Token![::]>,
        ) -> Option<&'a syn::PathSegment> {
            if path.len() != prefix.len() + 1 {
                return None;
            }
            let [krate, rest @ ..] = prefix else {
                return None;
            };
            let mut iter = path.iter();
            if iter.next().is_none_or(|seg| !match_crate(krate, &seg)) {
                return None;
            }
            if iter
                .by_ref()
                .zip(rest)
                .all(|(seg, expected)| seg.arguments.is_none() && seg.ident == expected)
            {
                return iter.next();
            }
            None
        }
        let path = match path.leading_colon {
            _ if path.segments.len() == outer_path.len() => {
                match_module_path(prefix, &path.segments)?
            }
            Some(_) => return None,
            None => (path.segments.len() == 1).then_some(&path.segments.first()?)?,
        };
        if path.ident != last {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(args) = &path.arguments {
            if args.args.len() == 1 {
                if let syn::GenericArgument::Type(ty) = args.args.first().unwrap() {
                    return Some(ty);
                }
            }
        }
        None
    }
}
