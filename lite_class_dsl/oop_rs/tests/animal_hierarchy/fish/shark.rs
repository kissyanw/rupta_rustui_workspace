// Shark class
//
// Shark class, inherits from Fish and mixes in Swimmable

use oop_rs::prelude::*;

use super::super::animal::{Animal, IAnimal};
use super::super::mixins::{ISwimmable, Swimmable};
use super::{Fish, IFish};

/// Shark class
///
/// Shark implementation, inherits from Fish and mixes in Swimmable
#[class(extends(Fish), with(Swimmable))]
pub type Shark = class<
    {
        // Teeth count - Copy type, mutable, public
        #[vis(pub)]
        let mut teeth_count: usize;

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Shark's name
        /// * `age` - Shark's age
        /// * `water_type` - Water type (usually saltwater)
        /// * `scale_pattern` - Scale pattern
        /// * `teeth_count` - Teeth count
        /// * `swim_speed` - Swimming speed (meters/second)
        pub fn new(
            name: String,
            age: i32,
            water_type: String,
            scale_pattern: String,
            teeth_count: usize,
            swim_speed: f64,
        ) -> Self {
            let self = Self {
                teeth_count,
                ..Super::new(name, age, water_type, scale_pattern)
            };
            self.set().swim_speed(swim_speed);
            self
        }

        /// Override describe method
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "Shark: {}, age {}, {} water, {} scales, {} teeth, swims at {:.1} m/s",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().water_type().as_ref().unwrap(),
                self.get().scale_pattern().as_ref().unwrap(),
                self.get().teeth_count(),
                self.get().swim_speed()
            )
        }

        /// Shark-specific hunting behavior
        ///
        /// # Returns
        /// String describing the hunting behavior
        pub fn hunt(&self) -> String {
            format!(
                "{} is hunting with its {} sharp teeth",
                self.get().name().as_ref().unwrap(),
                self.get().teeth_count()
            )
        }
    },
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shark() {
        let shark = Shark::new(
            "Great White".to_string(),
            10,
            "saltwater".to_string(),
            "rough".to_string(),
            300,
            15.0,
        );

        println!("Shark created: {}", shark.describe());
        println!("Shark sound: {}", shark.make_sound());
        println!("Shark movement: {}", shark.move_action());
        println!("Shark swimming: {}", shark.swim());
        println!("Shark hunting: {}", shark.hunt());

        assert_eq!(shark.get().teeth_count(), 300);
        assert_eq!(shark.get().swim_speed(), 15.0);
    }
}
