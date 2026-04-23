use proc_macro::TokenStream;

use syntax::{Class, ClassAttrs};

mod check;
mod errors;
mod expand;
mod syntax;

#[proc_macro_attribute]
pub fn class(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut class = syn::parse_macro_input!(input as Class);
    let class_attrs = syn::parse_macro_input!(attrs as ClassAttrs);
    class.class_attrs = class_attrs;
    let errors = class.check();
    let expanded = class.expand();
    quote::quote!(#expanded #errors).into()
}
