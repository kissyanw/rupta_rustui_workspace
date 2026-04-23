use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Eat = interface<
    {
        pub fn eat(&self);
    },
>;

#[class]
type Drink = interface<
    {
        pub fn drink(&self);
    },
>;

#[class]
type Sleep = interface<
    {
        pub fn sleep(&self);
    },
>;

#[class(implements(Eat, Drink, Sleep))]
type Cat = class<
    {
        pub fn new() -> Self {
            Self {}
        }

        #[method(override(Eat))]
        pub fn eat(&self) {
            println!("Cat::eat");
        }
        #[method(override(Drink))]
        pub fn drink(&self) {
            println!("Cat::drink");
        }
        #[method(override(Sleep))]
        pub fn sleep(&self) {
            println!("Cat::sleep");
        }
    },
>;

static EXPECTED: &[&str] = &["Cat::eat", "Cat::drink", "Cat::sleep"];

#[test]
fn multiple_interfaces() {
    let cat = Cat::new();
    cat.eat();
    cat.drink();
    cat.sleep();
    assert_eq!(BUF.take(), EXPECTED);
}
