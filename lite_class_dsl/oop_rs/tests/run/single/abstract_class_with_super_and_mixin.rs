use oop_rs::prelude::*;

#[class(implements(EqHash, Format))]
pub type GestureArenaMember = interface<{}>;

#[class(abstract, implements(GestureArenaMember, Format))]
pub type GestureRecognizer = class<
    {
        pub fn new() -> Self {
            Self {}
        }
    },
>;

#[class(abstract, extends(GestureRecognizer))]
pub type OneSequenceGestureRecognizer = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
    },
>;

#[class(on(GestureArenaMember, GestureRecognizer, OneSequenceGestureRecognizer))]
type TapStatusTrackerMixin = mixin<{}>;

#[class(
    abstract,
    extends(OneSequenceGestureRecognizer),
    with(TapStatusTrackerMixin)
)]
pub type BaseTapAndDragGestureRecognizer = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
    },
>;

#[class(extends(BaseTapAndDragGestureRecognizer))]
pub type TapAndDragGestureRecognizer = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
    },
>;

#[test]
fn test_abstract_class_with_super_and_mixin() {
    let recognizer = TapAndDragGestureRecognizer::new();
    assert_eq!(
        format!("{:?}", &*recognizer as &GestureArenaMember),
        format!("{:p}", recognizer),
    );
}
