use crate::BUF;
use oop_rs::prelude::*;

#[class(extends(Object))]
type Animal = class<
    {
        pub fn new() -> Self {
            Self {}
        }
        pub fn eat(&self) {
            println!("Animal::eat");
        }
    },
>;

#[class(extends(Animal))]
type Dog = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(Animal))]
        pub fn eat(&self) {
            println!("Dog::eat");
        }

        pub fn bark(&self) {
            println!("Dog::bark");
        }
    },
>;

static EXPECTED: &[&str] = &["Dog::eat", "Dog::bark"];

#[test]
fn two_classes_casting() {
    // upcast
    let animal: CRc<Animal> = Dog::new();
    animal.eat(); // use the override method
    // downcast
    let dog: &Dog = animal.downcast_ref().unwrap();
    dog.bark();
    assert_eq!(BUF.take(), EXPECTED);
}
