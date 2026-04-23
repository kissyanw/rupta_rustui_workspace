use crate::BUF;
use oop_rs::prelude::*;

#[class(implements(Downcast))]
type Eat = interface<
    {
        pub fn eat(&self);
    },
>;

#[class(implements(Downcast))]
type Drink = interface<
    {
        pub fn drink(&self);
    },
>;

#[class(implements(Eat, Drink))]
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
        pub fn meow(&self) {
            println!("Cat::meow");
        }
    },
>;

static EXPECTED: &[&str] = &["Cat::eat", "Cat::drink", "Cat::meow", "Cat::meow"];

#[test]
fn multiple_interfaces_casting() {
    BUF.take();
    let cat = Cat::new();
    let eater: &Eat = &*cat;
    eater.eat();
    let drinker: &Drink = &*cat;
    drinker.drink();
    let cat1: &Cat = drinker.downcast_ref().unwrap();
    cat1.meow();
    let cat2: &Cat = eater.downcast_ref().unwrap();
    cat2.meow();
    assert_eq!(BUF.take(), EXPECTED);
    assert_eq!(std::ptr::from_ref(cat1), std::ptr::from_ref(cat2));
}
