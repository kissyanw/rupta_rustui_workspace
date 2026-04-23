use crate::BUF;
use std::fmt::Debug;

use oop_rs::prelude::*;

#[class]
type Class = class<
    {
        fn new() -> Self {
            Self {}
        }

        #[method(final)]
        fn method<T: Debug>(&self, value: T) {
            println!("Class::method: {value:?}");
        }
    },
>;

#[class(extends(Class))]
type DerivedClass = class<
    {
        fn new() -> Self {
            Self {}
        }
        #[method(final)]
        fn method<T: Debug>(&self, value: T) {
            println!("DerivedClass::method: {value:?}");
        }
    },
>;

static EXPECTED: &[&str] = &[
    "Class::method: 42",
    "DerivedClass::method: 42",
    "DerivedClass::method: 42",  // via &DerivedClass
    "Class::method: 42",
];

#[test]
fn test_final_methods() {
    BUF.take();
    let class = Class::new();
    class.method(42);
    let derived_class = DerivedClass::new();
    derived_class.method(42);
    let derived_class_dyn: &DerivedClass = &*derived_class;
    derived_class_dyn.method(42);
    let derived_class: &Class = &*derived_class;
    derived_class.method(42);
    assert_eq!(BUF.take(), EXPECTED);
}
