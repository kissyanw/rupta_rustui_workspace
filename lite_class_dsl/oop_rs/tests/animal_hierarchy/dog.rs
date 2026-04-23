// Dog class definition
//
// Mammal class, directly inherits from Animal

use classes::*;

use super::animal::Animal;

classes! {
    /// Dog class
    ///
    /// Mammal, directly inherits from Animal
    /// Has specific properties like breed and fur color
    pub class Dog extends Animal {
        struct {
            // Dog's breed
            pub final breed: Option<String> = None,
            // Dog's fur color
            pub final fur_color: Option<String> = None,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Dog's name
        /// * `age` - Dog's age
        /// * `breed` - Dog's breed
        /// * `fur_color` - Dog's fur color
        pub fn new(name: String, age: i32, breed: String, fur_color: String) -> Self {
            Self {
                super: Super::new(name, age),
                breed: Some(breed),
                fur_color: Some(fur_color),
            }
        }

        /// Override make_sound method
        pub override fn Animal::make_sound(&self) -> String {
            format!("{} barks: Woof! Woof!", self.get_name().as_ref().unwrap())
        }

        /// Override move_action method
        pub override fn Animal::move_action(&self) -> String {
            format!("{} runs on four legs", self.get_name().as_ref().unwrap())
        }

        /// Override describe method
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Dog: {}, age {}, breed: {}, fur color: {}",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_breed().as_ref().unwrap(),
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
    fn test_create_dog() {
        // Test creating Dog instance
        let dog = Dog::new(
            "Buddy".to_string(),
            5,
            "Golden Retriever".to_string(),
            "Golden".to_string(),
        );

        // Verify field initialization
        assert_eq!(dog.get_name().as_ref().unwrap(), "Buddy");
        assert_eq!(dog.get_age(), 5);
        assert_eq!(dog.get_breed().as_ref().unwrap(), "Golden Retriever");
        assert_eq!(dog.get_fur_color().as_ref().unwrap(), "Golden");

        println!("Dog created successfully: {}", dog.describe());
    }

    #[test]
    fn test_dog_methods() {
        // Test Dog methods
        let dog = Dog::new(
            "Max".to_string(),
            4,
            "Labrador".to_string(),
            "Black".to_string(),
        );

        let sound = dog.make_sound();
        let movement = dog.move_action();
        let description = dog.describe();

        // Verify methods return non-empty strings
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // Verify returned content contains expected information
        assert!(sound.contains("Max"));
        assert!(sound.contains("Woof"));
        assert!(movement.contains("Max"));
        assert!(description.contains("Max"));
        assert!(description.contains("Labrador"));

        println!("Dog sound: {}", sound);
        println!("Dog movement: {}", movement);
        println!("Dog description: {}", description);
    }

    #[test]
    fn test_dog_upcast_to_animal() {
        println!("\n=== Testing Dog upcast to Animal ===");

        // Create Dog instance
        let dog = Dog::new(
            "Buddy".to_string(),
            5,
            "Golden Retriever".to_string(),
            "Golden".to_string(),
        );

        // Record original behavior
        let original_sound = dog.make_sound();
        let original_move = dog.move_action();
        let original_desc = dog.describe();

        println!("Original Dog - Sound: {}", original_sound);
        println!("Original Dog - Move: {}", original_move);
        println!("Original Dog - Desc: {}", original_desc);

        // Upcast to Animal
        let animal: CRc<Animal> = dog.into_superclass();

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
        assert_eq!(animal.get_name().as_ref().unwrap(), "Buddy");
        assert_eq!(animal.get_age(), 5);

        println!("✓ Dog upcast to Animal: behavior preserved");
    }
}
