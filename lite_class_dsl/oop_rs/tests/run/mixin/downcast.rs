use crate::BUF;
use oop_rs::prelude::*;

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

static EXPECTED: &[&str] = &[
    "B::f", "M::f", "A::f", // b.f()
    "B::f", "M::f", "A::f", // a.f()
    "B::f", "M::f", "A::f", // m.f()
    "B::f", "M::f", "A::f", // b.f()
];

#[test]
fn test_downcast() {
    let b = B::new();
    b.f();
    let a: &A = &*b;
    a.f();
    let m: &M = a.downcast_ref().unwrap();
    m.f();
    let b: &B = b.downcast_ref().unwrap();
    b.f();
    assert_eq!(BUF.take(), EXPECTED);
}
