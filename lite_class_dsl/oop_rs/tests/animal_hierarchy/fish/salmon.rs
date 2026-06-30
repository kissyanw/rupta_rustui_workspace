// Salmon class
//
// Salmon class, inherits from Fish and mixes in Swimmable

use oop_rs::prelude::*;

use super::super::animal::{Animal, IAnimal};
use super::super::mixins::{ISwimmable, Swimmable};
use super::{Fish, IFish};

/// Salmon class
///
/// Salmon implementation, inherits from Fish and mixes in Swimmable
#[class(extends(Fish), with(Swimmable))]
pub type Salmon = class<
    {
        // Spawning ground - immutable, public, non-Copy type
        #[vis(pub)]
        let ref spawning_ground: Option<String>;

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Salmon's name
        /// * `age` - Salmon's age
        /// * `water_type` - Water type (freshwater or saltwater)
        /// * `scale_pattern` - Scale pattern
        /// * `spawning_ground` - Spawning ground
        /// * `swim_speed` - Swimming speed (meters/second)
        pub fn new(
            name: String,
            age: i32,
            water_type: String,
            scale_pattern: String,
            spawning_ground: String,
            swim_speed: f64,
        ) -> Self {
            let self = Self {
                spawning_ground: Some(spawning_ground),
                ..Super::new(name, age, water_type, scale_pattern)
            };
            self.set().swim_speed(swim_speed);
            self
        }

        /// Override describe method
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "Salmon: {}, age {}, {} water, {} scales, spawns at {}, swims at {:.1} m/s",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().water_type().as_ref().unwrap(),
                self.get().scale_pattern().as_ref().unwrap(),
                self.get().spawning_ground().as_ref().unwrap(),
                self.get().swim_speed()
            )
        }

        /// Salmon-specific migration behavior
        ///
        /// # Returns
        /// String describing the migration behavior
        pub fn migrate(&self) -> String {
            format!(
                "{} is migrating to {} to spawn",
                self.get().name().as_ref().unwrap(),
                self.get().spawning_ground().as_ref().unwrap()
            )
        }
    },
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_salmon() {
        let salmon = Salmon::new(
            "Atlantic Salmon".to_string(),
            3,
            "freshwater".to_string(),
            "silver".to_string(),
            "Alaska River".to_string(),
            8.0,
        );

        println!("Salmon created: {}", salmon.describe());
        println!("Salmon sound: {}", salmon.make_sound());
        println!("Salmon movement: {}", salmon.move_action());
        println!("Salmon swimming: {}", salmon.swim());
        println!("Salmon migrating: {}", salmon.migrate());

        assert_eq!(
            salmon.get().spawning_ground().as_ref().unwrap(),
            "Alaska River"
        );
        assert_eq!(salmon.get().swim_speed(), 8.0);
    }
}
