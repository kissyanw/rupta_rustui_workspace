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

#[class(extends(A))]
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
        pub fn g(&self) {
            println!("B::g");
        }
    },
>;

#[class(extends(B), with(M))]
type C = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
        #[method(override(A))]
        pub fn f(&self) {
            super.f();
            println!("C::f");
        }
        #[method(override(B))]
        pub fn g(&self) {
            super.g();
            println!("C::g");
        }
    },
>;

#[class(on(A, B))]
type M = mixin<
    {
        #[method(override(A))]
        pub fn f(&self) {
            super.f();
            println!("M::f");
        }
        #[method(override(B))]
        pub fn g(&self) {
            super.g();
            println!("M::g");
        }
    },
>;

static EXPECTED: &[&str] = &["A::f", "B::f", "M::f", "C::f", "B::g", "M::g", "C::g"];

#[test]
fn test_multi_mixin() {
    let c = C::new();
    c.f();
    c.g();
    assert_eq!(BUF.take(), EXPECTED);
}
