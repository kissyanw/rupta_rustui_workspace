// FlyingFish class
//
// Flying fish class, inherits from Fish and mixes in Flyable and Swimmable

use oop_rs::prelude::*;

use super::super::animal::{Animal, IAnimal};
use super::super::mixins::{Flyable, IFlyable, ISwimmable, Swimmable};
use super::{Fish, IFish};

/// FlyingFish class
///
/// Flying fish implementation, inherits from Fish and mixes in Flyable and Swimmable
#[class(extends(Fish), with(Flyable, Swimmable))]
pub type FlyingFish = class<
    {
        // Glide distance (meters) - Copy type, mutable, public
        #[vis(pub)]
        let mut glide_distance: f64;

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Flying fish's name
        /// * `age` - Flying fish's age
        /// * `water_type` - Water type (usually saltwater)
        /// * `scale_pattern` - Scale pattern
        /// * `glide_distance` - Glide distance (meters)
        /// * `max_altitude` - Maximum flying altitude (meters)
        /// * `swim_speed` - Swimming speed (meters/second)
        pub fn new(
            name: String,
            age: i32,
            water_type: String,
            scale_pattern: String,
            glide_distance: f64,
            max_altitude: f64,
            swim_speed: f64,
        ) -> Self {
            let self = Self {
                glide_distance,
                ..Super::new(name, age, water_type, scale_pattern)
            };
            self.set().max_altitude(max_altitude);
            self.set().swim_speed(swim_speed);
            self
        }

        /// Override describe method
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "FlyingFish: {}, age {}, {} water, {} scales, glides {:.1}m, flies at {:.1}m altitude, swims at {:.1} m/s",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().water_type().as_ref().unwrap(),
                self.get().scale_pattern().as_ref().unwrap(),
                self.get().glide_distance(),
                self.get().max_altitude(),
                self.get().swim_speed()
            )
        }

        /// Flying fish-specific gliding behavior
        ///
        /// # Returns
        /// String describing the gliding behavior
        pub fn glide(&self) -> String {
            format!(
                "{} is gliding {:.1} meters above the water",
                self.get().name().as_ref().unwrap(),
                self.get().glide_distance()
            )
        }

        /// Flying fish leaping from water behavior
        ///
        /// # Returns
        /// String describing the leaping behavior
        pub fn leap_from_water(&self) -> String {
            format!(
                "{} leaps from the water and glides through the air",
                self.get().name().as_ref().unwrap()
            )
        }
    },
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_flying_fish() {
        let flying_fish = FlyingFish::new(
            "Pacific Flying Fish".to_string(),
            2,
            "saltwater".to_string(),
            "blue-silver".to_string(),
            50.0,
            3.0,
            12.0,
        );

        println!("FlyingFish created: {}", flying_fish.describe());
        println!("FlyingFish sound: {}", flying_fish.make_sound());
        println!("FlyingFish movement: {}", flying_fish.move_action());
        println!("FlyingFish swimming: {}", flying_fish.swim());
        println!("FlyingFish flying: {}", flying_fish.fly());
        println!("FlyingFish gliding: {}", flying_fish.glide());
        println!("FlyingFish leaping: {}", flying_fish.leap_from_water());

        assert_eq!(flying_fish.get().glide_distance(), 50.0);
        assert_eq!(flying_fish.get().max_altitude(), 3.0);
        assert_eq!(flying_fish.get().swim_speed(), 12.0);
    }
}
