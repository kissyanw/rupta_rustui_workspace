#[class]
type I = interface<
    {
        pub fn i(&self);
    },
>;

#[class]
type A = class<
    {
        pub fn new() -> Self {
            Self {}
        }
    },
>;

#[class(on(A), implements(I))]
type M = mixin<
    {
        #[method(override(I))]
        pub fn i(&self) {}
    },
>;

#[class(extends(A), with(M))]
type B = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
        #[method(override(I))]
        pub fn i(&self) {
            super.i();
        }
    },
>;
