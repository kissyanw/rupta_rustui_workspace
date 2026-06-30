// Circle class module

use oop_rs::prelude::*;

use crate::shape::{Shape, IShape};

pub mod colored_circle;
pub mod ellipse;

#[class(extends(Shape))]
pub type Circle = class<
    {
        #[vis(pub)]
        let mut radius: f64;

        pub fn new(radius: f64, color: String) -> Self {
            Self {
                radius,
                ..Super::new(color)
            }
        }

        #[method(override(Shape))]
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.get().radius() * self.get().radius()
        }

        #[method(override(Shape))]
        pub fn perimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.get().radius()
        }

        #[method(override(Shape))]
        pub fn description(&self) -> String {
            format!("Circle with radius {:.2}", self.get().radius())
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
        fn prop_circle_area_correctness(radius in 0.1f64..1000.0f64) {
            let circle = Circle::new(radius, "test".to_string());
            let expected_area = std::f64::consts::PI * radius * radius;
            let actual_area = circle.area();
            let epsilon = 1e-10;
            prop_assert!(
                (actual_area - expected_area).abs() < epsilon,
                "Circle area mismatch: expected {}, got {}",
                expected_area,
                actual_area
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_circle_perimeter_correctness(radius in 0.1f64..1000.0f64) {
            let circle = Circle::new(radius, "test".to_string());
            let expected_perimeter = 2.0 * std::f64::consts::PI * radius;
            let actual_perimeter = circle.perimeter();
            let epsilon = 1e-10;
            prop_assert!(
                (actual_perimeter - expected_perimeter).abs() < epsilon,
                "Circle perimeter mismatch: expected {}, got {}",
                expected_perimeter,
                actual_perimeter
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_circle_description_contains_type_info(radius in 0.1f64..1000.0f64) {
            let circle = Circle::new(radius, "test".to_string());
            let description = circle.description();
            prop_assert!(
                description.contains("Circle"),
                "Description should contain 'Circle', but got: {}",
                description
            );
            let radius_str = format!("{:.2}", radius);
            prop_assert!(
                description.contains(&radius_str),
                "Description should contain radius value '{}', but got: {}",
                radius_str,
                description
            );
        }
    }
}
