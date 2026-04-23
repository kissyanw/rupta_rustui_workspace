// Description: Inheritance and multiple interfaces

use crate::BUF;
use oop_rs::class;

#[class]
type Play = interface<
    {
        pub fn play(&self);
    },
>;

#[class]
type Sleep = interface<
    {
        pub fn sleep(&self);
    },
>;

#[class]
type Animal = class<
    {
        pub fn new() -> Self {
            Self {}
        }
        pub fn drink(&self) {
            println!("Animal::drink");
        }
        pub fn eat(&self) {
            println!("Animal::eat");
        }
    },
>;

#[class(extends(Animal), implements(Play, Sleep))]
type Dog = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(Play))]
        pub fn play(&self) {
            println!("Dog::play");
        }

        #[method(override(Sleep))]
        pub fn sleep(&self) {
            println!("Dog::sleep");
        }

        #[method(override(Animal))]
        pub fn eat(&self) {
            println!("Dog::eat");
        }
    },
>;

static EXPECTED: &[&str] = &["Animal::drink", "Dog::eat", "Dog::play", "Dog::sleep"];

#[test]
fn inheritance_and_interface() {
    let dog = Dog::new();
    dog.drink();
    dog.eat();
    dog.play();
    dog.sleep();
    assert_eq!(BUF.take(), EXPECTED);
}
