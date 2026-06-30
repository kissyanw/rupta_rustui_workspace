// Animal abstract base class definition
//
// Defines common interfaces and properties for all animals

use oop_rs::prelude::*;

/// Animal abstract base class
///
/// Defines common interfaces and properties for all animals
/// All concrete animal classes must inherit from this class and implement its abstract methods
#[class(abstract, extends(Object))] // Add extends(Object) to enable downcasting
pub type Animal = class<
    {
        // Animal name - immutable by reference, public
        #[vis(pub)]
        let ref name: Option<String>;
        // Animal age - immutable by value (Copy type), public
        #[vis(pub)]
        let age: i32;

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
    },
>;
