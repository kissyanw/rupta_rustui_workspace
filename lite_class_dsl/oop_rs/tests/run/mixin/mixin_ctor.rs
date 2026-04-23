use crate::BUF;
use oop_rs::prelude::*;

#[class]
type A = class<
    {
        pub fn new(x: i32) -> Self {
            println!("A::new, before, x = {x}");
            let self = Self {};
            println!("A::new, after, x = {x}");
            self
        }
    },
>;

#[class]
type B = class<
    {
        pub fn new(x: i32, y: u32) -> Self {
            println!("B::new, before, x = {x}, y = {y}");
            let self = Self {};
            println!("B::new, after, x = {x}, y = {y}");
            self
        }
    },
>;

#[class]
type M1 = mixin<
    {
        let ref x: String = "M1".to_string();
    },
>;

#[class]
type M2 = mixin<
    {
        let ref y: String = "M2".to_string();
    },
>;

#[class(extends(A), with(M1, M2))]
type C = class<
    {
        pub fn new() -> Self {
            println!("C::new, before");
            let self = Self {
                ..Super::new(1_i32)
            };
            println!(
                "C::new, after, x = {}, y = {}",
                self.get().x(),
                self.get().y()
            );
            self
        }
    },
>;

#[class(extends(B), with(M2, M1))]
type D = class<
    {
        pub fn new() -> Self {
            println!("D::new, before");
            let self = Self {
                ..Super::new(1_i32, 2_u32)
            };
            println!(
                "D::new, after, x = {}, y = {}",
                self.get().x(),
                self.get().y()
            );
            self
        }
    },
>;

static EXPECTED: &[&str] = &[
    // C::new()
    "C::new, before",
    "A::new, before, x = 1",
    "A::new, after, x = 1",
    "C::new, after, x = M1, y = M2", //
    // D::new()
    "D::new, before",
    "B::new, before, x = 1, y = 2",
    "B::new, after, x = 1, y = 2",
    "D::new, after, x = M1, y = M2", //
];

#[test]
fn test_mixin_on_mixin() {
    let _ = C::new();
    let _ = D::new();
    assert_eq!(BUF.take(), EXPECTED);
}
