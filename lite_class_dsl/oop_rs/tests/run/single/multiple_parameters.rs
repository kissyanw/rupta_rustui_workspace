use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Animal = class<
    {
        let ref name: String;
        let age: usize;
        let gender: bool;

        pub fn new(name: String, age: usize, gender: bool) -> Self {
            println!("Animal::constructor");
            Self { name, age, gender }
        }
    },
>;

static EXPECTED: &[&str] = &["Animal::constructor"];

#[test]
fn multiple_parameters() {
    let _animal = Animal::new("Dog".to_string(), 10, true);
    assert_eq!(BUF.take(), EXPECTED);
}
