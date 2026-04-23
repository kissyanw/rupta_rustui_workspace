// ToTokens implementations for all syntax types

use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};

use super::class::*;
use super::field::*;
use super::method::*;

// ToTokens implementations for field.rs

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);
        if self.kw_late.is_some() {
            tokens.append_all(quote_spanned! { self.field.span() => #[late] });
        }
        self._tk_let.to_tokens(tokens);
        self.tk_mut.to_tokens(tokens);
        self.field.to_tokens(tokens);
        self.tk_colon.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.init.to_tokens(tokens);
        self._tk_semi.to_tokens(tokens);
    }
}

impl ToTokens for FieldInit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._tk_eq.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

// ToTokens implementations for method.rs

impl ToTokens for Methods {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.finals);
        tokens.append_all(&self.owns);
        tokens.append_all(&self.overrides);
    }
}

impl ToTokens for Override {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._tk_override.to_tokens(tokens);
        self._paren.surround(tokens, |tokens| {
            self.classes.to_tokens(tokens);
        });
    }
}

impl ToTokens for Ctor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);
        self.vis.to_tokens(tokens);
        self.constness.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.inputs.to_tokens(tokens);
        });
        tokens.append_all(quote! { -> });
        self._tk_self.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl ToTokens for CtorBody {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.brace.surround(tokens, |tokens| {
            tokens.append_all(&self.stmts);
            self._self.to_tokens(tokens);
        });
    }
}

impl ToTokens for CtorSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            CtorSelf::SelfExpr(expr) => expr.to_tokens(tokens),
            CtorSelf::SelfStmt(stmt) => stmt.to_tokens(tokens),
        }
    }
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);
        self.member.to_tokens(tokens);
        if let Some(colon) = &self._tk_colon {
            colon.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }
}

impl ToTokens for CtorSelfExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._tk_self_ty.to_tokens(tokens);
        self.brace.surround(tokens, |tokens| {
            self.fields.to_tokens(tokens);
            if let Some(super_expr) = &self.super_expr {
                // If we have fields, we need a comma before ..
                if !self.fields.is_empty() {
                    tokens.append_all(quote! { , });
                }
                tokens.append_all(quote! { .. });
                super_expr.to_tokens(tokens);
            }
        });
    }
}

impl ToTokens for CtorCallSuper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._kw_super.to_tokens(tokens);
        tokens.append_all(quote! { :: });
        self.method.to_tokens(tokens);
        self.paren.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
    }
}

impl ToTokens for CtorSelfStmt {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self._tk_let.to_tokens(tokens);
        self._tk_mut.to_tokens(tokens);
        self._self_binding.to_tokens(tokens);
        self._tk_eq.to_tokens(tokens);
        self.self_expr.to_tokens(tokens);
        self._tk_semi.to_tokens(tokens);
        tokens.append_all(&self.stmts);
        self._return_self.to_tokens(tokens);
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);

        // Add #[method(final)] attribute if this method is final
        if self.tk_final.is_some() {
            tokens.append_all(quote! { #[method(final)] });
        }

        self.vis.to_tokens(tokens);
        self.constness.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.receiver.to_tokens(tokens);
            if !self.inputs.is_empty() {
                tokens.append_all(quote! { , });
            }
            self.inputs.to_tokens(tokens);
        });
        self.output.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl ToTokens for OverridenMethod {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // Output the method's existing attributes
        tokens.append_all(&self.method.attrs);

        // Build the method attribute with override and optionally final
        let _override = &self.r#override;
        let attr_content = if self.method.tk_final.is_some() {
            quote! { #[method(#_override, final)] }
        } else {
            quote! { #[method(#_override)] }
        };
        tokens.append_all(attr_content);

        // Output the rest of the method
        self.method.to_tokens(tokens);
    }
}

// ToTokens implementations for class.rs

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.attrs);

        // Generate #[class(...)] attribute if there are class attributes
        if !self.class_attrs.is_empty() {
            let class_attrs_tokens = self.class_attrs.to_token_stream();
            tokens.append_all(quote! { #[class(#class_attrs_tokens)] });
        }

        self.vis.to_tokens(tokens);
        self._tk_type.to_tokens(tokens);
        self.name.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self._tk_eq.to_tokens(tokens);
        self.kind.to_tokens(tokens);
        self._tk_lt.to_tokens(tokens);
        self.brace.surround(tokens, |tokens| {
            self.items.to_tokens(tokens);
        });
        self._tk_gt.to_tokens(tokens);
        self._tk_semi.to_tokens(tokens);
    }
}

impl ToTokens for ClassKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ClassKind::Class(kw) => kw.to_tokens(tokens),
            ClassKind::Interface(kw) => kw.to_tokens(tokens),
            ClassKind::Mixin(kw) => kw.to_tokens(tokens),
        }
    }
}

impl ToTokens for ClassAttrs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut first = true;
        let mut add_comma = |tokens: &mut proc_macro2::TokenStream| {
            if !first {
                tokens.append_all(quote! { , });
            }
            first = false;
        };

        if let Some(kw) = &self.kw_abstract {
            add_comma(tokens);
            kw.to_tokens(tokens);
        }
        if let Some(kw) = &self.kw_final {
            add_comma(tokens);
            kw.to_tokens(tokens);
        }
        if let Some(ext) = &self.extends {
            add_comma(tokens);
            ext.to_tokens(tokens);
        }
        if let Some(imp) = &self.implements {
            add_comma(tokens);
            imp.to_tokens(tokens);
        }
        if let Some(o) = &self.ons {
            add_comma(tokens);
            o.to_tokens(tokens);
        }
        if let Some(w) = &self.withs {
            add_comma(tokens);
            w.to_tokens(tokens);
        }
    }
}

impl ToTokens for ClassItems {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(&self.fields);
        tokens.append_all(&self.ctors);
        self.methods.to_tokens(tokens);
        tokens.append_all(&self.functions);
        tokens.append_all(&self.consts);
    }
}

impl<Kw: ToTokens, Classes: ToTokens> ToTokens for ClassRefs<Kw, Classes> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.kw.to_tokens(tokens);
        self._paren.surround(tokens, |tokens| {
            self.classes.to_tokens(tokens);
        });
    }
}
