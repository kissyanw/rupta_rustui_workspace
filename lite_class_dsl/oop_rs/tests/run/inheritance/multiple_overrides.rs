use crate::BUF;
use oop_rs::class;

#[class(abstract)]
type Shape = class<
    {
        pub fn new() -> Self {
            Self {}
        }

        pub fn draw(&self);
        pub fn resize(&self, scale: f64);

        pub fn meow(&self) {
            println!("Shape::meow");
        }
    },
>;

#[class(extends(Shape))]
type Circle = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(Shape))]
        pub fn draw(&self) {
            println!("Circle::draw");
        }

        #[method(override(Shape))]
        pub fn resize(&self, _scale: f64) {
            println!("Circle::resize");
        }
    },
>;

#[class(extends(Shape))]
type Triangle = class<
    {
        pub fn new() -> Self {
            Self {}
        }

        #[method(override(Shape))]
        pub fn draw(&self) {
            println!("Triangle::draw");
        }

        #[method(override(Shape))]
        pub fn resize(&self, _scale: f64) {
            println!("Triangle::resize");
        }

        #[method(override(Shape))]
        pub fn meow(&self) {
            println!("Triangle::meow");
        }
    },
>;

static EXPECTED: &[&str] = &[
    "Circle::draw",
    "Circle::resize",
    "Shape::meow",
    "Triangle::draw",
    "Triangle::resize",
    "Triangle::meow",
];

#[test]
fn test() {
    let circle = Circle::new();
    circle.draw();
    circle.resize(1.0);
    circle.meow();
    let triangle = Triangle::new();
    triangle.draw();
    triangle.resize(1.0);
    triangle.meow();
    assert_eq!(BUF.take(), EXPECTED);
}
