// Square second-level derived class module

use super::Rectangle;
use crate::shape::Shape;
use classes::*;

classes! {
    /// Square second-level derived class
    /// Represents a square, inherits from Rectangle (width == height)
    pub class Square extends Rectangle {
        /// Constructor: create a square
        /// Parameters:
        /// - side: side length of the square (sets width = height = side)
        /// - color: color of the square
        pub fn new(side: f64, color: String) -> Self {
            Self {
                super: Super::new(side, side, color),
            }
        }

        /// Override: get square description
        pub override fn Shape::description(&self) -> String {
            format!(
                "Square with side {:.2}",
                self.get_width()
            )
        }
    }
}

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

            let square_as_rectangle: CRc<Rectangle> = square.clone().into_super();
            let area_via_rectangle = square_as_rectangle.area();
            prop_assert!(
                (area_via_rectangle - expected_area).abs() < epsilon,
                "Square area via Rectangle interface mismatch: expected {}, got {}",
                expected_area,
                area_via_rectangle
            );

            prop_assert!(
                (square.get_width() - square.get_height()).abs() < epsilon,
                "Square width and height should be equal: width={}, height={}",
                square.get_width(),
                square.get_height()
            );

            prop_assert!(
                (square.get_width() - side).abs() < epsilon,
                "Square width should equal side: expected {}, got {}",
                side,
                square.get_width()
            );

            prop_assert!(
                (square.get_height() - side).abs() < epsilon,
                "Square height should equal side: expected {}, got {}",
                side,
                square.get_height()
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

            let square_as_rectangle: CRc<Rectangle> = square.clone().into_super();
            let perimeter_via_rectangle = square_as_rectangle.perimeter();
            prop_assert!(
                (perimeter_via_rectangle - expected_perimeter).abs() < epsilon,
                "Square perimeter via Rectangle interface mismatch: expected {}, got {}",
                expected_perimeter,
                perimeter_via_rectangle
            );

            let square_as_shape: CRc<Shape> = square_as_rectangle.into_super();
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
