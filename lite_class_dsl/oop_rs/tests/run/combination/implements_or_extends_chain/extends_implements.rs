use crate::BUF;
use oop_rs::class;

// Test case: Class1 extends Class2 implements Class3
#[class]
type Class3 = interface<
    {
        pub fn print(&self) {
            println!("{}", self.get_value3());
        }

        pub fn get_value3(&self) -> usize {
            unimplemented!()
        }
    },
>;

#[class(implements(Class3))]
type Class2 = class<
    {
        let mut value2: usize;

        pub fn new(value2: usize) -> Self {
            println!("Class2::constructor");
            Self { value2 }
        }

        #[method(override(Class3))]
        pub fn print(&self) {
            println!("{}", self.get().value2());
        }

        pub fn double_print(&self) {
            self.print();
            self.print();
        }
    },
>;

#[class(extends(Class2))]
type Class1 = class<
    {
        let mut value1: usize;

        pub fn new(value1: usize, value2: usize) -> Self {
            let self = Self {
                value1,
                ..Super::new(value2)
            };
            println!("Class1::constructor");
            self
        }

        #[method(override(Class3))]
        pub fn print(&self) {
            println!("{}", self.get().value1());
        }

        #[method(override(Class2))]
        pub fn double_print(&self) {
            self.print();
            self.print();
        }
    },
>;

static EXPECTED: &[&str] = &["Class2::constructor", "Class1::constructor", "1", "1", "1"];

#[test]
fn extends_implements_schain() {
    let class1 = Class1::new(1, 2);
    class1.print();
    class1.double_print();
    assert_eq!(BUF.take(), EXPECTED);
}
