// Triangle class module

use oop_rs::prelude::*;

use crate::shape::{Shape, IShape};

#[class(extends(Shape))]
pub type Triangle = class<
    {
        #[vis(pub)]
        let mut side_a: f64;
        #[vis(pub)]
        let mut side_b: f64;
        #[vis(pub)]
        let mut side_c: f64;

        pub fn new(side_a: f64, side_b: f64, side_c: f64, color: String) -> Self {
            Self {
                side_a,
                side_b,
                side_c,
                ..Super::new(color)
            }
        }

        #[method(override(Shape))]
        pub fn area(&self) -> f64 {
            let a = self.get().side_a();
            let b = self.get().side_b();
            let c = self.get().side_c();

            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }

        #[method(override(Shape))]
        pub fn perimeter(&self) -> f64 {
            self.get().side_a() + self.get().side_b() + self.get().side_c()
        }

        #[method(override(Shape))]
        pub fn description(&self) -> String {
            format!(
                "Triangle with sides {:.2}, {:.2}, {:.2}",
                self.get().side_a(),
                self.get().side_b(),
                self.get().side_c()
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
        fn prop_triangle_perimeter_correctness(
            side_a in 0.1f64..1000.0f64,
            side_b in 0.1f64..1000.0f64,
            side_c in 0.1f64..1000.0f64
        ) {
            prop_assume!(side_a + side_b > side_c);
            prop_assume!(side_a + side_c > side_b);
            prop_assume!(side_b + side_c > side_a);

            let triangle = Triangle::new(side_a, side_b, side_c, "test".to_string());
            let expected_perimeter = side_a + side_b + side_c;
            let actual_perimeter = triangle.perimeter();
            let epsilon = 1e-10;

            prop_assert!(
                (actual_perimeter - expected_perimeter).abs() < epsilon,
                "Triangle perimeter mismatch: expected {}, got {}",
                expected_perimeter,
                actual_perimeter
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_triangle_area_correctness_heron(
            side_a in 0.1f64..1000.0f64,
            side_b in 0.1f64..1000.0f64,
            side_c in 0.1f64..1000.0f64
        ) {
            prop_assume!(side_a + side_b > side_c);
            prop_assume!(side_a + side_c > side_b);
            prop_assume!(side_b + side_c > side_a);

            let triangle = Triangle::new(side_a, side_b, side_c, "test".to_string());
            let actual_area = triangle.area();

            prop_assert!(
                actual_area > 0.0,
                "Triangle area should be positive, but got {}",
                actual_area
            );

            let s = (side_a + side_b + side_c) / 2.0;
            let expected_area = (s * (s - side_a) * (s - side_b) * (s - side_c)).sqrt();

            let epsilon = 1e-8;
            prop_assert!(
                (actual_area - expected_area).abs() < epsilon,
                "Triangle area mismatch: expected {}, got {}",
                expected_area,
                actual_area
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_triangle_description_contains_info(
            side_a in 0.1f64..1000.0f64,
            side_b in 0.1f64..1000.0f64,
            side_c in 0.1f64..1000.0f64
        ) {
            prop_assume!(side_a + side_b > side_c);
            prop_assume!(side_a + side_c > side_b);
            prop_assume!(side_b + side_c > side_a);

            let triangle = Triangle::new(side_a, side_b, side_c, "test".to_string());
            let description = triangle.description();

            prop_assert!(
                description.contains("Triangle"),
                "Description should contain 'Triangle', but got: {}",
                description
            );

            let side_a_str = format!("{:.2}", side_a);
            prop_assert!(
                description.contains(&side_a_str),
                "Description should contain side_a value '{}', but got: {}",
                side_a_str,
                description
            );

            let side_b_str = format!("{:.2}", side_b);
            prop_assert!(
                description.contains(&side_b_str),
                "Description should contain side_b value '{}', but got: {}",
                side_b_str,
                description
            );

            let side_c_str = format!("{:.2}", side_c);
            prop_assert!(
                description.contains(&side_c_str),
                "Description should contain side_c value '{}', but got: {}",
                side_c_str,
                description
            );
        }
    }
}
