use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Bark = interface<
    {
        pub fn bark(&self);
    },
>;

#[class(implements(Bark))]
type Dog = class<
    {
        pub fn new() -> Self {
            Self {}
        }

        #[method(override(Bark))]
        pub fn bark(&self) {
            println!("Dog::bark");
        }
    },
>;

static EXPECTED: &[&str] = &["Dog::bark"];

#[test]
fn override_method() {
    let dog = Dog::new();
    dog.bark();
    assert_eq!(BUF.take(), EXPECTED);
}
