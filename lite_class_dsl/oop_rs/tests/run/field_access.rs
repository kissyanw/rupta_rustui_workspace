use oop_rs::class;

use crate::BUF;

#[class(abstract)]
type Animal = class<
    {
        let ref name: String;

        fn new(name: impl Into<String>) -> Self {
            Self { name: name.into() }
        }
        fn eat(&self) {
            println!("{}::eat", self.get().name());
        }
    },
>;

#[class(extends(Animal))]
type Dog = class<
    {
        fn new(name: impl Into<String>) -> Self {
            Self { ..Super::new(name) }
        }
        #[method(override(Animal))]
        fn eat(&self) {
            super.eat();
            self.bark();
        }
        fn bark(&self) {
            println!("{}::bark", self.get().name());
        }
    },
>;

#[test]
fn test_field_access_dog() {
    BUF.take();

    let dog = Dog::new("dog");
    dog.eat();
    dog.bark();

    assert_eq!(BUF.take(), EXPECTED_OUTPUT);
}

static EXPECTED_OUTPUT: &[&str] = &["dog::eat", "dog::bark", "dog::bark"];
