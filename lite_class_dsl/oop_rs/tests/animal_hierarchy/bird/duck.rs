// Duck class definition
//
// Duck class, inherits from Bird and mixes in Flyable and Swimmable

use classes::*;

use super::super::animal::Animal;
use super::super::mixins::{Flyable, Swimmable};
use super::Bird;

classes! {
    /// Duck class
    ///
    /// Duck class, inherits from Bird and mixes in Flyable and Swimmable
    /// Can both fly and swim, has migration capability
    pub class Duck extends Bird with Flyable, Swimmable {
        struct {
            // Migration distance (kilometers) - Copy type used directly
            pub migration_distance: f64 = 0.0,
        }

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
            let duck: CRc<Self> = Self {
                super: Super::new(name, age, wingspan, feather_color),
                migration_distance,
                ..
            };
            duck.set_max_altitude(max_altitude);
            duck.set_swim_speed(swim_speed);
            duck
        }

        /// Override describe method to include Duck-specific information
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Duck: {}, age {}, wingspan {:.2}m, {} feathers, max altitude {:.1}m, swim speed {:.1} m/s, migration distance {:.1} km",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_wingspan(),
                self.get_feather_color().as_ref().unwrap(),
                self.get_max_altitude(),
                self.get_swim_speed(),
                self.get_migration_distance()
            )
        }

        /// Override make_sound method, ducks quack
        pub override fn Animal::make_sound(&self) -> String {
            format!("{} quacks", self.get_name().as_ref().unwrap())
        }

        /// Migration behavior
        pub fn migrate(&self) -> String {
            format!(
                "{} is migrating {:.1} km",
                self.get_name().as_ref().unwrap(),
                self.get_migration_distance()
            )
        }

        /// Diving behavior
        pub fn dive(&self) -> String {
            format!(
                "{} dives underwater to find food",
                self.get_name().as_ref().unwrap()
            )
        }
    }
}
