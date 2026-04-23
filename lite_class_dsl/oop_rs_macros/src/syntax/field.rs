use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

mod kw {
    syn::custom_keyword!(late);
    syn::custom_keyword!(vis);
}

pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub kw_late: Option<kw::late>,
    pub vis: syn::Visibility,
    pub(super) _tk_let: syn::Token![let],
    pub tk_ref: Option<syn::Token![ref]>,
    pub tk_mut: Option<syn::Token![mut]>,
    pub field: syn::Ident,
    pub tk_colon: syn::Token![:],
    pub ty: syn::Type,
    pub init: Option<FieldInit>,
    pub(super) _tk_semi: syn::Token![;],
}

pub struct FieldInit {
    pub(super) _tk_eq: syn::Token![=],
    pub expr: syn::Expr,
}

// Parse implementations

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Vec::new(); // Parsed by `ClassItems`
        let kw_late = None;
        let vis = syn::Visibility::Inherited;

        // Note: attrs are parsed by `ClassItems` and passed via `field.attrs`
        // But we need to extract the `#[late]` and `#[vis]` attributes.
        // This will be done after parsing in `ClassItems::parse`.

        let _tk_let = input.parse()?;
        let tk_ref = input.parse()?;
        let tk_mut = input.parse()?;
        let field = input.parse()?;
        let tk_colon = input.parse()?;
        let ty = input.parse()?;
        let init = if input.peek(syn::Token![=]) {
            Some(input.parse()?)
        } else {
            None
        };
        let _tk_semi = input.parse()?;

        Ok(Field {
            attrs,
            kw_late,
            vis,
            _tk_let,
            tk_ref,
            tk_mut,
            field,
            tk_colon,
            ty,
            init,
            _tk_semi,
        })
    }
}

impl Field {
    /// Extract and parse `#[late]` and `#[vis]` attributes from attrs
    pub fn extract_field_attrs(&mut self) -> syn::Result<()> {
        for attr in self.attrs.extract_if(.., |attr| {
            attr.path()
                .get_ident()
                .is_some_and(|ident| ident == "late" || ident == "vis")
        }) {
            match syn::parse2::<FieldAttr>(attr.meta.into_token_stream())? {
                FieldAttr::Late(late) => self.kw_late = Some(late),
                FieldAttr::Vis(vis) => self.vis = vis,
            }
        }
        Ok(())
    }
}

impl Parse for FieldInit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(FieldInit {
            _tk_eq: input.parse()?,
            expr: input.parse()?,
        })
    }
}

enum FieldAttr {
    Late(kw::late),
    Vis(syn::Visibility),
}

impl Parse for FieldAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::late) {
            Ok(FieldAttr::Late(input.parse()?))
        } else if lookahead.peek(kw::vis) {
            _ = input.parse::<kw::vis>()?;
            let content;
            syn::parenthesized!(content in input);
            Ok(FieldAttr::Vis(content.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
