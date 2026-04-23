// FlyingFish class
//
// Flying fish class, inherits from Fish and mixes in Flyable and Swimmable

use classes::*;

use super::super::animal::Animal;
use super::super::mixins::{Flyable, Swimmable};
use super::Fish;

classes! {
    /// FlyingFish class
    ///
    /// Flying fish implementation, inherits from Fish and mixes in Flyable and Swimmable
    pub class FlyingFish extends Fish with Flyable, Swimmable {
        struct {
            // Glide distance (meters) - Copy type used directly
            pub glide_distance: f64 = 0.0,
        }

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
            let flying_fish: CRc<Self> = Self {
                super: Super::new(name, age, water_type, scale_pattern),
                glide_distance,
                ..
            };
            flying_fish.set_max_altitude(max_altitude);
            flying_fish.set_swim_speed(swim_speed);
            flying_fish
        }

        /// Override describe method
        pub override fn Animal::describe(&self) -> String {
            format!(
                "FlyingFish: {}, age {}, {} water, {} scales, glides {:.1}m, flies at {:.1}m altitude, swims at {:.1} m/s",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_water_type().as_ref().unwrap(),
                self.get_scale_pattern().as_ref().unwrap(),
                self.get_glide_distance(),
                self.get_max_altitude(),
                self.get_swim_speed()
            )
        }

        /// Flying fish-specific gliding behavior
        ///
        /// # Returns
        /// String describing the gliding behavior
        pub fn glide(&self) -> String {
            format!(
                "{} is gliding {:.1} meters above the water",
                self.get_name().as_ref().unwrap(),
                self.get_glide_distance()
            )
        }

        /// Flying fish leaping from water behavior
        ///
        /// # Returns
        /// String describing the leaping behavior
        pub fn leap_from_water(&self) -> String {
            format!(
                "{} leaps from the water and glides through the air",
                self.get_name().as_ref().unwrap()
            )
        }
    }
}

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

        assert_eq!(flying_fish.get_glide_distance(), 50.0);
        assert_eq!(flying_fish.get_max_altitude(), 3.0);
        assert_eq!(flying_fish.get_swim_speed(), 12.0);
    }
}
