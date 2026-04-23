// Rectangle class module

use crate::shape::Shape;
use classes::*;

pub mod rounded_rectangle;
pub mod square;

classes! {
    /// Rectangle derived class
    /// Represents a rectangle, inherits from Shape
    pub class Rectangle extends Shape {
        struct {
            pub width: f64,
            pub height: f64,
        }

        /// Constructor: create a rectangle
        /// Parameters:
        /// - width: width of the rectangle
        /// - height: height of the rectangle
        /// - color: color of the rectangle
        pub fn new(width: f64, height: f64, color: String) -> Self {
            Self {
                super: Super::new(color),
                width,
                height,
            }
        }

        /// Override: calculate rectangle area
        /// Formula: width × height
        pub override fn Shape::area(&self) -> f64 {
            self.get_width() * self.get_height()
        }

        /// Override: calculate rectangle perimeter
        /// Formula: 2 × (width + height)
        pub override fn Shape::perimeter(&self) -> f64 {
            2.0 * (self.get_width() + self.get_height())
        }

        /// Override: get rectangle description
        pub override fn Shape::description(&self) -> String {
            format!(
                "Rectangle with width {:.2} and height {:.2}",
                self.get_width(),
                self.get_height()
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
        fn prop_rectangle_area_correctness(
            width in 0.1f64..1000.0f64,
            height in 0.1f64..1000.0f64
        ) {
            let rectangle = Rectangle::new(width, height, "test".to_string());
            let expected_area = width * height;
            let actual_area = rectangle.area();
            let epsilon = 1e-10;
            prop_assert!(
                (actual_area - expected_area).abs() < epsilon,
                "Rectangle area mismatch: expected {}, got {}",
                expected_area,
                actual_area
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_rectangle_perimeter_correctness(
            width in 0.1f64..1000.0f64,
            height in 0.1f64..1000.0f64
        ) {
            let rectangle = Rectangle::new(width, height, "test".to_string());
            let expected_perimeter = 2.0 * (width + height);
            let actual_perimeter = rectangle.perimeter();
            let epsilon = 1e-10;
            prop_assert!(
                (actual_perimeter - expected_perimeter).abs() < epsilon,
                "Rectangle perimeter mismatch: expected {}, got {}",
                expected_perimeter,
                actual_perimeter
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_rectangle_description_contains_info(
            width in 0.1f64..1000.0f64,
            height in 0.1f64..1000.0f64
        ) {
            let rectangle = Rectangle::new(width, height, "test".to_string());
            let description = rectangle.description();

            prop_assert!(
                description.contains("Rectangle"),
                "Description should contain 'Rectangle', but got: {}",
                description
            );

            let width_str = format!("{:.2}", width);
            prop_assert!(
                description.contains(&width_str),
                "Description should contain width value '{}', but got: {}",
                width_str,
                description
            );

            let height_str = format!("{:.2}", height);
            prop_assert!(
                description.contains(&height_str),
                "Description should contain height value '{}', but got: {}",
                height_str,
                description
            );
        }
    }
}
