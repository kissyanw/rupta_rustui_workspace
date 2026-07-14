use oop_rs::prelude::*;

#[class(extends(Object))]
type Animal = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(extends(Animal))]
type Dog = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(extends(Animal))]
type Cat = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

fn must_succeed_downcast(animal: CRc<Animal>) -> bool {
    animal.downcast_rc::<Dog>().is_ok()
}

fn must_fail_downcast(animal: CRc<Animal>) -> bool {
    animal.downcast_rc::<Cat>().is_ok()
}

fn main() {
    let animal: CRc<Animal> = Dog::new();

    assert!(must_succeed_downcast(animal.clone()));
    assert!(!must_fail_downcast(animal));
}

