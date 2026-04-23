// RoundedRectangle second-level derived class module

use super::Rectangle;
use crate::shape::Shape;
use classes::*;

classes! {
    /// RoundedRectangle second-level derived class
    /// Represents a rectangle with rounded corners, inherits from Rectangle
    pub class RoundedRectangle extends Rectangle {
        struct {
            pub corner_radius: f64,
        }

        /// Constructor: create a rectangle with rounded corners
        /// Parameters:
        /// - width: width of the rectangle
        /// - height: height of the rectangle
        /// - corner_radius: corner radius
        /// - color: color of the rectangle
        pub fn new(width: f64, height: f64, corner_radius: f64, color: String) -> Self {
            Self {
                super: Super::new(width, height, color),
                corner_radius,
            }
        }

        /// Override: calculate rounded rectangle area
        /// Formula: width × height - 4 × (r² - πr²/4)
        /// where r is the corner radius
        /// Explanation: rectangle area - 4 corner square areas + 4 corner circle areas
        pub override fn Shape::area(&self) -> f64 {
            let width = self.get_width();
            let height = self.get_height();
            let r = self.get_corner_radius();

            let rect_area = width * height;
            let square_corners = 4.0 * r * r;
            let circle_corners = std::f64::consts::PI * r * r;

            rect_area - square_corners + circle_corners
        }

        /// Override: calculate rounded rectangle perimeter
        /// Formula: 2 × (width + height) - 8r + 2πr
        /// Explanation: rectangle perimeter - 8 straight edge lengths + 4 arc lengths
        pub override fn Shape::perimeter(&self) -> f64 {
            let width = self.get_width();
            let height = self.get_height();
            let r = self.get_corner_radius();

            let rect_perimeter = 2.0 * (width + height);
            let removed_edges = 8.0 * r;
            let arc_length = 2.0 * std::f64::consts::PI * r;

            rect_perimeter - removed_edges + arc_length
        }

        /// Override: get rounded rectangle description
        pub override fn Shape::description(&self) -> String {
            format!(
                "RoundedRectangle with width {:.2}, height {:.2}, corner radius {:.2}",
                self.get_width(),
                self.get_height(),
                self.get_corner_radius()
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
        fn prop_rounded_rectangle_area_correctness(
            width in 0.1f64..1000.0f64,
            height in 0.1f64..1000.0f64,
            corner_radius in 0.1f64..100.0f64
        ) {
            let max_radius = (width.min(height)) / 2.0;
            prop_assume!(corner_radius <= max_radius);

            let rounded_rect = RoundedRectangle::new(
                width,
                height,
                corner_radius,
                "test".to_string()
            );

            let rect_area = width * height;
            let square_corners = 4.0 * corner_radius * corner_radius;
            let circle_corners = std::f64::consts::PI * corner_radius * corner_radius;
            let expected_area = rect_area - square_corners + circle_corners;

            let actual_area = rounded_rect.area();
            let epsilon = 1e-10;

            prop_assert!(
                (actual_area - expected_area).abs() < epsilon,
                "RoundedRectangle area mismatch: expected {}, got {}",
                expected_area,
                actual_area
            );

            prop_assert!(
                actual_area > 0.0,
                "RoundedRectangle area should be positive, but got {}",
                actual_area
            );

            prop_assert!(
                actual_area <= rect_area + epsilon,
                "RoundedRectangle area should not exceed rectangle area: rounded={}, rect={}",
                actual_area,
                rect_area
            );

            let rounded_rect_zero = RoundedRectangle::new(width, height, 0.0, "test".to_string());
            let area_zero_radius = rounded_rect_zero.area();
            prop_assert!(
                (area_zero_radius - rect_area).abs() < epsilon,
                "RoundedRectangle with zero radius should equal rectangle area: expected {}, got {}",
                rect_area,
                area_zero_radius
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_rounded_rectangle_perimeter_correctness(
            width in 0.1f64..1000.0f64,
            height in 0.1f64..1000.0f64,
            corner_radius in 0.1f64..100.0f64
        ) {
            let max_radius = (width.min(height)) / 2.0;
            prop_assume!(corner_radius <= max_radius);

            let rounded_rect = RoundedRectangle::new(
                width,
                height,
                corner_radius,
                "test".to_string()
            );

            let rect_perimeter = 2.0 * (width + height);
            let removed_edges = 8.0 * corner_radius;
            let arc_length = 2.0 * std::f64::consts::PI * corner_radius;
            let expected_perimeter = rect_perimeter - removed_edges + arc_length;

            let actual_perimeter = rounded_rect.perimeter();
            let epsilon = 1e-10;

            prop_assert!(
                (actual_perimeter - expected_perimeter).abs() < epsilon,
                "RoundedRectangle perimeter mismatch: expected {}, got {}",
                expected_perimeter,
                actual_perimeter
            );

            prop_assert!(
                actual_perimeter > 0.0,
                "RoundedRectangle perimeter should be positive, but got {}",
                actual_perimeter
            );

            let rounded_rect_zero = RoundedRectangle::new(width, height, 0.0, "test".to_string());
            let perimeter_zero_radius = rounded_rect_zero.perimeter();
            prop_assert!(
                (perimeter_zero_radius - rect_perimeter).abs() < epsilon,
                "RoundedRectangle with zero radius should equal rectangle perimeter: expected {}, got {}",
                rect_perimeter,
                perimeter_zero_radius
            );

            if corner_radius > 1e-6 {
                prop_assert!(
                    actual_perimeter < rect_perimeter + epsilon,
                    "RoundedRectangle perimeter should be less than rectangle perimeter: rounded={}, rect={}",
                    actual_perimeter,
                    rect_perimeter
                );
            }
        }
    }
}
