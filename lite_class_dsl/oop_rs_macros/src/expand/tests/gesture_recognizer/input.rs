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
