use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Animal = class<
    {
        let ref name: String = "".to_string();
        let age: usize = 0_usize;
        let gender: bool;
        pub fn new(gender: bool) -> Self {
            println!("Animal::constructor");
            Self { gender }
        }
    },
>;

static EXPECTED: &[&str] = &["Animal::constructor"];

#[test]
fn default_field_value() {
    let animal = Animal::new(true);
    assert_eq!(BUF.take(), EXPECTED);
    assert_eq!(animal.get().name(), "");
    assert_eq!(animal.get().age(), 0);
    assert_eq!(animal.get().gender(), true);
}
