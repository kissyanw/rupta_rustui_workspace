use crate::BUF;
use oop_rs::prelude::*;

#[class(extends(Object))]
type Alice = class<
    {
        pub fn new() -> Self {
            Self {}
        }

        pub fn eat(&self) {
            println!("Alice::eat");
        }
    },
>;

#[class(extends(Alice))]
type Bob = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(Alice))]
        pub fn eat(&self) {
            println!("Bob::eat");
        }
    },
>;

#[class(extends(Bob))]
type Carol = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(Alice))]
        pub fn eat(&self) {
            println!("Carol::eat");
        }
    },
>;

static EXPECTED: &[&str] = &["Carol::eat", "Carol::eat", "Carol::eat", "Carol::eat"];

#[test]
fn multiple_classes_casting() {
    let bob: CRc<Bob> = Carol::new();
    bob.eat();
    let alice: &Alice = &*bob;
    alice.eat();
    let bob: &Bob = alice.downcast_ref().unwrap();
    bob.eat();
    let carol: &Carol = bob.downcast_ref().unwrap();
    carol.eat();
    assert_eq!(BUF.take(), EXPECTED);
}
