#[class(extends(Object))]
type A = class<
    {
        fn new() -> Self {
            Self {}
        }

        fn f(&self) {
            println!("A::f");
        }
    },
>;

#[class(on(A))]
type M = mixin<
    {
        #[method(override(A))]
        fn f(&self) {
            println!("M::f");
            super.f();
        }
    },
>;

#[class(extends(A), with(M))]
type B = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(A))]
        fn f(&self) {
            println!("B::f");
            super.f();
        }
    },
>;
