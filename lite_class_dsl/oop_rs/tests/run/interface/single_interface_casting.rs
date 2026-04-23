use crate::BUF;
use oop_rs::prelude::*;

#[class(implements(Downcast))]
type Eat = interface<
    {
        pub fn eat(&self);
    },
>;

#[class(implements(Eat))]
type Cat = class<
    {
        pub fn new() -> Self {
            Self {}
        }

        #[method(override(Eat))]
        pub fn eat(&self) {
            println!("Cat::eat");
        }

        pub fn meow(&self) {
            println!("Cat::meow");
        }
    },
>;

static EXPECTED: &[&str] = &["Cat::eat", "Cat::meow"];

#[test]
fn single_interface_casting() {
    let eater: CRc<Eat> = Cat::new();
    eater.eat();
    let cat: &Cat = eater.downcast_ref().unwrap();
    cat.meow();
    assert_eq!(BUF.take(), EXPECTED);
}
