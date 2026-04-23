use crate::BUF;
use oop_rs::prelude::*;

#[class]
type Shape = class<
    {
        pub fn new() -> Self {
            println!("Shape::constructor");
            Self {}
        }
        pub fn area(&self) {
            println!("Shape::area");
        }
        pub fn perimeter(&self) {
            println!("Shape::perimeter");
        }
        pub fn description(&self) {
            println!("Shape::description");
        }
    },
>;

#[class(extends(Shape))]
type Circle = class<
    {
        let radius: f64;
        pub fn new(radius: f64) -> Self {
            let self = Self {
                radius,
                ..Super::new()
            };
            println!("Circle::constructor");
            self
        }
        #[method(override(Shape))]
        pub fn area(&self) {
            println!("Circle::area");
        }
        #[method(override(Shape))]
        pub fn perimeter(&self) {
            println!("Circle::perimeter");
        }
    },
>;

static EXPECTED: &[&str] = &[
    "Shape::constructor",
    "Circle::constructor",
    "Circle::area",
    "Circle::perimeter",
    "Shape::description",
];

#[test]
fn override_methods() {
    let circle = Circle::new(10.0);
    circle.area();
    circle.perimeter();
    circle.description();
    assert_eq!(BUF.take(), EXPECTED);
}
