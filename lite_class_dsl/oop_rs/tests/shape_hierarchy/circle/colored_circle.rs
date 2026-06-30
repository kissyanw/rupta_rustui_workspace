// ColoredCircle second-level derived class module

use oop_rs::prelude::*;

use super::{Circle, ICircle};
use crate::shape::Shape;

#[class(extends(Circle))]
pub type ColoredCircle = class<
    {
        #[vis(pub)]
        let ref fill_color: Option<String>;

        pub fn new(radius: f64, border_color: String, fill_color: String) -> Self {
            Self {
                fill_color: Some(fill_color),
                ..Super::new(radius, border_color)
            }
        }

        #[method(override(Shape))]
        pub fn description(&self) -> String {
            format!(
                "ColoredCircle with radius {:.2}, border color: {:?}, fill color: {:?}",
                self.get().radius(),
                self.get().color(),
                self.get().fill_color()
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
