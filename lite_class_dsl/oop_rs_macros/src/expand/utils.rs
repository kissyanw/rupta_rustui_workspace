use proc_macro2::Span;
use syn::visit_mut::VisitMut;

pub(super) trait VisitMutBy<V: VisitMut> {
    fn visit_by(&mut self, vis: &mut V);
}

impl<V: VisitMut> VisitMutBy<V> for syn::Type {
    fn visit_by(&mut self, vis: &mut V) {
        vis.visit_type_mut(self);
    }
}

impl<V: VisitMut> VisitMutBy<V> for syn::Expr {
    fn visit_by(&mut self, vis: &mut V) {
        vis.visit_expr_mut(self);
    }
}

pub(super) fn keep_span<T: VisitMutBy<KeepSpan>>(span: Span, mut value: T) -> T {
    value.visit_by(&mut KeepSpan { span });
    value
}

pub(super) struct KeepSpan {
    span: Span,
}

impl VisitMut for KeepSpan {
    fn visit_span_mut(&mut self, span: &mut Span) {
        *span = span.located_at(self.span);
    }
}

macro_rules! parse_quote_spanned {
    ($span:expr=> $($tt:tt)*) => {
        $crate::expand::utils::keep_span($span, syn::parse_quote!($($tt)*))
    };
}
