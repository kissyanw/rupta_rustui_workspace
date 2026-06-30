// Eagle class definition

//
// Eagle class, inherits from Bird and mixes in Flyable

use oop_rs::prelude::*;

use super::super::animal::Animal;
use super::super::mixins::{Flyable, IFlyable};
use super::{Bird, IBird};

/// Eagle class
///
/// Eagle class, inherits from Bird and mixes in Flyable
/// Has flying capability and hunting territory
#[class(extends(Bird), with(Flyable))]
pub type Eagle = class<
    {
        // Hunting territory size (square kilometers) - Copy type, mutable, public
        #[vis(pub)]
        let mut hunting_territory_size: f64;

        /// Constructor
        pub fn new(
            name: String,
            age: i32,
            wingspan: f64,
            feather_color: String,
            max_altitude: f64,
            hunting_territory_size: f64,
        ) -> Self {
            let self = Self {
                hunting_territory_size,
                ..Super::new(name, age, wingspan, feather_color)
            };
            self.set().max_altitude(max_altitude);
            self
        }

        /// Override describe method to include Eagle-specific information
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "Eagle: {}, age {}, wingspan {:.2}m, {} feathers, max altitude {:.1}m, territory {:.2} km²",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().wingspan(),
                self.get().feather_color().as_ref().unwrap(),
                self.get().max_altitude(),
                self.get().hunting_territory_size()
            )
        }

        /// Hunting behavior
        pub fn hunt(&self) -> String {
            format!(
                "{} is hunting in its {:.2} km² territory",
                self.get().name().as_ref().unwrap(),
                self.get().hunting_territory_size()
            )
        }
    },
>;
