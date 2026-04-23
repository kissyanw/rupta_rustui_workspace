use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Animal = class<
    {
        let ref name: String = "".to_string();
        let age: usize = 0_usize;
        let gender: bool;

        pub fn new(gender: bool, name: String) -> Self {
            println!("Animal::constructor");
            Self { gender, name }
        }
    },
>;

static EXPECTED: &[&str] = &["Animal::constructor"];

#[test]
fn override_default_field_value() {
    let animal = Animal::new(true, "Dog".to_string());
    assert_eq!(BUF.take(), EXPECTED);
    assert_eq!(animal.get().name(), "Dog");
    assert_eq!(animal.get().age(), 0);
    assert_eq!(animal.get().gender(), true);
}
