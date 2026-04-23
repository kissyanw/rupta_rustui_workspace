// ColoredCircle second-level derived class module

use super::Circle;
use crate::shape::Shape;
use classes::*;

classes! {
    /// ColoredCircle second-level derived class
    /// Represents a circle with fill color, inherits from Circle
    pub class ColoredCircle extends Circle {
        struct {
            pub final fill_color: Option<String> = None,
        }

        /// Constructor: create a circle with border color and fill color
        /// Parameters:
        /// - radius: radius of the circle
        /// - border_color: border color (inherited from Shape's color property)
        /// - fill_color: fill color
        pub fn new(radius: f64, border_color: String, fill_color: String) -> Self {
            Self {
                super: Super::new(radius, border_color),
                fill_color: Some(fill_color),
            }
        }

        /// Override: get ColoredCircle description
        /// Includes circle information, border color and fill color
        pub override fn Shape::description(&self) -> String {
            format!(
                "ColoredCircle with radius {:.2}, border color: {:?}, fill color: {:?}",
                self.get_radius(),
                self.get_color(),
                self.get_fill_color()
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
        fn prop_colored_circle_description_contains_dual_color_info(
            radius in 0.1f64..1000.0f64,
            border_color in "[a-z]{3,10}",
            fill_color in "[a-z]{3,10}"
        ) {
            let colored_circle = ColoredCircle::new(
                radius,
                border_color.clone(),
                fill_color.clone()
            );

            let description = colored_circle.description();

            prop_assert!(
                description.contains("ColoredCircle"),
                "Description should contain 'ColoredCircle', but got: {}",
                description
            );

            let radius_str = format!("{:.2}", radius);
            prop_assert!(
                description.contains(&radius_str),
                "Description should contain radius value '{}', but got: {}",
                radius_str,
                description
            );

            prop_assert!(
                description.contains(&border_color),
                "Description should contain border color '{}', but got: {}",
                border_color,
                description
            );

            prop_assert!(
                description.contains(&fill_color),
                "Description should contain fill color '{}', but got: {}",
                fill_color,
                description
            );
        }
    }
}
