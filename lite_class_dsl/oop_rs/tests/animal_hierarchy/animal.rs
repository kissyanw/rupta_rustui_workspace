// Animal abstract base class definition
//
// Defines common interfaces and properties for all animals

use classes::*;

classes! {
    /// Animal abstract base class
    ///
    /// Defines common interfaces and properties for all animals
    /// All concrete animal classes must inherit from this class and implement its abstract methods
    pub abstract class Animal {
        struct {
            // Animal name - uses pub final modifier to make getter method public
            pub final name: Option<String> = None,
            // Animal age - Copy type can be used directly, pub makes it public
            pub age: i32,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - The animal's name
        /// * `age` - The animal's age
        pub fn new(name: String, age: i32) -> Self {
            Self {
                name: Some(name),
                age,
            }
        }

        /// Abstract method: returns the animal's sound
        pub fn make_sound(&self) -> String;

        /// Abstract method: returns the animal's movement method
        pub fn move_action(&self) -> String;

        /// Abstract method: returns the animal's complete description
        pub fn describe(&self) -> String;
    }
}
