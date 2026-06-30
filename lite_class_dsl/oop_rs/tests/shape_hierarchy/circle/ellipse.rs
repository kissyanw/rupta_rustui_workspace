// Ellipse second-level derived class module

use oop_rs::prelude::*;

use super::{Circle, ICircle};
use crate::shape::Shape;

#[class(extends(Circle))]
pub type Ellipse = class<
    {
        #[vis(pub)]
        let mut semi_minor_axis: f64;

        pub fn new(semi_major_axis: f64, semi_minor_axis: f64, color: String) -> Self {
            Self {
                semi_minor_axis,
                ..Super::new(semi_major_axis, color)
            }
        }

        #[method(override(Shape))]
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.get().radius() * self.get().semi_minor_axis()
        }

        #[method(override(Shape))]
        pub fn perimeter(&self) -> f64 {
            let a = self.get().radius();
            let b = self.get().semi_minor_axis();
            std::f64::consts::PI * (3.0 * (a + b) - ((3.0 * a + b) * (a + 3.0 * b)).sqrt())
        }

        #[method(override(Shape))]
        pub fn description(&self) -> String {
            format!(
                "Ellipse with semi-major axis {:.2} and semi-minor axis {:.2}",
                self.get().radius(),
                self.get().semi_minor_axis()
            )
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
