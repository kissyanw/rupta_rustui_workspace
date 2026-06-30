use crate::BUF;
use oop_rs::prelude::*;

#[class]
type I = interface<
    {
        pub fn i(&self);
    },
>;
#[class]
type J = interface<
    {
        pub fn j(&self);
    },
>;
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
#[class(on(A), implements(I, J))]
type M = mixin<
    {
        #[method(override(A))]
        pub fn f(&self) {
            super.f();
            println!("M::f");
        }
        #[method(override(I))]
        pub fn i(&self) {
            println!("M::i");
        }
        #[method(override(J))]
        pub fn j(&self) {
            println!("M::j");
        }
    },
>;

#[class(extends(A), with(M))]
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
        #[method(override(I))]
        pub fn i(&self) {
            super.i();
            println!("B::i");
        }
        #[method(override(J))]
        pub fn j(&self) {
            super.j();
            println!("B::j");
        }
    },
>;

static EXPECTED: &[&str] = &["A::f", "M::f", "B::f", "M::i", "B::i", "M::j", "B::j"];

#[test]
fn test_multi_mixin() {
    let b = B::new();
    b.f();
    b.i();
    b.j();
    assert_eq!(BUF.take(), EXPECTED);
}
