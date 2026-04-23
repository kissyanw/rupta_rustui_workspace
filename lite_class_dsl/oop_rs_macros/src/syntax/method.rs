use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{Pair, Punctuated};
use syn::spanned::Spanned;

use crate::errors::ParseError;

mod kw {
    syn::custom_keyword!(Super);
}

pub struct Methods {
    pub finals: Vec<Method>,
    pub owns: Vec<Method>,
    pub overrides: Vec<OverridenMethod>,
}

impl Methods {
    pub fn iter(&self) -> impl Iterator<Item = &Method> + use<'_> {
        self.finals
            .iter()
            .chain(self.owns.iter())
            .chain(self.overrides.iter().map(|o| &o.method))
    }
}

pub struct Override {
    pub(super) _tk_override: syn::Token![override],
    pub(super) _paren: syn::token::Paren,
    pub classes: Punctuated<syn::Ident, syn::Token![,]>,
}

#[derive(Default)]
pub struct MethodModifiers {
    pub tk_final: Option<syn::Token![final]>,
    pub override_info: Option<Override>,
}

pub struct Ctor {
    pub attrs: Vec<syn::Attribute>,
    #[allow(dead_code)]
    pub vis: syn::Visibility,
    pub constness: Option<syn::Token![const]>,
    pub abi: Option<syn::Abi>,
    pub fn_token: syn::Token![fn],
    pub ident: syn::Ident,
    pub paren_token: syn::token::Paren,
    pub inputs: Punctuated<syn::PatType, syn::Token![,]>,
    pub(super) _tk_self: syn::Token![Self],
    pub body: CtorBody,
}

pub struct CtorBody {
    pub brace: syn::token::Brace,
    pub stmts: Vec<syn::Stmt>,
    pub _self: CtorSelf,
}

pub enum CtorSelf {
    SelfExpr(CtorSelfExpr),
    SelfStmt(CtorSelfStmt),
}

impl CtorSelf {
    pub fn self_expr(&self) -> &CtorSelfExpr {
        match self {
            CtorSelf::SelfExpr(expr) => expr,
            CtorSelf::SelfStmt(stmt) => &stmt.self_expr,
        }
    }
    pub fn stmts(&self) -> &[syn::Stmt] {
        match self {
            CtorSelf::SelfExpr(_) => &[],
            CtorSelf::SelfStmt(stmt) => &stmt.stmts,
        }
    }
}

pub struct CtorSelfExpr {
    pub(super) _tk_self_ty: syn::Token![Self],
    pub brace: syn::token::Brace,
    pub fields: Punctuated<FieldValue, syn::Token![,]>,
    pub super_expr: Option<CtorCallSuper>,
}

pub struct FieldValue {
    pub attrs: Vec<syn::Attribute>,
    pub member: syn::Ident,

    /// The colon in `Struct { x: x }`. If written in shorthand like
    /// `Struct { x }`, there is no colon.
    pub(super) _tk_colon: Option<syn::Token![:]>,

    pub expr: syn::Expr,
}

pub struct CtorCallSuperChain {
    pub method: syn::Ident,
    pub paren: syn::token::Paren,
    pub args: Punctuated<syn::Expr, syn::Token![,]>,
}

pub struct CtorCallSuper {
    pub(super) _kw_super: kw::Super,
    pub method: syn::Ident,
    pub paren: syn::token::Paren,
    pub args: Punctuated<syn::Expr, syn::Token![,]>,
    /// Additional method calls chained after the super call, e.g. `.build()` in
    /// `..Super::two_w_builder().build()`
    pub chain: Vec<CtorCallSuperChain>,
}

pub struct CtorSelfStmt {
    pub(super) _tk_let: syn::Token![let],
    pub(super) _tk_mut: Option<syn::Token![mut]>,
    pub(super) _self_binding: syn::Token![self],
    pub(super) _tk_eq: syn::Token![=],
    pub self_expr: CtorSelfExpr,
    pub(super) _tk_semi: syn::Token![;],
    pub stmts: Vec<syn::Stmt>,
    pub(super) _return_self: syn::Token![self],
}

pub struct ImplItemFn {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub sig: syn::Signature,
    pub body: FnBody,
}

pub struct Method {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub constness: Option<syn::Token![const]>,
    pub asyncness: Option<syn::Token![async]>,
    pub unsafety: Option<syn::Token![unsafe]>,
    pub abi: Option<syn::Abi>,
    pub fn_token: syn::Token![fn],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub paren_token: syn::token::Paren,
    pub receiver: syn::Receiver,
    pub inputs: Punctuated<syn::PatType, syn::Token![,]>,
    pub output: syn::ReturnType,
    pub body: FnBody,
    pub tk_final: Option<syn::Token![final]>,
}

pub enum FnBody {
    Absent(syn::Token![;]),
    Present(syn::Block),
}

impl FnBody {
    pub fn body(&self) -> Option<&syn::Block> {
        match self {
            FnBody::Absent(_) => None,
            FnBody::Present(block) => Some(block),
        }
    }
}

pub struct OverridenMethod {
    pub r#override: Override,
    pub method: Method,
}

// TryFrom implementations

impl Parse for ImplItemFn {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;
        let sig: syn::Signature = input.parse()?;
        let lookahead1 = input.lookahead1();
        let body = if lookahead1.peek(syn::Token![;]) {
            FnBody::Absent(input.parse()?)
        } else if lookahead1.peek(syn::token::Brace) {
            FnBody::Present(input.parse()?)
        } else {
            return Err(lookahead1.error());
        };

        Ok(Self {
            attrs,
            vis,
            sig,
            body,
        })
    }
}

impl ToTokens for FnBody {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FnBody::Absent(tk_semi) => tk_semi.to_tokens(tokens),
            FnBody::Present(block) => block.to_tokens(tokens),
        }
    }
}

impl TryFrom<ImplItemFn> for syn::ImplItemFn {
    type Error = syn::Error;

    fn try_from(item_fn: ImplItemFn) -> syn::Result<Self> {
        let block = match item_fn.body {
            FnBody::Absent(tk_semi) => {
                return Err(syn::Error::new(tk_semi.span(), ParseError::ExpectedFnBody));
            }
            FnBody::Present(block) => block,
        };

        Ok(syn::ImplItemFn {
            attrs: item_fn.attrs,
            vis: item_fn.vis,
            defaultness: None,
            sig: item_fn.sig,
            block,
        })
    }
}

impl TryFrom<ImplItemFn> for Ctor {
    type Error = syn::Error;

    fn try_from(item_fn: ImplItemFn) -> syn::Result<Self> {
        // Verify it has `-> Self` return type
        let _tk_self = if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output
            && let syn::Type::Path(type_path) = ty.as_ref()
            && let Some(ident) = type_path.path.get_ident()
            && ident == "Self"
        {
            syn::Token![Self](ident.span())
        } else {
            panic!("expected `-> Self` return type");
        };

        // Extract inputs as PatType (no self receiver in ctors)
        let inputs = item_fn
            .sig
            .inputs
            .into_pairs()
            .map(|input| match input.into_tuple() {
                (syn::FnArg::Typed(pat_type), comma) => Pair::new(pat_type, comma),
                (syn::FnArg::Receiver(_), _) => {
                    panic!("constructor cannot have `self` receiver");
                }
            })
            .collect();

        // Parse body as CtorBody
        let body = match item_fn.body {
            FnBody::Absent(tk_semi) => {
                return Err(syn::Error::new(
                    tk_semi.span(),
                    ParseError::ExpectedCtorBody,
                ));
            }
            FnBody::Present(block) => block,
        };
        let body = CtorBody::try_from(body)?;

        Ok(Ctor {
            attrs: item_fn.attrs,
            vis: item_fn.vis,
            constness: item_fn.sig.constness,
            abi: item_fn.sig.abi,
            fn_token: item_fn.sig.fn_token,
            ident: item_fn.sig.ident,
            paren_token: item_fn.sig.paren_token,
            inputs,
            _tk_self,
            body,
        })
    }
}

impl TryFrom<syn::Block> for CtorBody {
    type Error = syn::Error;

    fn try_from(block: syn::Block) -> syn::Result<Self> {
        let brace = block.brace_token;
        let mut stmts = Vec::new();
        let mut stmts_iter = block.stmts.into_iter();

        // Collect statements until we find `Self { ... }` or `let self = Self { ... };`
        let mut _self = None;
        while let Some(stmt) = stmts_iter.next() {
            match stmt {
                // find `Self { ... }`
                syn::Stmt::Expr(syn::Expr::Struct(expr), _) if expr.path.is_ident("Self") => {
                    _self = Some(CtorSelf::SelfExpr(CtorSelfExpr::try_from(expr)?));
                    break;
                }
                // find `let self = Self { ... };`
                syn::Stmt::Local(local)
                    if matches!(&local.pat, syn::Pat::Ident(pat_ident) if pat_ident.ident == "self")
                        && local.init.as_ref().is_some_and(|init| {
                            matches!(&*init.expr, syn::Expr::Struct(expr) if expr.path.is_ident("Self")) 
                        }) =>
                {
                    // Collect remaining statements (after let self = ...)
                    _self = Some(CtorSelf::SelfStmt(CtorSelfStmt::try_from((
                        local,
                        stmts_iter.collect(),
                    ))?));
                    break;
                }
                _ => stmts.push(stmt),
            }
        }

        // Try to parse the remaining statements as CtorSelf
        let Some(_self) = _self else {
            return Err(syn::Error::new(
                brace.span.join(),
                ParseError::CtorBodyMissingConstruction,
            ));
        };

        Ok(CtorBody {
            brace,
            stmts,
            _self,
        })
    }
}

impl TryFrom<syn::ExprStruct> for CtorSelfExpr {
    type Error = syn::Error;

    fn try_from(expr_struct: syn::ExprStruct) -> syn::Result<Self> {
        let span = expr_struct.path.span();

        // Verify it's Self
        if !matches!(expr_struct.path.segments.first(), Some(seg) if seg.ident == "Self") {
            return Err(syn::Error::new(span, ParseError::ExpectedSelfConstruction));
        }

        let tk_self_ty = syn::Token![Self](span);
        let brace = expr_struct.brace_token;

        // Parse fields and optional super expression
        let mut fields = Punctuated::new();
        let mut super_expr = None;

        for field in expr_struct.fields {
            fields.push(field.try_into()?);
        }

        // Check for ..Super::ctor(...) in rest
        if let Some(rest) = expr_struct.rest {
            super_expr = Some(CtorCallSuper::try_from(*rest)?);
        }

        Ok(CtorSelfExpr {
            _tk_self_ty: tk_self_ty,
            brace,
            fields,
            super_expr,
        })
    }
}

impl TryFrom<syn::FieldValue> for FieldValue {
    type Error = syn::Error;

    fn try_from(field: syn::FieldValue) -> syn::Result<Self> {
        let member = match field.member {
            syn::Member::Named(ident) => ident,
            syn::Member::Unnamed(index) => {
                return Err(syn::Error::new(
                    index.span(),
                    ParseError::ExpectedNamedField,
                ));
            }
        };
        Ok(FieldValue {
            attrs: field.attrs,
            member,
            _tk_colon: field.colon_token,
            expr: field.expr,
        })
    }
}

impl TryFrom<syn::Expr> for CtorCallSuper {
    type Error = syn::Error;

    fn try_from(expr: syn::Expr) -> syn::Result<Self> {
        // Unwrap any trailing method calls like `.build()`, `.chain2()`, etc.
        // These are stored and re-applied after the base super call expansion.
        let mut chain_items = Vec::new();
        let mut current = expr;
        loop {
            match current {
                syn::Expr::MethodCall(mc) => {
                    chain_items.push(CtorCallSuperChain {
                        method: mc.method,
                        paren: mc.paren_token,
                        args: mc.args,
                    });
                    current = *mc.receiver;
                }
                _ => break,
            }
        }
        // chain_items was collected inside-out (outermost first), reverse to get proper order
        chain_items.reverse();

        // Expect Super::method(...) as the base
        if let syn::Expr::Call(call) = current
            && let syn::Expr::Path(path) = *call.func
            && let Ok([_super, method]) =
                <Box<[_; 2]>>::try_from(path.path.segments.into_iter().collect::<Vec<_>>())
                    .map(|arr| *arr)
            && _super.ident == "Super"
            && _super.arguments.is_none()
        {
            return Ok(CtorCallSuper {
                _kw_super: kw::Super(_super.ident.span()),
                method: method.ident,
                paren: call.paren_token,
                args: call.args,
                chain: chain_items,
            });
        }

        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            ParseError::ExpectedSuperCall,
        ))
    }
}

impl TryFrom<(syn::Local, Vec<syn::Stmt>)> for CtorSelfStmt {
    type Error = syn::Error;

    fn try_from((local, mut stmts): (syn::Local, Vec<syn::Stmt>)) -> syn::Result<Self> {
        // First statement should be let self = Self { ... };
        let syn::Pat::Ident(pat_ident) = local.pat else {
            return Err(syn::Error::new(
                local.pat.span(),
                ParseError::ExpectedSelfBinding,
            ));
        };
        if let Some(by_ref) = pat_ident.by_ref {
            return Err(syn::Error::new(
                by_ref.span(),
                ParseError::UnexpectedByRefPattern,
            ));
        }
        if let Some(subpat) = pat_ident.subpat {
            return Err(syn::Error::new(
                subpat.0.span(),
                ParseError::UnexpectedSubpatPattern,
            ));
        }
        if pat_ident.ident != "self" {
            return Err(syn::Error::new(
                pat_ident.span(),
                ParseError::ExpectedLetSelfPattern,
            ));
        }

        let tk_let = local.let_token;
        let self_binding = syn::Token![self](pat_ident.ident.span());

        // Extract eq_token and expr from LocalInit
        let Some(init) = local.init.as_ref() else {
            return Err(syn::Error::new(
                local.let_token.span,
                ParseError::ExpectedInitializer,
            ));
        };
        let tk_eq = init.eq_token;

        let syn::Expr::Struct(expr_struct) = init.expr.as_ref() else {
            return Err(syn::Error::new(
                local.let_token.span,
                ParseError::ExpectedSelfConstruction,
            ));
        };

        let self_expr = CtorSelfExpr::try_from(expr_struct.clone())?;
        let tk_semi = local.semi_token;

        // Last statement should be returning self
        let return_self = if let Some(syn::Stmt::Expr(syn::Expr::Path(path), _)) = stmts.pop()
            && let Some(ident) = path.path.get_ident()
            && ident == "self"
        {
            syn::Token![self](ident.span())
        } else {
            return Err(syn::Error::new(
                stmts.last().map_or(tk_semi.span, |stmt| stmt.span()),
                ParseError::ExpectedSelfReturn,
            ));
        };

        Ok(CtorSelfStmt {
            _tk_let: tk_let,
            _tk_mut: pat_ident.mutability,
            _self_binding: self_binding,
            _tk_eq: tk_eq,
            self_expr,
            _tk_semi: tk_semi,
            stmts,
            _return_self: return_self,
        })
    }
}

impl TryFrom<ImplItemFn> for Method {
    type Error = syn::Error;

    fn try_from(item_fn: ImplItemFn) -> syn::Result<Self> {
        // Extract receiver and other inputs
        let mut inputs_iter = item_fn.sig.inputs.into_iter();
        let receiver = match inputs_iter.next() {
            Some(syn::FnArg::Receiver(r)) => r,
            Some(syn::FnArg::Typed(pat_ty)) => {
                return Err(syn::Error::new(
                    pat_ty.span(),
                    ParseError::MethodMustHaveSelfReceiver,
                ));
            }
            None => {
                return Err(syn::Error::new(
                    item_fn.sig.paren_token.span.join(),
                    ParseError::MethodMissingSelfReceiver,
                ));
            }
        };

        let mut inputs = Punctuated::new();
        for input in inputs_iter {
            match input {
                syn::FnArg::Typed(pat_type) => {
                    inputs.push(pat_type);
                }
                syn::FnArg::Receiver(r) => {
                    return Err(syn::Error::new(
                        r.span(),
                        ParseError::MethodMultipleSelfReceivers,
                    ));
                }
            }
        }

        Ok(Method {
            attrs: item_fn.attrs,
            vis: item_fn.vis,
            constness: item_fn.sig.constness,
            asyncness: item_fn.sig.asyncness,
            unsafety: item_fn.sig.unsafety,
            abi: item_fn.sig.abi,
            fn_token: item_fn.sig.fn_token,
            ident: item_fn.sig.ident,
            generics: item_fn.sig.generics,
            paren_token: item_fn.sig.paren_token,
            receiver,
            inputs,
            output: item_fn.sig.output,
            body: item_fn.body,
            tk_final: None,
        })
    }
}

impl Method {
    /// Extract and parse the #[method(...)] attribute from attrs
    /// Returns MethodModifiers containing final and override information
    /// The method attribute will be removed from attrs
    pub fn extract_method_modifiers(&mut self) -> syn::Result<MethodModifiers> {
        // Find #[method(...)] attribute
        let method_attr = self
            .attrs
            .extract_if(.., |attr| attr.path().is_ident("method"))
            .next();

        // Parse the attribute to look for final and override(ClassName)
        // Expected format: #[method(final, override(ClassName))] or #[method(final)] or #[method(override(Foo))]
        if let Some(attr) = method_attr
            && let syn::Meta::List(meta_list) = &attr.meta
        {
            // Parse the tokens inside method(...)
            fn parse_method_modifiers(input: ParseStream<'_>) -> syn::Result<MethodModifiers> {
                let mut tk_final = None;
                let mut override_info = None;

                while !input.is_empty() {
                    let lookahead = input.lookahead1();

                    if lookahead.peek(syn::Token![final]) {
                        tk_final = Some(input.parse()?);
                    } else if lookahead.peek(syn::Token![override]) {
                        let tk_override = input.parse()?;

                        // Parse (ClassName)
                        let content;
                        let paren = syn::parenthesized!(content in input);
                        let classes = content.parse_terminated(syn::Ident::parse, syn::Token![,])?;

                        override_info = Some(Override {
                            _tk_override: tk_override,
                            _paren: paren,
                            classes,
                        });
                    } else {
                        return Err(lookahead.error());
                    }

                    // Parse optional comma
                    let _: Option<syn::Token![,]> = input.parse()?;
                }

                Ok(MethodModifiers {
                    tk_final,
                    override_info,
                })
            }

            let modifiers = meta_list.parse_args_with(parse_method_modifiers)?;
            if let Some(tk_final) = &modifiers.tk_final {
                if modifiers.override_info.is_some() {
                    return Err(syn::Error::new(tk_final.span(), ParseError::FinalAndOverrideConflict));
                }
            }
            return Ok(modifiers);
        }

        // No #[method(...)] attribute found
        Ok(MethodModifiers::default())
    }
}
