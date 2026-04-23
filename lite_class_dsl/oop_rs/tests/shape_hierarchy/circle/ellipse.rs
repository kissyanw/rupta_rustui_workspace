// Ellipse second-level derived class module

use super::Circle;
use crate::shape::Shape;
use classes::*;

classes! {
    /// Ellipse second-level derived class
    /// Represents an ellipse, inherits from Circle (radius serves as semi_major_axis)
    pub class Ellipse extends Circle {
        struct {
            pub semi_minor_axis: f64,
        }

        /// Constructor: create an ellipse
        /// Parameters:
        /// - semi_major_axis: semi-major axis (stored in inherited radius field)
        /// - semi_minor_axis: semi-minor axis
        /// - color: color of the ellipse
        pub fn new(semi_major_axis: f64, semi_minor_axis: f64, color: String) -> Self {
            Self {
                super: Super::new(semi_major_axis, color),
                semi_minor_axis,
            }
        }

        /// Override: calculate ellipse area
        /// Formula: π × semi_major_axis × semi_minor_axis
        pub override fn Shape::area(&self) -> f64 {
            std::f64::consts::PI * self.get_radius() * self.get_semi_minor_axis()
        }

        /// Override: calculate ellipse perimeter
        /// Using Ramanujan's approximation formula:
        /// π × [3(a+b) - √((3a+b)(a+3b))]
        /// where a = semi_major_axis, b = semi_minor_axis
        pub override fn Shape::perimeter(&self) -> f64 {
            let a = self.get_radius(); // semi_major_axis
            let b = self.get_semi_minor_axis();

            // Ramanujan's approximation formula
            std::f64::consts::PI * (3.0 * (a + b) - ((3.0 * a + b) * (a + 3.0 * b)).sqrt())
        }

        /// Override: get ellipse description
        pub override fn Shape::description(&self) -> String {
            format!(
                "Ellipse with semi-major axis {:.2} and semi-minor axis {:.2}",
                self.get_radius(),
                self.get_semi_minor_axis()
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
        fn prop_ellipse_area_correctness(
            semi_major_axis in 0.1f64..1000.0f64,
            semi_minor_axis in 0.1f64..1000.0f64
        ) {
            let ellipse = Ellipse::new(semi_major_axis, semi_minor_axis, "test".to_string());
            let expected_area = std::f64::consts::PI * semi_major_axis * semi_minor_axis;
            let actual_area = ellipse.area();
            let epsilon = 1e-10;
            prop_assert!(
                (actual_area - expected_area).abs() < epsilon,
                "Ellipse area mismatch: expected {}, got {}",
                expected_area,
                actual_area
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_ellipse_perimeter_correctness(
            semi_major_axis in 0.1f64..1000.0f64,
            semi_minor_axis in 0.1f64..1000.0f64
        ) {
            let ellipse = Ellipse::new(semi_major_axis, semi_minor_axis, "test".to_string());
            let actual_perimeter = ellipse.perimeter();

            prop_assert!(
                actual_perimeter > 0.0,
                "Ellipse perimeter should be positive, but got {}",
                actual_perimeter
            );

            let a = semi_major_axis;
            let b = semi_minor_axis;
            let expected_perimeter = std::f64::consts::PI * (3.0 * (a + b) - ((3.0 * a + b) * (a + 3.0 * b)).sqrt());

            let epsilon = 1e-8;
            prop_assert!(
                (actual_perimeter - expected_perimeter).abs() < epsilon,
                "Ellipse perimeter mismatch: expected {}, got {}",
                expected_perimeter,
                actual_perimeter
            );

            if (a - b).abs() < 1e-6 {
                let circle_perimeter = 2.0 * std::f64::consts::PI * a;
                let relative_error = ((actual_perimeter - circle_perimeter) / circle_perimeter).abs();
                prop_assert!(
                    relative_error < 0.01,
                    "When a=b, ellipse perimeter should be close to circle perimeter: expected ~{}, got {}",
                    circle_perimeter,
                    actual_perimeter
                );
            }
        }
    }
}
