// Shape abstract base class module

use classes::*;

classes! {
    /// Shape abstract base class
    /// Defines the common interface for all geometric shapes
    pub abstract class Shape {
        struct {
            pub final color: Option<String> = None,
        }

        /// Constructor: initialize color property
        pub fn new(color: String) -> Self {
            Self {
                color: Some(color),
            }
        }

        /// Abstract method: calculate shape area
        pub fn area(&self) -> f64;

        /// Abstract method: calculate shape perimeter
        pub fn perimeter(&self) -> f64;

        /// Abstract method: get shape description
        pub fn description(&self) -> String;
    }
}
