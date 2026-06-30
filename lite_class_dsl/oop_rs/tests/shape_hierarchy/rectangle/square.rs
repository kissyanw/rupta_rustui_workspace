// Square second-level derived class module

use oop_rs::prelude::*;

use super::{Rectangle, IRectangle};
use crate::shape::{Shape, IShape};

#[class(extends(Rectangle))]
pub type Square = class<
    {
        pub fn new(side: f64, color: String) -> Self {
            Self {
                ..Super::new(side, side, color)
            }
        }

        #[method(override(Shape))]
        pub fn description(&self) -> String {
            format!("Square with side {:.2}", self.get().width())
        }
    },
>;

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_square_area_correctness(side in 0.1f64..1000.0f64) {
            let square = Square::new(side, "test".to_string());
            let expected_area = side * side;
            let actual_area = square.area();
            let epsilon = 1e-10;

            prop_assert!(
                (actual_area - expected_area).abs() < epsilon,
                "Square area mismatch: expected {}, got {}",
                expected_area,
                actual_area
            );

            let square_as_rectangle: CRc<Rectangle> = square.clone() as CRc<Rectangle>;
            let area_via_rectangle = square_as_rectangle.area();
            prop_assert!(
                (area_via_rectangle - expected_area).abs() < epsilon,
                "Square area via Rectangle interface mismatch: expected {}, got {}",
                expected_area,
                area_via_rectangle
            );

            prop_assert!(
                (square.get().width() - square.get().height()).abs() < epsilon,
                "Square width and height should be equal: width={}, height={}",
                square.get().width(),
                square.get().height()
            );

            prop_assert!(
                (square.get().width() - side).abs() < epsilon,
                "Square width should equal side: expected {}, got {}",
                side,
                square.get().width()
            );

            prop_assert!(
                (square.get().height() - side).abs() < epsilon,
                "Square height should equal side: expected {}, got {}",
                side,
                square.get().height()
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_square_perimeter_correctness(side in 0.1f64..1000.0f64) {
            let square = Square::new(side, "test".to_string());
            let expected_perimeter = 4.0 * side;
            let actual_perimeter = square.perimeter();
            let epsilon = 1e-10;

            prop_assert!(
                (actual_perimeter - expected_perimeter).abs() < epsilon,
                "Square perimeter mismatch: expected {}, got {}",
                expected_perimeter,
                actual_perimeter
            );

            let square_as_rectangle: CRc<Rectangle> = square.clone() as CRc<Rectangle>;
            let perimeter_via_rectangle = square_as_rectangle.perimeter();
            prop_assert!(
                (perimeter_via_rectangle - expected_perimeter).abs() < epsilon,
                "Square perimeter via Rectangle interface mismatch: expected {}, got {}",
                expected_perimeter,
                perimeter_via_rectangle
            );

            let square_as_shape: CRc<Shape> = square_as_rectangle as CRc<Shape>;
            let perimeter_via_shape = square_as_shape.perimeter();
            prop_assert!(
                (perimeter_via_shape - expected_perimeter).abs() < epsilon,
                "Square perimeter via Shape interface mismatch: expected {}, got {}",
                expected_perimeter,
                perimeter_via_shape
            );
        }
    }
}
