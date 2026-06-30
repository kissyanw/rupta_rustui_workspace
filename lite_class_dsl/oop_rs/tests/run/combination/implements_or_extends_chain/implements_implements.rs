use crate::BUF;
use oop_rs::class;

// Test case: Class1 implements Class2 extends Class3
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
type Class2 = interface<
    {
        #[method(override(Class3))]
        pub fn print(&self) {
            println!("{}", self.get_value2());
        }

        pub fn double_print(&self) {
            self.print();
            self.print();
        }

        pub fn get_value2(&self) -> usize {
            unimplemented!()
        }
    },
>;

#[class(implements(Class2))]
type Class1 = class<
    {
        let mut value1: usize;

        pub fn new(value1: usize) -> Self {
            println!("Class1::constructor");
            Self { value1 }
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

static EXPECTED: &[&str] = &["Class1::constructor", "1", "1", "1"];

#[test]
#[cfg_attr(miri, ignore)] // FIXME: undefined behavior in miri
fn implements_extends_chain() {
    let class1 = Class1::new(1);
    class1.print();
    class1.double_print();
    assert_eq!(BUF.take(), EXPECTED);
}
