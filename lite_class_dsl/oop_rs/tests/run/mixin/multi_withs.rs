use crate::BUF;
use oop_rs::prelude::*;

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

static EXPECTED: &[&str] = &["A::f", "M1::f", "M2::f", "B::f"];

#[test]
fn test_multi_mixin() {
    let b = B::new();
    b.f();
    assert_eq!(BUF.take(), EXPECTED);
}
