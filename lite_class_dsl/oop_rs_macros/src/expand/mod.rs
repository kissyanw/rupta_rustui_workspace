use std::{cell::OnceCell, collections::HashMap};

use quote::format_ident;

use crate::syntax::Class;
use crate::syntax::field::Field;

#[macro_use]
mod utils;
mod class;
mod constants;
mod field;
mod function_helper;
mod method;
mod mixin;
mod trait_;
mod vtable;

#[cfg(test)]
mod tests;

struct ExpandCtxt<'a> {
    class: &'a Class,
    fields_map: OnceCell<HashMap<&'a syn::Ident, &'a Field>>,
}

impl<'a> ExpandCtxt<'a> {
    fn new(class: &'a Class) -> Self {
        Self {
            class,
            fields_map: OnceCell::new(),
        }
    }

    fn mod_name(&self) -> syn::Ident {
        format_ident!("__{}", self.class.name)
    }

    fn class_impl_name(&self) -> syn::Ident {
        format_ident!("__S{}", self.class.name)
    }

    fn trait_name(&self) -> syn::Ident {
        trait_name(&self.class.name)
    }

    fn data_name(&self) -> syn::Ident {
        format_ident!("__D{}", self.class.name)
    }

    fn vtable_name(&self) -> syn::Ident {
        format_ident!("__V{}", self.class.name)
    }

    fn instance_vtable_name(&self) -> syn::Ident {
        format_ident!("__VM{}", self.class.name)
    }

    fn concrete_name(&self) -> syn::Ident {
        format_ident!("__C{}", self.class.name)
    }

    fn accessor_name(&self) -> syn::Ident {
        format_ident!("__A{}", self.class.name)
    }

    fn fields_map(&self) -> &HashMap<&'a syn::Ident, &'a Field> {
        self.fields_map.get_or_init(|| {
            self.class
                .items
                .fields
                .iter()
                .map(|field| (&field.field, field))
                .collect()
        })
    }
    fn get_field(&self, ident: &syn::Ident) -> Option<&'a Field> {
        self.fields_map().get(ident).copied()
    }
}

fn trait_name(name: &syn::Ident) -> syn::Ident {
    format_ident!("I{}", name)
}

fn interface_field(name: &syn::Ident) -> syn::Ident {
    format_ident!("__{}", name)
}

trait PathExt {
    fn as_ident(&self) -> &syn::Ident;
    fn interface_field(&self) -> syn::Ident {
        interface_field(self.as_ident())
    }
}

impl PathExt for syn::Path {
    fn as_ident(&self) -> &syn::Ident {
        &self
            .segments
            .last()
            .expect("path must have at least one segment")
            .ident
    }
}

trait VisibilityExt {
    /// expand visibility with the depth of the relative module
    fn expand_for_super(&self) -> syn::Visibility;
}

impl VisibilityExt for syn::Visibility {
    fn expand_for_super(&self) -> syn::Visibility {
        use syn::Visibility::*;
        match self {
            &Public(_pub) => Public(_pub),
            Inherited => syn::parse_quote!(pub (super)),
            Restricted(syn::VisRestricted { path, .. })
                if path.leading_colon.is_some()
                    || path
                        .segments
                        .first()
                        .is_some_and(|seg| seg.ident == "crate") =>
            {
                self.clone()
            }
            Restricted(syn::VisRestricted { path, .. }) => {
                syn::parse_quote!(pub (in super::#path))
            }
        }
    }
}
