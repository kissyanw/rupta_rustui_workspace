// Eagle class definition
//
// Eagle class, inherits from Bird and mixes in Flyable

use classes::*;

use super::super::animal::Animal;
use super::super::mixins::Flyable;
use super::Bird;

classes! {
    /// Eagle class
    ///
    /// Eagle class, inherits from Bird and mixes in Flyable
    /// Has flying capability and hunting territory
    pub class Eagle extends Bird with Flyable {
        struct {
            // Hunting territory size (square kilometers) - Copy type used directly
            pub hunting_territory_size: f64 = 0.0,
        }

        /// Constructor
        pub fn new(
            name: String,
            age: i32,
            wingspan: f64,
            feather_color: String,
            max_altitude: f64,
            hunting_territory_size: f64,
        ) -> Self {
            let eagle: CRc<Self> = Self {
                super: Super::new(name, age, wingspan, feather_color),
                hunting_territory_size,
                ..
            };
            eagle.set_max_altitude(max_altitude);
            eagle
        }

        /// Override describe method to include Eagle-specific information
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Eagle: {}, age {}, wingspan {:.2}m, {} feathers, max altitude {:.1}m, territory {:.2} km²",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_wingspan(),
                self.get_feather_color().as_ref().unwrap(),
                self.get_max_altitude(),
                self.get_hunting_territory_size()
            )
        }

        /// Hunting behavior
        pub fn hunt(&self) -> String {
            format!(
                "{} is hunting in its {:.2} km² territory",
                self.get_name().as_ref().unwrap(),
                self.get_hunting_territory_size()
            )
        }
    }
}
