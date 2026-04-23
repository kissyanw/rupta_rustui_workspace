use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

use super::field::Field;
use super::method::{Ctor, Method, Methods, OverridenMethod};
use crate::errors::ParseError;
use crate::syntax::method::ImplItemFn;

mod kw_class {
    syn::custom_keyword!(class);
    syn::custom_keyword!(interface);
    syn::custom_keyword!(mixin);

    syn::custom_keyword!(extends);
    syn::custom_keyword!(implements);
    syn::custom_keyword!(on);
    syn::custom_keyword!(with);
}

pub struct Class {
    #[allow(dead_code)]
    pub attrs: Vec<syn::Attribute>,
    pub class_attrs: ClassAttrs,
    pub vis: syn::Visibility,
    pub(super) _tk_type: syn::Token![type],
    pub name: syn::Ident,
    #[allow(dead_code)]
    pub generics: syn::Generics,
    pub(super) _tk_eq: syn::Token![=],
    pub kind: ClassKind,
    pub(super) _tk_lt: syn::Token![<],
    #[allow(dead_code)]
    pub brace: syn::token::Brace,
    pub items: ClassItems,
    pub(super) _tk_gt: syn::Token![>],
    pub(super) _tk_semi: syn::Token![;],
}

pub enum ClassKind {
    Class(#[allow(dead_code)] kw_class::class),
    Interface(#[allow(dead_code)] kw_class::interface),
    Mixin(#[allow(dead_code)] kw_class::mixin),
}

impl ClassKind {
    pub fn is_class(&self) -> bool {
        matches!(self, ClassKind::Class(_))
    }
    #[allow(dead_code)]
    pub fn is_interface(&self) -> bool {
        matches!(self, ClassKind::Interface(_))
    }
    pub fn is_mixin(&self) -> bool {
        matches!(self, ClassKind::Mixin(_))
    }
}

#[derive(Default)]
pub struct ClassAttrs {
    pub kw_abstract: Option<syn::Token![abstract]>,
    pub kw_final: Option<syn::Token![final]>,
    pub extends: Option<Extends>,
    pub withs: Option<Withs>,
    pub ons: Option<Ons>,
    pub implements: Option<Implements>,
}

pub struct ClassItems {
    pub fields: Vec<Field>,
    pub ctors: Vec<Ctor>,
    pub methods: Methods,
    pub functions: Vec<syn::ImplItemFn>,
    pub consts: Vec<syn::ImplItemConst>,
}

enum ClassItemKind {
    Ctor(Ctor),
    Function(syn::ImplItemFn),
    Method(Method),
}

// Represents a class reference that may be marked with #[mixin]
#[derive(Clone)]
pub struct ClassRef {
    pub is_mixin: bool,
    pub path: syn::Path,
}

impl Parse for ClassRef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Check for #[mixin] attribute
        let is_mixin = if input.peek(syn::Token![#]) {
            let _hash = input.parse::<syn::Token![#]>()?;
            let content;
            syn::bracketed!(content in input);
            let _mixin_kw = content.parse::<kw_class::mixin>()?;
            true
        } else {
            false
        };

        let path = input.parse()?;
        Ok(ClassRef { is_mixin, path })
    }
}

impl quote::ToTokens for ClassRef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.path.to_tokens(tokens);
    }
}

pub struct ClassRefs<Kw, Classes> {
    pub kw: Kw,
    pub(super) _paren: syn::token::Paren,
    pub classes: Classes,
}

pub type Extends = ClassRefs<kw_class::extends, syn::Path>;
pub type Implements = ClassRefs<kw_class::implements, Punctuated<syn::Path, syn::Token![,]>>;
pub type Ons = ClassRefs<kw_class::on, Punctuated<ClassRef, syn::Token![,]>>;
pub type Withs = ClassRefs<kw_class::with, Punctuated<syn::Path, syn::Token![,]>>;

impl Class {
    fn has_implements(&self, interface_name: &str) -> bool {
        // Check implements($interface)
        let has_implements = self
            .class_attrs
            .implements
            .as_ref()
            .is_some_and(|implements| {
                implements
                    .classes
                    .iter()
                    .any(|p| p.is_ident(interface_name))
            });
        // Check #[method(override($interface))]
        let has_overrides = self
            .items
            .methods
            .overrides
            .iter()
            .any(|m| m.r#override.classes.iter().any(|c| c == interface_name));
        has_implements || has_overrides
    }
    pub fn has_eq_hash(&self) -> bool {
        self.has_implements("EqHash")
    }

    pub fn has_format(&self) -> bool {
        self.has_implements("Format")
    }
    pub fn is_abstract(&self) -> bool {
        !self.kind.is_class() || self.class_attrs.kw_abstract.is_some()
    }

    #[cfg(test)]
    pub(crate) fn extract_class_attrs(&mut self) {
        let attr = self
            .attrs
            .extract_if(.., |attr| attr.path().is_ident("class"))
            .next()
            .expect("class attribute not found");
        match attr.meta {
            syn::Meta::Path(_) => {}
            syn::Meta::List(meta_list) => {
                self.class_attrs = meta_list
                    .parse_args()
                    .expect("failed to parse class attributes")
            }
            syn::Meta::NameValue(_) => panic!("class attribute must be a path or a list"),
        }
    }
}

// Parse implementations

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;

        // class_attrs will be populated later from attribute macro parameters
        let class_attrs = ClassAttrs::default();

        let vis = input.parse()?;
        let _tk_type = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;
        let _tk_eq = input.parse()?;
        let kind = input.parse()?;
        let _tk_lt = input.parse()?;

        let content;
        let brace = syn::braced!(content in input);
        let items = content.parse()?;

        let _: Option<syn::Token![,]> = input.parse()?;
        let _tk_gt = input.parse()?;
        let _tk_semi = input.parse()?;

        Ok(Class {
            attrs,
            class_attrs,
            vis,
            _tk_type,
            name,
            generics,
            _tk_eq,
            kind,
            _tk_lt,
            brace,
            items,
            _tk_gt,
            _tk_semi,
        })
    }
}

impl Parse for ClassKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw_class::class) {
            Ok(ClassKind::Class(input.parse()?))
        } else if lookahead.peek(kw_class::interface) {
            Ok(ClassKind::Interface(input.parse()?))
        } else if lookahead.peek(kw_class::mixin) {
            Ok(ClassKind::Mixin(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ClassAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse modifiers like abstract, final, extends, implements, on, with
        let mut kw_abstract = None;
        let mut kw_final = None;
        let mut extends = None;
        let mut implements = None;
        let mut on = None;
        let mut with = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(syn::Token![abstract]) {
                kw_abstract = Some(input.parse()?);
            } else if lookahead.peek(syn::Token![final]) {
                kw_final = Some(input.parse()?);
            } else if lookahead.peek(kw_class::extends) {
                extends = Some(input.parse()?);
            } else if lookahead.peek(kw_class::implements) {
                implements = Some(input.parse()?);
            } else if lookahead.peek(kw_class::on) {
                on = Some(input.parse()?);
            } else if lookahead.peek(kw_class::with) {
                with = Some(input.parse()?);
            } else {
                break;
            }

            // Parse optional comma
            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }

        Ok(ClassAttrs {
            kw_abstract,
            kw_final,
            extends,
            implements,
            ons: on,
            withs: with,
        })
    }
}

// Specialized Parse implementations for ClassRefs variants
impl Parse for Extends {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kw = input.parse()?;
        let content;
        let paren = syn::parenthesized!(content in input);
        let classes = content.parse()?;
        Ok(ClassRefs {
            kw,
            _paren: paren,
            classes,
        })
    }
}

impl Parse for Implements {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kw = input.parse()?;
        let content;
        let paren = syn::parenthesized!(content in input);
        let classes = Punctuated::parse_terminated(&content)?;
        Ok(ClassRefs {
            kw,
            _paren: paren,
            classes,
        })
    }
}

impl Implements {
    pub fn get_downcast(&self) -> Option<&syn::Path> {
        self.classes.iter().find(|path| path.is_ident("Downcast"))
    }
}

impl Parse for Ons {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kw = input.parse()?;
        let content;
        let paren = syn::parenthesized!(content in input);
        let classes = Punctuated::parse_terminated(&content)?;
        Ok(ClassRefs {
            kw,
            _paren: paren,
            classes,
        })
    }
}

impl Parse for Withs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kw = input.parse()?;
        let content;
        let paren = syn::parenthesized!(content in input);
        let classes = Punctuated::parse_terminated(&content)?;
        Ok(ClassRefs {
            kw,
            _paren: paren,
            classes,
        })
    }
}

impl Parse for ClassItems {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        let mut ctors = Vec::new();
        let mut methods = Methods {
            finals: Vec::new(),
            owns: Vec::new(),
            overrides: Vec::new(),
        };
        let mut functions = Vec::new();
        let mut consts = Vec::new();

        while !input.is_empty() {
            let attrs = input.call(syn::Attribute::parse_outer)?;

            // Check for field (let ...)
            if input.peek(syn::Token![let]) {
                let mut field: Field = input.parse()?;
                field.attrs = attrs;
                field.extract_field_attrs()?;
                fields.push(field);
                continue;
            }

            // Check for const
            let vis = input.parse()?;
            let fork = input.fork();
            if fork.peek(syn::Token![const]) && !fork.peek2(syn::Token![fn]) {
                let mut item_const: syn::ImplItemConst = input.parse()?;
                item_const.attrs = attrs;
                item_const.vis = vis;
                consts.push(item_const);
                continue;
            }

            // Try to parse as function (ctor, method, or regular function)
            if fork.peek(syn::Token![fn])
                || fork.peek(syn::Token![pub])
                || fork.peek(syn::Token![const])
                || fork.peek(syn::Token![async])
                || fork.peek(syn::Token![unsafe])
                || fork.peek(syn::Token![extern])
            {
                // Try to parse as ItemFn first (with body)
                let fork_for_item_fn = input.fork();
                if let Ok(mut item_fn) = fork_for_item_fn.parse::<ImplItemFn>() {
                    // Parsing succeeded, advance the actual input
                    input.advance_to(&fork_for_item_fn);
                    item_fn.attrs = attrs;
                    item_fn.vis = vis;

                    // Try to convert to ClassItemKind
                    match ClassItemKind::try_from(item_fn)? {
                        ClassItemKind::Ctor(ctor) => ctors.push(ctor),
                        ClassItemKind::Function(func) => functions.push(func),
                        ClassItemKind::Method(mut method) => {
                            // Extract method modifiers (final and override)
                            let modifiers = method.extract_method_modifiers()?;

                            // Set the final token if method is final
                            method.tk_final = modifiers.tk_final;

                            if let Some(override_info) = modifiers.override_info {
                                // This is an override method (may also be final)
                                methods.overrides.push(OverridenMethod {
                                    r#override: override_info,
                                    method,
                                });
                            } else if method.tk_final.is_some() {
                                // This is a final method (but not override)
                                methods.finals.push(method);
                            } else {
                                // This is a regular own method
                                methods.owns.push(method);
                            }
                        }
                    }
                    continue;
                }
            }

            return Err(syn::Error::new(input.span(), ParseError::ExpectedClassItem));
        }

        Ok(ClassItems {
            fields,
            ctors,
            methods,
            functions,
            consts,
        })
    }
}

impl TryFrom<ImplItemFn> for ClassItemKind {
    type Error = syn::Error;

    fn try_from(item_fn: ImplItemFn) -> syn::Result<Self> {
        // Check if it's a constructor (has `-> Self` return type)
        if let syn::ReturnType::Type(_, ref ty) = item_fn.sig.output
            && let syn::Type::Path(type_path) = ty.as_ref()
            && type_path.path.is_ident("Self")
        {
            return Ctor::try_from(item_fn).map(ClassItemKind::Ctor);
        }

        // Check if it's a method (has `self` receiver)
        if let Some(syn::FnArg::Receiver(_)) = item_fn.sig.inputs.first() {
            return Method::try_from(item_fn).map(ClassItemKind::Method);
        }

        // Otherwise it's a regular function
        Ok(ClassItemKind::Function(item_fn.try_into()?))
    }
}

impl ClassAttrs {
    /// Check if ClassAttrs has any attributes set
    #[cfg(test)]
    pub(super) fn is_empty(&self) -> bool {
        self.kw_abstract.is_none()
            && self.kw_final.is_none()
            && self.extends.is_none()
            && self.implements.is_none()
            && self.ons.is_none()
            && self.withs.is_none()
    }
}
