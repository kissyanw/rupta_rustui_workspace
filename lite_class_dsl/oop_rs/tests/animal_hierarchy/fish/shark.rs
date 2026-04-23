// Shark class
//
// Shark class, inherits from Fish and mixes in Swimmable

use classes::*;

use super::super::animal::Animal;
use super::super::mixins::Swimmable;
use super::Fish;

classes! {
    /// Shark class
    ///
    /// Shark implementation, inherits from Fish and mixes in Swimmable
    pub class Shark extends Fish with Swimmable {
        struct {
            // Teeth count - Copy type used directly
            pub teeth_count: usize = 0,
        }

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
            let shark: CRc<Self> = Self {
                super: Super::new(name, age, water_type, scale_pattern),
                teeth_count,
                ..
            };
            shark.set_swim_speed(swim_speed);
            shark
        }

        /// Override describe method
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Shark: {}, age {}, {} water, {} scales, {} teeth, swims at {:.1} m/s",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_water_type().as_ref().unwrap(),
                self.get_scale_pattern().as_ref().unwrap(),
                self.get_teeth_count(),
                self.get_swim_speed()
            )
        }

        /// Shark-specific hunting behavior
        ///
        /// # Returns
        /// String describing the hunting behavior
        pub fn hunt(&self) -> String {
            format!(
                "{} is hunting with its {} sharp teeth",
                self.get_name().as_ref().unwrap(),
                self.get_teeth_count()
            )
        }
    }
}

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

        assert_eq!(shark.get_teeth_count(), 300);
        assert_eq!(shark.get_swim_speed(), 15.0);
    }
}
