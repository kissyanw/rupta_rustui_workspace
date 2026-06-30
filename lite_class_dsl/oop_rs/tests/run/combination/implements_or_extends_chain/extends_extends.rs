use crate::BUF;

use oop_rs::class;

// Test case: Class1 extends Class2 extends Class3
#[class]
type Class3 = class<
    {
        let mut value3: usize;

        pub fn new(value3: usize) -> Self {
            println!("Class3::constructor");
            Self { value3 }
        }

        #[allow(dead_code)]
        pub fn print(&self) {
            println!("{}", self.get().value3());
        }
    },
>;

#[class(extends(Class3))]
type Class2 = class<
    {
        let mut value2: usize;

        pub fn new(value3: usize, value2: usize) -> Self {
            let self = Self {
                value2,
                ..Super::new(value3)
            };
            println!("Class2::constructor");
            self
        }

        #[allow(dead_code)]
        pub fn print(&self) {
            println!("{}", self.get().value2());
        }
    },
>;

#[class(extends(Class2))]
type Class1 = class<
    {
        let mut value1: usize;

        pub fn new(value3: usize, value2: usize, value1: usize) -> Self {
            let self = Self {
                value1,
                ..Super::new(value3, value2)
            };
            println!("Class1::constructor");
            self
        }

        pub fn print(&self) {
            println!("{}", self.get().value1());
        }
    },
>;

static EXPECTED: &[&str] = &[
    "Class3::constructor",
    "Class2::constructor",
    "Class1::constructor",
    "3",
];

#[test]
fn extends_extends_chain() {
    let class1 = Class1::new(1, 2, 3);
    class1.print();
    assert_eq!(BUF.take(), EXPECTED);
}
