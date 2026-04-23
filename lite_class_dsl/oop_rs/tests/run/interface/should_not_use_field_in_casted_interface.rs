// Description: should not use field in casted interface

use crate::test_utils::{BUF, printlntb};
use ::classes::prelude::CRc;
use classes_macros::classes;
classes! {
    class Eat {
        struct {
            food: usize,
        }

        pub fn new() -> Self {
            Self {
                food: 0_usize,
            }
        }

        pub fn eat(&self) {
            printlntb!("Eat::eat");
        }
    }

    class Cat implements Eat {
        struct {
            name: usize,
        }

        pub fn new() -> Self {
            Self { name: 1_usize }
        }

        pub override fn Eat::eat(&self) {
            printlntb!("Cat::eat");
        }

        pub fn meow(&self) {
            printlntb!("Cat::meow");
        }
    }
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn single_interface_casting() {
    let eater = Cat::new().upcast::<CRc<Cat>, CRc<Eat>>();
    let food = eater.get_food();
    // FIXME: Undefined behavior, get_food() actually get Cat::name
    assert_eq!(food, 0);
}
