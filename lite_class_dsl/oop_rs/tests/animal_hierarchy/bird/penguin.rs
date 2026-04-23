// Penguin class definition
//
// Penguin class, inherits from Bird and mixes in Swimmable

use classes::*;

use super::super::animal::Animal;
use super::super::mixins::Swimmable;
use super::Bird;

classes! {
    /// Penguin class
    ///
    /// Penguin class, inherits from Bird and mixes in Swimmable
    /// Cannot fly but excels at swimming, lives in colonies
    pub class Penguin extends Bird with Swimmable {
        struct {
            // Colony size - Copy type used directly
            pub colony_size: usize = 0,
        }

        /// Constructor
        pub fn new(
            name: String,
            age: i32,
            wingspan: f64,
            feather_color: String,
            swim_speed: f64,
            colony_size: usize,
        ) -> Self {
            let penguin: CRc<Self> = Self {
                super: Super::new(name, age, wingspan, feather_color),
                colony_size,
                ..
            };
            penguin.set_swim_speed(swim_speed);
            penguin
        }

        /// Override describe method to include Penguin-specific information
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Penguin: {}, age {}, wingspan {:.2}m, {} feathers, swim speed {:.1} m/s, colony size {}",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_wingspan(),
                self.get_feather_color().as_ref().unwrap(),
                self.get_swim_speed(),
                self.get_colony_size()
            )
        }

        /// Override move_action method, penguins primarily move by swimming
        pub override fn Animal::move_action(&self) -> String {
            format!(
                "{} waddles on land and swims in water at {:.1} m/s",
                self.get_name().as_ref().unwrap(),
                self.get_swim_speed()
            )
        }

        /// Colony behavior
        pub fn huddle(&self) -> String {
            format!(
                "{} is huddling with {} other penguins",
                self.get_name().as_ref().unwrap(),
                self.get_colony_size()
            )
        }
    }
}
