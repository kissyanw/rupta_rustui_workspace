// Cat class definition
//
// Mammal class, directly inherits from Animal

use classes::*;

use super::animal::Animal;

classes! {
    /// Cat class
    ///
    /// Mammal, directly inherits from Animal
    /// Has specific properties like indoor/outdoor and fur color
    pub class Cat extends Animal {
        struct {
            // Whether this is an indoor cat
            pub indoor: bool,
            // Cat's fur color
            pub final fur_color: Option<String> = None,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Cat's name
        /// * `age` - Cat's age
        /// * `indoor` - Whether this is an indoor cat
        /// * `fur_color` - Cat's fur color
        pub fn new(name: String, age: i32, indoor: bool, fur_color: String) -> Self {
            Self {
                super: Super::new(name, age),
                indoor,
                fur_color: Some(fur_color),
            }
        }

        /// Override make_sound method
        pub override fn Animal::make_sound(&self) -> String {
            format!("{} meows: Meow! Meow!", self.get_name().as_ref().unwrap())
        }

        /// Override move_action method
        pub override fn Animal::move_action(&self) -> String {
            let location = if self.get_indoor() { "indoors" } else { "outdoors" };
            format!("{} prowls {}", self.get_name().as_ref().unwrap(), location)
        }

        /// Override describe method
        pub override fn Animal::describe(&self) -> String {
            let cat_type = if self.get_indoor() { "indoor" } else { "outdoor" };
            format!(
                "Cat: {}, age {}, {} cat, fur color: {}",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                cat_type,
                self.get_fur_color().as_ref().unwrap()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Animal;
    use classes::prelude::*;

    #[test]
    fn test_create_cat() {
        // Test creating Cat instance
        let cat = Cat::new("Whiskers".to_string(), 3, true, "Gray".to_string());

        // Verify field initialization
        assert_eq!(cat.get_name().as_ref().unwrap(), "Whiskers");
        assert_eq!(cat.get_age(), 3);
        assert_eq!(cat.get_indoor(), true);
        assert_eq!(cat.get_fur_color().as_ref().unwrap(), "Gray");

        println!("Cat created successfully: {}", cat.describe());
    }

    #[test]
    fn test_cat_methods() {
        // Test Cat methods
        let cat = Cat::new("Luna".to_string(), 2, false, "White".to_string());

        let sound = cat.make_sound();
        let movement = cat.move_action();
        let description = cat.describe();

        // Verify methods return non-empty strings
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // Verify returned content contains expected information
        assert!(sound.contains("Luna"));
        assert!(sound.contains("Meow"));
        assert!(movement.contains("Luna"));
        assert!(movement.contains("outdoors"));
        assert!(description.contains("Luna"));
        assert!(description.contains("outdoor"));

        println!("Cat sound: {}", sound);
        println!("Cat movement: {}", movement);
        println!("Cat description: {}", description);
    }

    #[test]
    fn test_cat_upcast_to_animal() {
        println!("\n=== Testing Cat upcast to Animal ===");

        // Create Cat instance
        let cat = Cat::new("Whiskers".to_string(), 3, true, "Gray".to_string());

        // Record original behavior
        let original_sound = cat.make_sound();
        let original_move = cat.move_action();
        let original_desc = cat.describe();

        println!("Original Cat - Sound: {}", original_sound);
        println!("Original Cat - Move: {}", original_move);
        println!("Original Cat - Desc: {}", original_desc);

        // Upcast to Animal
        let animal: CRc<Animal> = cat.into_superclass();

        // Call methods through Animal reference
        let animal_sound = animal.make_sound();
        let animal_move = animal.move_action();
        let animal_desc = animal.describe();

        println!("After upcast - Sound: {}", animal_sound);
        println!("After upcast - Move: {}", animal_move);
        println!("After upcast - Desc: {}", animal_desc);

        // Verify behavior remains the same
        assert_eq!(
            original_sound, animal_sound,
            "make_sound should remain the same after upcast"
        );
        assert_eq!(
            original_move, animal_move,
            "move_action should remain the same after upcast"
        );
        assert_eq!(
            original_desc, animal_desc,
            "describe should remain the same after upcast"
        );

        // Verify field values remain the same
        assert_eq!(animal.get_name().as_ref().unwrap(), "Whiskers");
        assert_eq!(animal.get_age(), 3);

        println!("✓ Cat upcast to Animal: behavior preserved");
    }
}
