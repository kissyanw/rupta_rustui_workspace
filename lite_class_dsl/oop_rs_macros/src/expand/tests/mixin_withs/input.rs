#[class]
type A = class<
    {
        pub fn new() -> Self {
            Self {}
        }
        pub fn f(&self) {
            println!("A::f");
        }
    },
>;

#[class(on(A))]
type M1 = mixin<
    {
        #[method(override(A))]
        pub fn f(&self) {
            super.f();
            println!("M1::f");
        }
    },
>;

#[class(on(A))]
type M2 = mixin<
    {
        #[method(override(A))]
        pub fn f(&self) {
            super.f();
            println!("M2::f");
        }
    },
>;

#[class(extends(A), with(M1, M2))]
type B = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
        #[method(override(A))]
        pub fn f(&self) {
            super.f();
            println!("B::f");
        }
    },
>;
