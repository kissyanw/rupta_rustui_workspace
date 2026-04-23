// Salmon class
//
// Salmon class, inherits from Fish and mixes in Swimmable

use classes::*;

use super::super::animal::Animal;
use super::super::mixins::Swimmable;
use super::Fish;

classes! {
    /// Salmon class
    ///
    /// Salmon implementation, inherits from Fish and mixes in Swimmable
    pub class Salmon extends Fish with Swimmable {
        struct {
            // Spawning ground - non-Copy type uses final + Option
            pub final spawning_ground: Option<String> = None,
        }

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
            let salmon: CRc<Self> = Self {
                super: Super::new(name, age, water_type, scale_pattern),
                spawning_ground: Some(spawning_ground),
                ..
            };
            salmon.set_swim_speed(swim_speed);
            salmon
        }

        /// Override describe method
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Salmon: {}, age {}, {} water, {} scales, spawns at {}, swims at {:.1} m/s",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_water_type().as_ref().unwrap(),
                self.get_scale_pattern().as_ref().unwrap(),
                self.get_spawning_ground().as_ref().unwrap(),
                self.get_swim_speed()
            )
        }

        /// Salmon-specific migration behavior
        ///
        /// # Returns
        /// String describing the migration behavior
        pub fn migrate(&self) -> String {
            format!(
                "{} is migrating to {} to spawn",
                self.get_name().as_ref().unwrap(),
                self.get_spawning_ground().as_ref().unwrap()
            )
        }
    }
}

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
            salmon.get_spawning_ground().as_ref().unwrap(),
            "Alaska River"
        );
        assert_eq!(salmon.get_swim_speed(), 8.0);
    }
}
