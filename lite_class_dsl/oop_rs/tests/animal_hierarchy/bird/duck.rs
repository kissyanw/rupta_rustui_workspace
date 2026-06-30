// Duck class definition
//
// Duck class, inherits from Bird and mixes in Flyable and Swimmable

use oop_rs::prelude::*;

use super::super::animal::Animal;
use super::super::mixins::{Flyable, IFlyable, ISwimmable, Swimmable};
use super::{Bird, IBird};

/// Duck class
///
/// Duck class, inherits from Bird and mixes in Flyable and Swimmable
/// Can both fly and swim, has migration capability
#[class(extends(Bird), with(Flyable, Swimmable))]
pub type Duck = class<
    {
        // Migration distance (kilometers) - Copy type, mutable, public
        #[vis(pub)]
        let mut migration_distance: f64;

        /// Constructor
        pub fn new(
            name: String,
            age: i32,
            wingspan: f64,
            feather_color: String,
            max_altitude: f64,
            swim_speed: f64,
            migration_distance: f64,
        ) -> Self {
            let self = Self {
                migration_distance,
                ..Super::new(name, age, wingspan, feather_color)
            };
            self.set().max_altitude(max_altitude);
            self.set().swim_speed(swim_speed);
            self
        }

        /// Override describe method to include Duck-specific information
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "Duck: {}, age {}, wingspan {:.2}m, {} feathers, max altitude {:.1}m, swim speed {:.1} m/s, migration distance {:.1} km",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().wingspan(),
                self.get().feather_color().as_ref().unwrap(),
                self.get().max_altitude(),
                self.get().swim_speed(),
                self.get().migration_distance()
            )
        }

        /// Override make_sound method, ducks quack
        #[method(override(Animal))]
        pub fn make_sound(&self) -> String {
            format!("{} quacks", self.get().name().as_ref().unwrap())
        }

        /// Migration behavior
        pub fn migrate(&self) -> String {
            format!(
                "{} is migrating {:.1} km",
                self.get().name().as_ref().unwrap(),
                self.get().migration_distance()
            )
        }

        /// Diving behavior
        pub fn dive(&self) -> String {
            format!(
                "{} dives underwater to find food",
                self.get().name().as_ref().unwrap()
            )
        }
    },
>;
