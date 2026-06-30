// Ostrich class definition
//
// Ostrich class, inherits from Bird, cannot fly but excels at running

use oop_rs::prelude::*;

use super::super::animal::Animal;
use super::{Bird, IBird};

/// Ostrich class
///
/// Ostrich class, inherits from Bird
/// Cannot fly but is the fastest running bird in the world
#[class(extends(Bird))]
pub type Ostrich = class<
    {
        // Running speed (kilometers/hour) - Copy type, mutable, public
        #[vis(pub)]
        let mut running_speed: f64;

        /// Constructor
        pub fn new(
            name: String,
            age: i32,
            wingspan: f64,
            feather_color: String,
            running_speed: f64,
        ) -> Self {
            Self {
                running_speed,
                ..Super::new(name, age, wingspan, feather_color)
            }
        }

        /// Override describe method to include Ostrich-specific information
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "Ostrich: {}, age {}, wingspan {:.2}m, {} feathers, running speed {:.1} km/h",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().wingspan(),
                self.get().feather_color().as_ref().unwrap(),
                self.get().running_speed()
            )
        }

        /// Override move_action method, emphasizing running instead of flying
        #[method(override(Animal))]
        pub fn move_action(&self) -> String {
            format!(
                "{} runs at {:.1} km/h instead of flying",
                self.get().name().as_ref().unwrap(),
                self.get().running_speed()
            )
        }

        /// Override make_sound method, ostriches make booming sounds
        #[method(override(Animal))]
        pub fn make_sound(&self) -> String {
            format!(
                "{} makes a booming sound",
                self.get().name().as_ref().unwrap()
            )
        }

        /// Running behavior
        pub fn sprint(&self) -> String {
            format!(
                "{} sprints across the savanna at {:.1} km/h",
                self.get().name().as_ref().unwrap(),
                self.get().running_speed()
            )
        }

        /// Kicking defense behavior
        pub fn kick(&self) -> String {
            format!(
                "{} kicks with its powerful legs for defense",
                self.get().name().as_ref().unwrap()
            )
        }
    },
>;
