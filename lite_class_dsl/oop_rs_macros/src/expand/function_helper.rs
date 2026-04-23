use crate::{
    expand::{ExpandCtxt, constants::*},
    syntax::{
        class::Class,
        method::{Ctor, Method},
    },
};
use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{punctuated::Punctuated, visit_mut::VisitMut};

pub const FN_HELPER_KEY: &str = "helper";
pub const FN_CTOR_HELPER_KEY: &str = "builder";

// ─────────────────────────────────────────────────────────────────────────────
// Local data types
// ─────────────────────────────────────────────────────────────────────────────

pub struct Param {
    pub name: syn::Ident,
    pub ty: syn::Type,
    pub default_value: Option<syn::Expr>,
}

pub enum FnEntryKind {
    Method(syn::ReturnType),
    Fn(syn::ReturnType),
    Ctor,
}

pub struct FnEntry<'a> {
    pub name: &'a syn::Ident,
    pub attrs: &'a [syn::Attribute],
    pub params: Vec<Param>,
    pub kind: FnEntryKind,
}

fn extract_params(inputs: &Punctuated<syn::PatType, syn::Token![,]>) -> Vec<Param> {
    inputs
        .iter()
        .map(|pat_type| {
            let name = match &*pat_type.pat {
                syn::Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                _ => panic!("expected ident pattern in builder/helper param"),
            };
            let default_value = pat_type.attrs.iter().find_map(|attr| {
                if !attr.path().is_ident("default") {
                    return None;
                }
                if let syn::Meta::List(list) = &attr.meta {
                    Some(
                        list.parse_args::<syn::Expr>()
                            .expect("expected expression in #[default(...)]"),
                    )
                } else {
                    None
                }
            });
            let ty = (*pat_type.ty).clone();
            Param {
                name,
                ty,
                default_value,
            }
        })
        .collect()
}

fn ctor_fn_entry<'a>(ctor: &'a Ctor) -> Option<FnEntry<'a>> {
    if !ctor
        .attrs
        .iter()
        .any(|a| a.path().is_ident(FN_CTOR_HELPER_KEY))
    {
        return None;
    }
    let params = extract_params(&ctor.inputs);
    if params.iter().all(|p| p.default_value.is_none()) {
        return None;
    }
    Some(FnEntry {
        name: &ctor.ident,
        attrs: &ctor.attrs,
        params,
        kind: FnEntryKind::Ctor,
    })
}

fn fn_entry<'a>(r#fn: &'a syn::ImplItemFn) -> Option<FnEntry<'a>> {
    if !r#fn.attrs.iter().any(|a| a.path().is_ident(FN_HELPER_KEY)) {
        return None;
    }
    let params = extract_params(
        &r#fn
            .sig
            .inputs
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Typed(pat_type) => pat_type,
                syn::FnArg::Receiver(_) => panic!("expected typed argument in helper"),
            })
            .cloned()
            .collect(),
    );
    if params.iter().all(|p| p.default_value.is_none()) {
        return None;
    }
    Some(FnEntry {
        name: &r#fn.sig.ident,
        attrs: &r#fn.attrs,
        params,
        kind: FnEntryKind::Fn(r#fn.sig.output.clone()),
    })
}

fn method_fn_entry<'a>(method: &'a Method) -> Option<FnEntry<'a>> {
    if !method
        .attrs
        .iter()
        .any(|a| a.path().is_ident(FN_HELPER_KEY))
    {
        return None;
    }
    let params = extract_params(&method.inputs);
    if params.iter().all(|p| p.default_value.is_none()) {
        return None;
    }
    Some(FnEntry {
        name: &method.ident,
        attrs: &method.attrs,
        params,
        kind: FnEntryKind::Method(method.output.clone()),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Public entry points
// ─────────────────────────────────────────────────────────────────────────────

impl Class {
    /// Generates helper structs for methods with `#[helper]` and external (user-facing)
    /// builders for ctors with `#[builder]`.
    pub(super) fn expand_helper_impls<'a>(
        &'a self,
        cx: &'a ExpandCtxt<'a>,
    ) -> impl Iterator<Item = ExpandHelperImpl<'a>> + use<'a> {
        let method_helpers = self
            .items
            .methods
            .owns
            .iter()
            .filter_map(|m| ExpandHelperImpl::from_method(self, m));

        let ctor_helpers = self
            .items
            .ctors
            .iter()
            .filter(|_| !cx.class.is_abstract())
            .filter_map(|c| ExpandHelperImpl::from_ctor(self, c));

        let fn_helpers = self
            .items
            .functions
            .iter()
            .filter_map(|f| ExpandHelperImpl::from_fn(self, f));

        method_helpers.chain(ctor_helpers).chain(fn_helpers)
    }

    /// Generates internal builders for ctors with `#[builder]`, called from subclass
    /// ctor bodies via `Super::builder_method(__self).build()`.
    ///
    /// These are on `impl<__Self: ClassConcrete> __S{ClassName}<__Self>`.
    pub(super) fn expand_ctor_data_helpers<'a>(
        &'a self,
        cx: &'a ExpandCtxt<'a>,
    ) -> impl Iterator<Item = ExpandCtorDataHelper<'a>> + use<'a> {
        self.items
            .ctors
            .iter()
            .filter_map(move |c| ExpandCtorDataHelper::new(self, cx, c))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper name derivation
// ─────────────────────────────────────────────────────────────────────────────

fn helper_name(ty: &Class, entry: &FnEntry) -> (syn::Ident, syn::Ident) {
    let try_extract_name = |attr: &syn::Attribute, name: &str| -> Option<syn::Ident> {
        let named_value = attr.meta.require_name_value().ok()?;
        if named_value.path.is_ident(name) {
            let syn::Expr::Lit(syn::ExprLit { lit, .. }) = &named_value.value else {
                panic!("Expected literal expression for attribute `{}`", name)
            };
            let syn::Lit::Str(s) = lit else {
                panic!("Expected string literal for attribute `{}`", name)
            };
            Some(format_ident!("{}", s.value(), span = s.span()))
        } else {
            None
        }
    };
    let fn_suffix = match entry.kind {
        FnEntryKind::Ctor => FN_CTOR_HELPER_KEY,
        FnEntryKind::Fn { .. } | FnEntryKind::Method { .. } => FN_HELPER_KEY,
    };
    let fn_name = entry
        .attrs
        .iter()
        .find_map(|attr| try_extract_name(attr, fn_suffix))
        .unwrap_or_else(|| {
            if entry.name == "new" {
                format_ident!("builder", span = entry.name.span())
            } else {
                format_ident!("{}_{fn_suffix}", entry.name)
            }
        });

    let helper_ty = format_ident!(
        "{}{}",
        ty.name,
        fn_name.to_string().to_case(Case::Pascal),
        span = fn_name.span()
    );
    (helper_ty, fn_name)
}

// ─────────────────────────────────────────────────────────────────────────────
// External helpers (methods + external ctor builders)
// ─────────────────────────────────────────────────────────────────────────────

/// Generates a helper struct and its constructor impl for a method helper or
/// an external (user-facing) ctor builder.
pub struct ExpandHelperImpl<'a> {
    class: &'a syn::Ident,
    entry: FnEntry<'a>,
    helper_ty: syn::Ident,
    helper_new_fn: syn::Ident,
}

impl<'a> ExpandHelperImpl<'a> {
    fn from_method(ty: &'a Class, method: &'a Method) -> Option<Self> {
        let entry = method_fn_entry(method)?;
        let (helper_ty, helper_new_fn) = helper_name(ty, &entry);
        Some(Self {
            class: &ty.name,
            entry,
            helper_ty,
            helper_new_fn,
        })
    }

    fn from_ctor(ty: &'a Class, ctor: &'a Ctor) -> Option<Self> {
        let entry = ctor_fn_entry(ctor)?;
        let (helper_ty, helper_new_fn) = helper_name(ty, &entry);
        Some(Self {
            class: &ty.name,
            entry,
            helper_ty,
            helper_new_fn,
        })
    }

    fn from_fn(ty: &'a Class, r#fn: &'a syn::ImplItemFn) -> Option<Self> {
        let entry = fn_entry(r#fn)?;
        let (helper_ty, helper_new_fn) = helper_name(ty, &entry);
        Some(Self {
            class: &ty.name,
            entry,
            helper_ty,
            helper_new_fn,
        })
    }

    /// Generate the helper struct and its `call()`/`build()` + setter methods.
    fn helper(&self) -> TokenStream {
        let Self {
            class,
            helper_ty,
            entry,
            ..
        } = self;
        let mut lifetime_visitor = LifetimeReplacer::new();
        let (mut struct_fields, mod_fns): (Vec<_>, Vec<_>) = entry
            .params
            .iter()
            .map(|param| {
                let Param { name, ty, .. } = param;
                let mut ty = ty.clone();
                lifetime_visitor.try_replace_lifetime(&mut ty);
                let struct_field = quote! { #name: #ty };
                let modify_fn = quote! {
                    pub fn #name(self, #name: #ty) -> Self {
                        Self { #name, ..self }
                    }
                };
                (struct_field, modify_fn)
            })
            .unzip();

        let helper_fn_impl = self.helper_fn_impl();

        if matches!(entry.kind, FnEntryKind::Method { .. }) {
            // Method helpers hold a `CRc<Class>` to call the method on.
            struct_fields.insert(0, quote! { __self: #_CRc<#class> });
        }
        // External ctor builders have no `__self` – they call the public ctor API.

        let lifetime_params = lifetime_visitor
            .generate_lifetime_params()
            .map_or(quote!(), |lp| quote! { #lp });

        quote! {
            #[must_use]
            pub struct #helper_ty #lifetime_params {
                #(#struct_fields),*
            }

            impl #lifetime_params #helper_ty #lifetime_params {
                #(#mod_fns)*
                #helper_fn_impl
            }
        }
    }

    fn helper_fn_impl(&self) -> TokenStream {
        let Self { class, entry, .. } = self;

        let fields = entry
            .params
            .iter()
            .map(|param| {
                let Param { name, .. } = param;
                quote! { #name }
            })
            .collect::<Vec<_>>();
        let fn_name = entry.name;
        let final_fn = match entry.kind {
            FnEntryKind::Ctor => syn::Ident::new("build", fn_name.span()),
            FnEntryKind::Fn { .. } | FnEntryKind::Method { .. } => {
                syn::Ident::new("call", fn_name.span())
            }
        };

        match &entry.kind {
            FnEntryKind::Method(ret) => quote! {
                pub fn #final_fn(self) #ret {
                    let Self { __self, #(#fields),* } = self;
                    __self.#fn_name(#(#fields),*)
                }
            },
            // External builder: call the public ctor (defined on `impl ClassName`)
            // which is available in the same module as `type __Self = ...`.
            FnEntryKind::Ctor => quote! {
                pub fn #final_fn(self) -> #_CRc<__Self> {
                    let Self { #(#fields),* } = self;
                    #class::#fn_name(#(#fields),*)
                }
            },
            FnEntryKind::Fn(ret) => quote! {
                pub fn #final_fn(self) #ret {
                    let Self { #(#fields),* } = self;
                    #class::#fn_name(#(#fields),*)
                }
            },
        }
    }

    /// Generate the `impl ClassName { fn helper_new_fn(...) -> HelperType { ... } }` block.
    fn class_helper_impl(&self) -> TokenStream {
        let Self {
            class,
            helper_ty,
            helper_new_fn,
            entry,
        } = self;
        let mut lifetime_visitor = LifetimeReplacer::new();
        let required_params = entry
            .params
            .iter()
            .filter_map(|param| {
                let Param { name, ty, .. } = param;
                let mut ty = ty.clone();
                lifetime_visitor.try_replace_lifetime(&mut ty);
                param.default_value.is_none().then(|| quote! { #name: #ty })
            })
            .collect::<Vec<_>>();
        let init_fields = entry.params.iter().map(|param| {
            let Param {
                name,
                default_value,
                ..
            } = param;
            if let Some(default_value) = default_value.as_ref() {
                quote! { #name: #default_value }
            } else {
                quote! { #name }
            }
        });
        let lifetime_params = lifetime_visitor
            .generate_lifetime_params()
            .map_or(quote!(), |lp| quote! { #lp });

        match entry.kind {
            // Method helper: called as `self.helper_name(required_args)`
            FnEntryKind::Method(_) => quote! {
                impl #class {
                    pub fn #helper_new_fn #lifetime_params (&self, #(#required_params),*) -> #helper_ty #lifetime_params {
                        #helper_ty {
                            __self: #_crate::rc::RcRefImpl::to_rc(self),
                            #(#init_fields),*
                        }
                    }
                }
            },
            // External ctor builder: called as `ClassName::builder_name(required_args)`
            FnEntryKind::Ctor | FnEntryKind::Fn(_) => quote! {
                impl #class {
                    pub fn #helper_new_fn #lifetime_params (#(#required_params),*) -> #helper_ty #lifetime_params {
                        #helper_ty {
                            #(#init_fields),*
                        }
                    }
                }
            },
        }
    }
}

impl ToTokens for ExpandHelperImpl<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let helper_impl = self.helper();
        let class_helper_impl = self.class_helper_impl();
        tokens.extend(quote! {
            #helper_impl
            #class_helper_impl
        });
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal ctor builders (CTOR_OF_DATA)
//
// These are called from subclass ctor bodies:
//   `..Super::builder_method(__self).build()`
// The `__self: CRcUninit<__Self, CClass<ClassName, __Self>>` is automatically
// inserted by the ctor body expansion.
// ─────────────────────────────────────────────────────────────────────────────

/// Generates an internal (data-level) ctor builder generic over `__Self: ClassConcrete`.
pub struct ExpandCtorDataHelper<'a> {
    #[allow(dead_code)]
    class: &'a syn::Ident,
    class_impl: syn::Ident,
    class_impl_bounds: Punctuated<syn::Path, syn::Token![+]>,
    entry: FnEntry<'a>,
    /// Helper struct name, e.g. `__TestSuperTwoWBuilder`
    helper_ty: syn::Ident,
    helper_new_fn: syn::Ident,
}

impl<'a> ExpandCtorDataHelper<'a> {
    fn new(ty: &'a Class, cx: &ExpandCtxt<'_>, ctor: &'a Ctor) -> Option<Self> {
        let entry = ctor_fn_entry(ctor)?;
        let (external_ty, helper_new_fn) = helper_name(ty, &entry);
        // Use a distinct name to avoid conflict with the external builder struct.
        let helper_ty = format_ident!("__D{}", external_ty);
        let class_impl = cx.class_impl_name();
        let class_impl_bounds = cx.class.class_attrs.expand_concrete_with_trait_bounds(cx);
        Some(Self {
            class: &ty.name,
            class_impl,
            class_impl_bounds,
            entry,
            helper_ty,
            helper_new_fn,
        })
    }

    fn helper(&self) -> TokenStream {
        let Self {
            class_impl,
            class_impl_bounds,
            helper_ty,
            entry,
            ..
        } = self;
        let (struct_fields, mod_fns): (Vec<_>, Vec<_>) = entry
            .params
            .iter()
            .map(|param| {
                let Param { name, ty, .. } = param;
                let struct_field = quote! { #name: #ty };
                let modify_fn = quote! {
                    pub fn #name(self, #name: #ty) -> Self {
                        Self { #name, ..self }
                    }
                };
                (struct_field, modify_fn)
            })
            .unzip();

        let helper_fn_impl = self.helper_fn_impl();

        quote! {
            #[must_use]
            pub struct #helper_ty<__Self: #class_impl_bounds> {
                __self: #_CRcUninit<__Self, #class_impl<__Self>>,
                #(#struct_fields),*
            }

            impl<__Self: #class_impl_bounds> #helper_ty<__Self> {
                #(#mod_fns)*
                #helper_fn_impl
            }
        }
    }

    fn helper_fn_impl(&self) -> TokenStream {
        let Self {
            class_impl, entry, ..
        } = self;
        let fields = entry.params.iter().map(|p| &p.name).collect::<Vec<_>>();
        let fn_name = entry.name;
        quote! {
            pub fn build(self) -> #_CRc<__Self> {
                let Self { __self, #(#fields),* } = self;
                #class_impl::<__Self>::#fn_name(__self, #(#fields),*)
            }
        }
    }

    fn class_helper_impl(&self) -> TokenStream {
        let Self {
            class_impl,
            class_impl_bounds,
            helper_ty,
            helper_new_fn,
            entry,
            ..
        } = self;
        let required_params = entry
            .params
            .iter()
            .filter_map(|param| {
                let Param { name, ty, .. } = param;
                param.default_value.is_none().then(|| quote! { #name: #ty })
            })
            .collect::<Vec<_>>();
        let init_fields = entry.params.iter().map(|param| {
            let Param {
                name,
                default_value,
                ..
            } = param;
            if let Some(default_value) = default_value.as_ref() {
                quote! { #name: #default_value }
            } else {
                quote! { #name }
            }
        });

        quote! {
            impl<__Self: #class_impl_bounds> #class_impl<__Self> {
                pub fn #helper_new_fn(
                    __self: #_CRcUninit<__Self, #class_impl<__Self>>,
                    #(#required_params),*
                ) -> #helper_ty<__Self> {
                    #helper_ty {
                        __self,
                        #(#init_fields),*
                    }
                }
            }
        }
    }
}

impl ToTokens for ExpandCtorDataHelper<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let helper = self.helper();
        let class_helper = self.class_helper_impl();
        tokens.extend(quote! {
            #helper
            #class_helper
        });
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Lifetime handling
// ─────────────────────────────────────────────────────────────────────────────

struct LifetimeReplacer {
    next: usize,
    lifetimes: Vec<syn::Lifetime>,
    generic_param_depth: u32,
}
impl LifetimeReplacer {
    fn new() -> Self {
        Self {
            next: 1,
            lifetimes: Vec::new(),
            generic_param_depth: 0,
        }
    }

    fn new_lifetime(&mut self) -> syn::Lifetime {
        let lifetime = syn::Lifetime::new(&format!("'__{}", self.next), Span::call_site());
        self.lifetimes.push(lifetime.clone());
        self.next += 1;
        lifetime
    }

    fn try_replace_lifetime(&mut self, ty: &mut syn::Type) {
        self.visit_type_mut(ty);
    }

    fn generate_lifetime_params(self) -> Option<syn::PathArguments> {
        if self.lifetimes.is_empty() {
            return None;
        }
        let lifetimes = self
            .lifetimes
            .into_iter()
            .map(syn::GenericArgument::Lifetime);
        let generic_args = syn::parse_quote! {
            <#(#lifetimes),*>
        };
        Some(syn::PathArguments::AngleBracketed(generic_args))
    }
}

impl VisitMut for LifetimeReplacer {
    fn visit_fn_arg_mut(&mut self, _: &mut syn::FnArg) {
        // Don't change function types
    }
    fn visit_item_fn_mut(&mut self, _: &mut syn::ItemFn) {
        // Don't change function types
    }
    fn visit_type_bare_fn_mut(&mut self, _: &mut syn::TypeBareFn) {
        // Don't change function types
    }
    fn visit_lifetime_mut(&mut self, i: &mut syn::Lifetime) {
        if i.ident == "_" {
            *i = self.new_lifetime();
        } else if i.ident != "static" {
            self.lifetimes.push(i.clone());
        }
    }
    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        i: &mut syn::ParenthesizedGenericArguments,
    ) {
        self.generic_param_depth += 1;
        syn::visit_mut::visit_parenthesized_generic_arguments_mut(self, i);
        self.generic_param_depth -= 1;
    }
    fn visit_type_reference_mut(&mut self, i: &mut syn::TypeReference) {
        if i.lifetime.is_none() && self.generic_param_depth == 0 {
            // Will be replaced in `visit_lifetime_mut`
            i.lifetime = Some(syn::Lifetime::new("'_", Span::call_site()));
        }
        syn::visit_mut::visit_type_reference_mut(self, i);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_replace_implicit_lifetime() {
        let mut visitor = LifetimeReplacer::new();
        macro_rules! test_case {
            ($visitor:expr, $item:ty, $expected:ty $(,)?) => {{
                let mut item: syn::Type = syn::parse_quote!($item);
                let expected: syn::Type = syn::parse_quote!($expected);
                $visitor.try_replace_lifetime(&mut item);
                assert_eq!(
                    item.to_token_stream().to_string(),
                    expected.to_token_stream().to_string(),
                );
            }};
        }
        test_case!(visitor, &i32, &'__1 i32);
        test_case!(visitor, Foo<'a, 'b>, Foo<'a, 'b>);
        test_case!(visitor, Foo<'_>, Foo<'__2>);
        test_case!(visitor, &Foo<'a, 'b>, &'__3 Foo<'a, 'b>);
        test_case!(visitor, fn(&i32) -> &i64, fn(&i32) -> &i64);
        test_case!(visitor, dyn Fn(&i32) -> &i64, dyn Fn(&i32) -> &i64);
        test_case!(
            visitor,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) -> &i64,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) -> &i64,
        );
        test_case!(visitor, &dyn Fn(&i32) -> &i64, &'__4 dyn Fn(&i32) -> &i64);
        test_case!(visitor, dyn Fn(&i32) + '_, dyn Fn(&i32) + '__5);
        test_case!(
            visitor,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) + '_,
            dyn Fn(&dyn Fn(&i32), &dyn Fn(&i32)) + '__6,
        );
    }
}
