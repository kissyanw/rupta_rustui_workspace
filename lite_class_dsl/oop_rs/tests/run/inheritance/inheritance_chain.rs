use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Alice = class<
    {
        let alice: usize;

        pub fn new(alice: usize) -> Self {
            println!("Alice::constructor");
            Self { alice }
        }
    },
>;

#[class(extends(Alice))]
type Bob = class<
    {
        let bob: usize;

        pub fn new(alice: usize, bob: usize) -> Self {
            let self = Self {
                bob,
                ..Super::new(alice)
            };
            println!("Bob::constructor");
            self
        }
    },
>;

#[class(extends(Bob))]
type Carol = class<
    {
        let carol: usize;

        pub fn new(alice: usize, bob: usize, carol: usize) -> Self {
            let self = Self {
                carol,
                ..Super::new(alice, bob)
            };
            println!("Carol::constructor");
            self
        }
    },
>;

#[class(extends(Carol))]
type Dave = class<
    {
        let dave: usize;

        pub fn new(alice: usize, bob: usize, carol: usize, dave: usize) -> Self {
            let self = Self {
                dave,
                ..Super::new(alice, bob, carol)
            };
            println!("Dave::constructor");
            self
        }
    },
>;

static EXPECTED: &[&str] = &[
    "Alice::constructor",
    "Bob::constructor",
    "Carol::constructor",
    "Dave::constructor",
];

#[test]
fn inheritance_chain() {
    let _dave = Dave::new(1, 2, 3, 4);
    assert_eq!(BUF.take(), EXPECTED);
}
