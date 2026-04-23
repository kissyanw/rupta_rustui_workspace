use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::quote_spanned;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

use super::constants::*;
use crate::errors::ExpandError;
use crate::expand::VisibilityExt;
use crate::expand::field::AccessMode;
use crate::expand::{ExpandCtxt, PathExt};
use crate::syntax::method::*;

/// Strip `#[builder]`, `#[builder = "..."]`, `#[helper]`, `#[helper = "..."]` attrs.
fn strip_fn_marker_attrs(attrs: &[syn::Attribute]) -> Vec<syn::Attribute> {
    attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("builder") && !attr.path().is_ident("helper"))
        .cloned()
        .collect()
}

/// Strip `#[default(...)]` from `PatType` attrs.
fn strip_default_attr(pat_type: &syn::PatType) -> syn::PatType {
    let mut pt = pat_type.clone();
    pt.attrs.retain(|attr| !attr.path().is_ident("default"));
    pt
}

trait FnExt {
    fn inputs<'a>(&'a self) -> impl Iterator<Item = &'a syn::PatType> + use<'a, Self>;
    fn input_types<'a>(&'a self) -> impl Iterator<Item = &'a syn::Type> + use<'a, Self> {
        self.inputs().map(|input| &*input.ty)
    }
    fn input_types_with_numbered_pats(&self) -> impl Iterator<Item = syn::PatType> + use<'_, Self> {
        self.inputs().enumerate().map(|(i, input)| syn::PatType {
            attrs: Vec::new(),
            pat: match &*input.pat {
                syn::Pat::Ident(syn::PatIdent { ident, .. }) => syn::parse_quote!(#ident),
                // replace non-ident patterns with numbered patterns
                _ => Box::new(numbered_pat_ident(i).into()),
            },
            colon_token: syn::parse_quote!(:),
            ty: input.ty.clone(),
        })
    }
}

impl FnExt for Method {
    fn inputs<'a>(&'a self) -> impl Iterator<Item = &'a syn::PatType> + use<'a> {
        self.inputs.iter()
    }
}

impl FnExt for Ctor {
    fn inputs<'a>(&'a self) -> impl Iterator<Item = &'a syn::PatType> + use<'a> {
        self.inputs.iter()
    }
}

impl FnExt for syn::ImplItemFn {
    fn inputs<'a>(&'a self) -> impl Iterator<Item = &'a syn::PatType> + use<'a> {
        self.sig.inputs.iter().filter_map(|input| match input {
            syn::FnArg::Typed(pat_ty) => Some(pat_ty),
            syn::FnArg::Receiver(_) => None,
        })
    }
}

impl Method {
    fn expand_replaced_receiver_ty(&self) -> syn::Type {
        let mut ty = syn::Type::clone(&self.receiver.ty);
        SelfTyVisitor(None).visit_type_mut(&mut ty);
        ty
    }
    fn expand_replaced_receiver(&self) -> syn::PatType {
        let syn::Receiver {
            attrs,
            self_token,
            mutability,
            ty,
            ..
        } = &self.receiver;
        let mut ty = ty.clone();
        SelfTyVisitor(None).visit_type_mut(&mut ty);
        syn::PatType {
            attrs: self.receiver.attrs.clone(),
            pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                attrs: attrs.clone(),
                by_ref: None,
                mutability: *mutability,
                ident: quote::format_ident!("__self", span = self_token.span),
                subpat: None,
            })),
            colon_token: syn::parse_quote!(:),
            ty,
        }
    }
    pub(super) fn expand_fn_ptr(&self, _cx: &ExpandCtxt<'_>) -> syn::TypeBareFn {
        let Self {
            asyncness: _, // TODO
            unsafety,
            abi,
            fn_token,
            generics,
            paren_token,
            output,
            ..
        } = self;
        let lifetimes =
            Option::zip(generics.lt_token, generics.gt_token).map(|(lt_token, gt_token)| {
                syn::BoundLifetimes {
                    for_token: syn::parse_quote!(for),
                    lt_token,
                    lifetimes: generics.params.clone(),
                    gt_token,
                }
            });

        fn wrap_bare_fn_arg(ty: syn::Type) -> syn::BareFnArg {
            syn::BareFnArg {
                attrs: Vec::new(),
                name: None,
                ty,
            }
        }
        let receiver = wrap_bare_fn_arg(self.expand_replaced_receiver_ty());

        syn::TypeBareFn {
            lifetimes,
            unsafety: *unsafety,
            abi: abi.clone(),
            fn_token: *fn_token,
            paren_token: *paren_token,
            inputs: [receiver]
                .into_iter()
                .chain(self.input_types().cloned().map(wrap_bare_fn_arg))
                .collect(),
            variadic: None,
            output: output.clone(),
        }
    }
    pub(super) fn expand_def_in_trait(&self) -> syn::TraitItemFn {
        let Self {
            attrs,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            receiver,
            output,
            body,
            ..
        } = self;
        syn::TraitItemFn {
            attrs: strip_fn_marker_attrs(attrs),
            sig: syn::Signature {
                constness: None,
                asyncness: *asyncness,
                unsafety: *unsafety,
                abi: abi.clone(),
                fn_token: *fn_token,
                ident: ident.clone(),
                generics: generics.clone(),
                paren_token: *paren_token,
                inputs: [syn::FnArg::Receiver(receiver.clone())]
                    .into_iter()
                    .chain(self.input_types_with_numbered_pats().map(syn::FnArg::Typed))
                    .collect(),
                variadic: None,
                output: output.clone(),
            },
            default: None,
            semi_token: syn::parse_quote_spanned!(body.span()=> ;),
        }
    }
    pub(super) fn expand_impl_in_trait(&self, cx: &ExpandCtxt<'_>) -> syn::ImplItemFn {
        let Self {
            asyncness: _, // TODO
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            receiver,
            inputs,
            output,
            body,
            ..
        } = self;
        let name = &cx.class.name;

        let args = inputs.iter().map(|pat_ty| {
            if let syn::Pat::Ident(pat_ident) = &*pat_ty.pat {
                &pat_ident.ident
            } else {
                panic!("expected ident pattern");
            }
        });

        let (generics, _, where_clause) = generics.split_for_impl();
        let inputs_stripped: syn::punctuated::Punctuated<syn::PatType, syn::Token![,]> =
            inputs.iter().map(strip_default_attr).collect();
        let params = quote_spanned!(paren_token.span.join() => (#receiver, #inputs_stripped));
        let args = quote_spanned!(paren_token.span.join() => (self, #(#args),*));
        let body = quote_spanned!(body.span() => { #_read_vtable!(<#name>::#ident) #args });

        syn::parse_quote! {
            #[inline(always)]
            #unsafety #abi #fn_token #ident #generics #params #output
            #where_clause
            #body
        }
    }

    pub(super) fn expand_impl_in_concrete(&self, cx: &ExpandCtxt<'_>) -> Option<syn::ImplItemFn> {
        let Self {
            asyncness: _, // TODO
            vis,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            receiver,
            inputs,
            output,
            body,
            ..
        } = self;
        let class = cx.class_impl_name();

        let body = body.body()?;

        let args = inputs.iter().map(|pat_ty| {
            if let syn::Pat::Ident(pat_ident) = &*pat_ty.pat {
                &pat_ident.ident
            } else {
                panic!("expected ident pattern");
            }
        });

        let (generics, _, where_clause) = generics.split_for_impl();
        let inputs_stripped: syn::punctuated::Punctuated<syn::PatType, syn::Token![,]> =
            inputs.iter().map(strip_default_attr).collect();
        let params = quote_spanned!(paren_token.span.join() => (#receiver, #inputs_stripped));
        let args = quote_spanned!(paren_token.span.join() => (self, #(#args),*));
        let body = quote_spanned!(body.span() => { #class::#ident #args });
        Some(syn::parse_quote! {
            #[inline(always)]
            #[allow(dead_code)]
            #vis #unsafety #abi #fn_token #ident #generics #params #output
            #where_clause
            #body
        })
    }

    pub(super) fn expand_def(
        &self,
        cx: &ExpandCtxt<'_>,
        super_classes: &[&syn::Ident],
    ) -> Option<syn::ImplItem> {
        let Self {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            output,
            body,
            ..
        } = self;

        // Strip #[helper]/builder and #[default] attrs before emitting
        let attrs = strip_fn_marker_attrs(&self.attrs);
        let inputs: Punctuated<syn::PatType, syn::Token![,]> =
            inputs.iter().map(strip_default_attr).collect();

        // Transform &self to &__Self in the receiver
        let transformed_receiver = self.expand_replaced_receiver();

        // Transform method body to replace 'self' with '__self' and 'super' calls
        let mut body = body.body()?.clone();
        SelfExprVisitor::new()
            .maybe_with_super_classes(super_classes)
            .visit_block_mut(&mut body);

        let name = &cx.class.name;
        let mut ty = syn::Type::clone(&self.receiver.ty);
        SelfTyVisitor(Some(&name)).visit_type_mut(&mut ty);
        body.stmts = [syn::parse_quote!(let __self_dyn = __self as #ty;)]
            .into_iter()
            .chain(body.stmts)
            .collect();

        let params = quote_spanned! { paren_token.span.join() => (#transformed_receiver, #inputs) };
        let (generics, _, where_clause) = generics.split_for_impl();
        Some(syn::parse_quote! (
            #(#attrs)*
            #constness #asyncness #unsafety #abi
            #fn_token #ident #generics #params #output
            #where_clause
            #body
        ))
    }

    pub(super) fn expand_def_for_dyn(&self, _cx: &ExpandCtxt<'_>) -> Option<syn::ImplItemFn> {
        let Self {
            attrs,
            vis,
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            receiver,
            inputs,
            output,
            body,
            ..
        } = self;

        let attrs = strip_fn_marker_attrs(attrs);
        let inputs: Punctuated<syn::PatType, syn::Token![,]> =
            inputs.iter().map(strip_default_attr).collect();

        // Require a body (abstract final methods are not meaningful)
        let mut body = body.body()?.clone();

        // Transform `self` → `__self` and `self.get()` → `__self_dyn.get()` in body
        SelfExprVisitor::new().visit_block_mut(&mut body);

        // Prepend bindings so that `__self` and `__self_dyn` resolve to `self` (the dyn ref)
        body.stmts = [
            syn::parse_quote!(let __self = self;),
            syn::parse_quote!(let __self_dyn = self;),
        ]
        .into_iter()
        .chain(body.stmts)
        .collect();

        let (generics, _, where_clause) = generics.split_for_impl();
        let params = quote_spanned!(paren_token.span.join() => (#receiver, #inputs));
        let vis = vis.expand_for_super();

        Some(syn::parse_quote! {
            #(#attrs)*
            #vis #constness #asyncness #unsafety #abi
            #fn_token #ident #generics #params #output
            #where_clause
            #body
        })
    }
}

impl OverridenMethod {
    pub(super) fn expand_def(&self, cx: &ExpandCtxt<'_>) -> Option<syn::ImplItem> {
        let classes: Vec<&syn::Ident> = self.r#override.classes.iter().collect();
        self.method.expand_def(cx, &classes)
    }
}

impl Ctor {
    pub(super) fn expand_def(&self, cx: &ExpandCtxt<'_>) -> syn::ImplItem {
        let Self {
            constness,
            abi,
            fn_token,
            ident,
            paren_token,
            inputs,
            body,
            ..
        } = self;
        let attrs = strip_fn_marker_attrs(&self.attrs);
        let inputs: Punctuated<_, syn::Token![,]> = std::iter::once(syn::parse_quote!(
            mut __self: #_CRcUninit<__Self, Self>
        ))
        .chain(inputs.iter().map(strip_default_attr))
        .collect();

        let params = quote_spanned! { paren_token.span.join() => (#inputs) };
        let body = body.expand(cx);
        syn::parse_quote! {
            #(#attrs)*
            pub #constness #abi
            #fn_token #ident #params -> #_CRc<__Self>
            #body
        }
    }

    pub(super) fn expand_impl(&self, cx: &ExpandCtxt<'_>) -> syn::ImplItem {
        let Self {
            constness,
            abi,
            fn_token,
            ident,
            paren_token,
            body,
            ..
        } = self;
        let attrs = strip_fn_marker_attrs(&self.attrs);
        let class = cx.class_impl_name();

        // input_types_with_numbered_pats already strips all attrs from PatType
        let inputs: Punctuated<_, syn::Token![,]> = self.input_types_with_numbered_pats().collect();
        let args = inputs.iter().map(|input| &input.pat);
        let params = quote_spanned! { paren_token.span.join() => (#inputs) };
        let body = quote_spanned!(body.brace.span.join() => {
            #class::#ident(#_CRcUninit::new().cast_uninit(), #(#args,)*)
        });
        syn::parse_quote! {
            #(#attrs)*
            pub #constness #abi
            #fn_token #ident #params -> #_CRc<__Self>
            #body
        }
    }
}

impl CtorBody {
    fn expand(&self, cx: &ExpandCtxt<'_>) -> syn::Block {
        let Self { brace, _self, .. } = self;
        let class = &cx.class.name;

        let self_expr = _self.self_expr().expand_struct_expr(cx);
        let super_call = self._self.expand_super_call(cx);

        let mixins = self.expand_mixins(cx);

        syn::Block {
            brace_token: *brace,
            stmts: self.stmts.iter().cloned().chain([
                syn::parse_quote!(let __data = #self_expr;),
                syn::parse_quote!(let __self = unsafe {
                    #_core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, #_PhantomData);
                    #_core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
                    __self.assume_init_except()
                };),
            ])
            .chain(mixins)
            .chain([
                syn::parse_quote!(let __self = #super_call;),
                syn::parse_quote!(let __self_dyn = &*__self as &#class;),
            ])
            .chain(self._self.stmts().iter().cloned().map(|mut stmt| {
                SelfExprVisitor::new().visit_stmt_mut(&mut stmt);
                stmt
            }))
            .chain(std::iter::once(syn::Stmt::Expr(syn::parse_quote!(__self), None)))
            .collect(),
        }
    }

    fn expand_mixins(&self, cx: &ExpandCtxt<'_>) -> impl IntoIterator<Item = syn::Stmt> {
        let Some((superclass, withs)) = cx.class.class_attrs.extends_and_withs() else {
            return vec![];
        };

        let mut stmts = Vec::new();

        // Build the mixin chain from innermost to outermost
        let mixin_chain: Vec<_> = withs.iter().collect();

        // Generate mixin calls in reverse order (outermost to innermost)
        for (i, mixin) in mixin_chain.iter().enumerate().rev() {
            // Build the super type for this mixin
            let current_super = mixin_chain.iter().take(i).fold(
                quote!(#_CClass<#superclass, __Self>),
                |acc, m| quote!(#_MClass<#acc, #m, __Self>),
            );

            let stmt: syn::Stmt = syn::parse_quote! {
                let __self = #_MClass::<#current_super, #mixin, __Self>::__mixin(__self);
            };
            stmts.push(stmt);
        }

        if cx.class.class_attrs.extends.is_none() {
            stmts.push(syn::parse_quote!(let __self = __self.cast_uninit::<()>();));
        }

        stmts
    }
}

impl CtorSelf {
    fn expand_super_call(&self, cx: &ExpandCtxt<'_>) -> syn::Expr {
        match &self.self_expr().super_expr {
            Some(super_expr) => super_expr.expand(cx),
            None => syn::parse_quote!(__self.assume_init()),
        }
    }
}

impl CtorSelfExpr {
    fn expand_struct_expr(&self, cx: &ExpandCtxt<'_>) -> syn::ExprStruct {
        syn::ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: syn::Path::from(cx.data_name()),
            brace_token: self.brace,
            fields: self.expand_struct_expr_fields(cx),
            dot2_token: None,
            rest: None,
        }
    }
}

impl CtorCallSuper {
    fn expand(&self, cx: &ExpandCtxt<'_>) -> syn::Expr {
        let Self {
            method,
            paren,
            args,
            chain,
            ..
        } = self;

        // When using mixins, we need to call the underlying superclass method, not the mixin
        let func = if cx.class.class_attrs.withs.is_some()
            // If there are mixins, call the method on the underlying superclass
            && let Some(extends) = &cx.class.class_attrs.extends
        {
            let superclass = &extends.classes;
            syn::parse_quote!(#_CClass::<#superclass, __Self>::#method)
        } else {
            syn::parse_quote!(<Self as #_HasSuper>::Super::#method)
        };

        let base = syn::Expr::Call(syn::ExprCall {
            attrs: Vec::new(),
            func,
            paren_token: *paren,
            args: std::iter::once(syn::parse_quote!(__self))
                .chain(args.iter().cloned())
                .collect(),
        });

        // Apply any chained method calls (e.g. `.build()` in `..Super::builder().build()`)
        chain.iter().fold(base, |expr, item| {
            syn::Expr::MethodCall(syn::ExprMethodCall {
                attrs: Vec::new(),
                receiver: Box::new(expr),
                dot_token: syn::parse_quote!(.),
                method: item.method.clone(),
                turbofish: None,
                paren_token: item.paren,
                args: item.args.clone(),
            })
        })
    }
}

fn numbered_pat_ident(i: usize) -> syn::PatIdent {
    syn::PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: None,
        subpat: None,
        ident: quote::format_ident!("__{i}"),
    }
}

/// Replace `Self` with `__Self` in type.
struct SelfTyVisitor<'a>(Option<&'a syn::Ident>);

impl VisitMut for SelfTyVisitor<'_> {
    fn visit_type_path_mut(&mut self, i: &mut syn::TypePath) {
        if i.path.is_ident("Self") {
            #[expect(non_snake_case)]
            let __Self = match self.0 {
                Some(ident) => ident,
                None => &quote::format_ident!("__Self"),
            };
            i.path = syn::parse_quote_spanned!(i.path.span()=> #__Self);
            return;
        }
        syn::visit_mut::visit_type_path_mut(self, i);
    }
}

/// Replace
/// - `self` with `__self`
/// - `super.method(...args)` with `read_vtable!(<Super as ParentClass>::method)(__self, ...args)`
/// - `(super as SuperClass).method(...args)` with `read_vtable!(<Super as SuperClass>::method)(__self, ...args)`
/// - `self.get()`, `self.set()`, ... with `__self_dyn.get()`, `__self_dyn.set()`, ...
/// in expressions and statements.
pub(super) struct SelfExprVisitor<'a> {
    super_classes: &'a [&'a syn::Ident],
}

impl<'a> SelfExprVisitor<'a> {
    pub(super) fn new() -> Self {
        Self { super_classes: &[] }
    }
    fn maybe_with_super_classes(&mut self, super_classes: &'a [&'a syn::Ident]) -> &mut Self {
        self.super_classes = super_classes;
        self
    }
}

impl VisitMut for SelfExprVisitor<'_> {
    /// replace `self` with `__self`
    fn visit_expr_path_mut(&mut self, expr: &mut syn::ExprPath) {
        if expr.path.is_ident("self") || expr.path.is_ident("super") {
            expr.path = syn::parse_quote_spanned!(expr.path.span()=> __self);
            return;
        }
        syn::visit_mut::visit_expr_path_mut(self, expr);
    }
    /// Replace
    /// - `super.method(...args)` with `read_vtable!(<Super as ParentClass>::method)(__self, ...args)`
    /// - `(super as SuperClass).method(...args)` with `read_vtable!(<Super as SuperClass>::method)(__self, ...args)`
    /// - `self.get()`, `self.set()`, ... with `__self_dyn.get()`, `__self_dyn.set()`, ...
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        let syn::Expr::MethodCall(syn::ExprMethodCall {
            receiver,
            attrs,
            method,
            turbofish,
            paren_token,
            args,
            ..
        }) = expr
        else {
            syn::visit_mut::visit_expr_mut(self, expr);
            return;
        };
        args.iter_mut().for_each(|arg| self.visit_expr_mut(arg));

        match &mut **receiver {
            // `(super as SuperClass).method(...args)`
            syn::Expr::Paren(e) if matches!(&*e.expr, syn::Expr::Cast(e) if matches!(&*e.expr, syn::Expr::Path(path) if path.path.is_ident("super")) && matches!(&*e.ty, syn::Type::Path(_))) =>
            {
                let syn::Expr::Cast(syn::ExprCast {
                    expr: receiver, ty, ..
                }) = &mut *e.expr
                else {
                    unreachable!()
                };
                let syn::Type::Path(syn::TypePath {
                    path: super_class, ..
                }) = &**ty
                else {
                    unreachable!()
                };
                let super_class = super_class.as_ident();
                let func = super_method(
                    &method,
                    &[super_class],
                    receiver.span(),
                    turbofish.as_ref(),
                );
                args.insert(0, syn::parse_quote_spanned!(receiver.span()=> __self));
                *expr = syn::Expr::Call(syn::ExprCall {
                    attrs: std::mem::take(attrs),
                    func,
                    paren_token: *paren_token,
                    args: std::mem::take(args),
                });
            }
            // `super.method(...args)`
            syn::Expr::Path(path) if path.path.is_ident("super") => {
                path.path = syn::parse_quote_spanned!(path.path.span()=> __self);
                let func = super_method(&method, self.super_classes, path.span(), turbofish.as_ref());
                args.insert(0, syn::Expr::clone(receiver));
                *expr = syn::Expr::Call(syn::ExprCall {
                    attrs: std::mem::take(attrs),
                    func,
                    paren_token: *paren_token,
                    args: std::mem::take(args),
                });
            }
            // `self.get()`, `self.set()`, ...
            syn::Expr::Path(path)
                if path.path.is_ident("self")
                    && AccessMode::MODE_FN_NAMES.into_iter().any(|m| method == m)
                    && args.is_empty() =>
            {
                *path = syn::parse_quote_spanned!(path.path.span()=> __self_dyn);
            }
            _ => {
                self.visit_expr_mut(receiver);
            }
        }
    }
    fn visit_macro_mut(&mut self, i: &mut syn::Macro) {
        i.tokens = syn::parse::Parser::parse2(
            |input: ParseStream<'_>| transform_self_expr(input, self.super_classes),
            std::mem::take(&mut i.tokens),
        )
        .unwrap_or_default();
    }
    fn visit_stmt_mut(&mut self, stmt: &mut syn::Stmt) {
        match stmt {
            syn::Stmt::Expr(expr, _) => self.visit_expr_mut(expr),
            syn::Stmt::Local(local) => self.visit_local_mut(local),
            syn::Stmt::Macro(macro_) => self.visit_stmt_macro_mut(macro_),
            // ignore nested items
            syn::Stmt::Item(_) => {}
        }
    }
}

/// Replace:
/// - `self` with `__self`
/// - `super.method(...args)` with `read_vtable!(<Super as ParentClass>::method)(__self, ...args)`
/// in expressions and statements.
fn transform_self_expr(
    input: ParseStream<'_>,
    super_classes: &[&syn::Ident],
) -> syn::Result<TokenStream> {
    let mut tokens = TokenStream::new();
    loop {
        let Ok(self_or_super) = input.parse::<SelfOrSuper>() else {
            match input.parse::<TokenTree>() {
                Ok(TokenTree::Group(group)) => {
                    let group = Group::new(
                        group.delimiter(),
                        syn::parse::Parser::parse2(
                            |input: ParseStream<'_>| transform_self_expr(input, super_classes),
                            group.stream(),
                        )
                        .unwrap_or_default(),
                    );
                    group.to_tokens(&mut tokens);
                }
                Ok(tt) => tt.to_tokens(&mut tokens),
                Err(_) => break,
            };
            continue;
        };

        if let Ok(colons) = input.parse::<syn::Token![::]>() {
            self_or_super.to_tokens(&mut tokens);
            colons.to_tokens(&mut tokens);
            continue;
        }

        let __self = quote::format_ident!("__self", span = self_or_super.span());
        let __self_dyn = quote::format_ident!("__self_dyn", span = __self.span());

        if let SelfOrSuper::SuperToken(__super) = self_or_super
            && input.peek(syn::Token![.])
            && input.peek2(syn::Ident)
            && (input.peek3(syn::token::Paren) || input.peek3(syn::Token![::]))
        {
            // `super.method(...args)`
            let _: syn::Token![.] = input.parse()?;
            let method: syn::Ident = input.parse()?;

            let turbofish: Option<syn::AngleBracketedGenericArguments> = input
                .peek(syn::Token![::])
                .then(|| input.parse())
                .transpose()?;
            let content;
            let paren = syn::parenthesized!(content in input);
            let args = content.cursor().token_stream();
            let args = syn::parse::Parser::parse2(
                |input: ParseStream<'_>| transform_self_expr(input, super_classes),
                args,
            )
            .unwrap_or_default();

            let func = super_method(&method, super_classes, __super.span(), turbofish.as_ref());

            let args = quote_spanned!(paren.span.join()=> (#__self, #args));
            quote::quote_each_token! { tokens #func #args }
        } else if let SelfOrSuper::SelfToken(__self) = self_or_super
            && input.fork().parse::<FieldAccessor>().is_ok()
        {
            // `self.get()`, `self.set()`, ...
            __self_dyn.to_tokens(&mut tokens);
        } else if input.peek(syn::Token![as]) {
            // `self as Class`
            __self_dyn.to_tokens(&mut tokens);
        } else {
            // other `self` or `super` expressions
            __self.to_tokens(&mut tokens);
        }
    }
    Ok(tokens)
}

// `.get()`, `.set()`, ...
struct FieldAccessor {
    _method: syn::Ident,
}

impl Parse for FieldAccessor {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![.]>()?;
        let _method: syn::Ident = input.parse()?;
        if !AccessMode::MODE_FN_NAMES.into_iter().any(|m| _method == m) {
            return Err(syn::Error::new(
                _method.span(),
                "expected field accessor method",
            ));
        }
        // ensure there are no arguments
        let _content;
        syn::parenthesized!(_content in input);
        Ok(Self { _method })
    }
}

fn super_method(
    method: &syn::Ident,
    super_classes: &[&syn::Ident],
    receiver_span: Span,
    _turbofish: Option<&syn::AngleBracketedGenericArguments>,
) -> Box<syn::Expr> {
    match super_classes {
        [] => {
            let err = syn::Error::new(receiver_span, ExpandError::SuperCallInNonOverriddenMethod)
                .into_compile_error();
            syn::parse_quote!(#err)
        }
        [super_class] => syn::parse_quote!(#_read_vtable!(<Super as #super_class>::#method)),
        super_classes => syn::parse_quote!(#_read_vtable!(<Super as [#(#super_classes),*]>::#method)),
    }
}

enum SelfOrSuper {
    SelfToken(syn::Token![self]),
    SuperToken(syn::Token![super]),
}

impl Parse for SelfOrSuper {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![self]) {
            Ok(Self::SelfToken(input.parse()?))
        } else if lookahead.peek(syn::Token![super]) {
            Ok(Self::SuperToken(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for SelfOrSuper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::SelfToken(self_token) => self_token.to_tokens(tokens),
            Self::SuperToken(super_token) => super_token.to_tokens(tokens),
        }
    }
}
