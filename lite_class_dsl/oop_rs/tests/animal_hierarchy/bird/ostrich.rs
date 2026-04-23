// Ostrich class definition
//
// Ostrich class, inherits from Bird, cannot fly but excels at running

use classes::*;

use super::super::animal::Animal;
use super::Bird;

classes! {
    /// Ostrich class
    ///
    /// Ostrich class, inherits from Bird
    /// Cannot fly but is the fastest running bird in the world
    pub class Ostrich extends Bird {
        struct {
            // Running speed (kilometers/hour) - Copy type used directly
            pub running_speed: f64 = 0.0,
        }

        /// Constructor
        pub fn new(
            name: String,
            age: i32,
            wingspan: f64,
            feather_color: String,
            running_speed: f64,
        ) -> Self {
            Self {
                super: Super::new(name, age, wingspan, feather_color),
                running_speed,
            }
        }

        /// Override describe method to include Ostrich-specific information
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Ostrich: {}, age {}, wingspan {:.2}m, {} feathers, running speed {:.1} km/h",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_wingspan(),
                self.get_feather_color().as_ref().unwrap(),
                self.get_running_speed()
            )
        }

        /// Override move_action method, emphasizing running instead of flying
        pub override fn Animal::move_action(&self) -> String {
            format!(
                "{} runs at {:.1} km/h instead of flying",
                self.get_name().as_ref().unwrap(),
                self.get_running_speed()
            )
        }

        /// Override make_sound method, ostriches make booming sounds
        pub override fn Animal::make_sound(&self) -> String {
            format!("{} makes a booming sound", self.get_name().as_ref().unwrap())
        }

        /// Running behavior
        pub fn sprint(&self) -> String {
            format!(
                "{} sprints across the savanna at {:.1} km/h",
                self.get_name().as_ref().unwrap(),
                self.get_running_speed()
            )
        }

        /// Kicking defense behavior
        pub fn kick(&self) -> String {
            format!(
                "{} kicks with its powerful legs for defense",
                self.get_name().as_ref().unwrap()
            )
        }
    }
}
