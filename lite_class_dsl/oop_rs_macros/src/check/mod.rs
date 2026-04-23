use crate::syntax::Class;

mod class;

struct CheckCtxt<'a> {
    class: &'a Class,
    errors: Vec<syn::Error>,
}

impl<'a> CheckCtxt<'a> {
    fn new(class: &'a Class) -> Self {
        Self {
            class,
            errors: Vec::new(),
        }
    }

    fn into_errors(self) -> impl Iterator<Item = syn::Error> {
        self.errors.into_iter()
    }
}
