use oop_rs::prelude::*;

struct PointerEvent;

#[class]
pub type PointerRoute = interface<
    {
        fn handle_event(&self, event: &PointerEvent);
    },
>;

#[class(abstract, implements(PointerRoute))]
pub type OneSequenceGestureRecognizer = class<
    {
        #[method(override(PointerRoute))]
        fn handle_event(&self, event: &PointerEvent);
    },
>;
