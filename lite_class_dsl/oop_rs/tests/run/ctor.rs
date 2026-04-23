use crate::BUF;

use oop_rs::class;

#[class]
type A = class<
    {
        let mut x: u32 = 0_u32;
        pub fn new() -> Self {
            let self = Self { .. };
            self.f();
            self
        }
        pub fn f(&self) {
            println!("A::f, x = {}", self.get().x());
        }
    },
>;

#[class(extends(A))]
type B = class<
    {
        let mut y: u32 = 1_u32;
        pub fn new() -> Self {
            let self = Self { ..Super::new() };
            self.f();
            self.g();
            self
        }
        #[method(override(A))]
        pub fn f(&self) {
            super.f();
            println!("B::f, x = {}, y = {}", self.get().x(), self.get().y());
        }
        pub fn g(&self) {
            println!("B::g, x = {}, y = {}", self.get().x(), self.get().y());
        }
    },
>;

static EXPECTED: &[&str] = &[
    // A::new()
    "A::f, x = 0", // A::f()
    // A::new()
    "A::f, x = 0",        // A::f()
    "B::f, x = 0, y = 1", // B::f()
    // B::new()
    "A::f, x = 0",        // A::f()
    "B::f, x = 0, y = 1", // B::f()
    "B::g, x = 0, y = 1", // B::g()
];

#[test]
fn test_ctor() {
    let _a = A::new();
    let _b = B::new();
    assert_eq!(BUF.take(), EXPECTED);
}
