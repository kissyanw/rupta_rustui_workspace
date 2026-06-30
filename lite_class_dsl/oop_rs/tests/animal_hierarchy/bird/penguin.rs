// Penguin class definition
//
// Penguin class, inherits from Bird and mixes in Swimmable

use oop_rs::prelude::*;

use super::super::animal::Animal;
use super::super::mixins::{ISwimmable, Swimmable};
use super::{Bird, IBird};

/// Penguin class
///
/// Penguin class, inherits from Bird and mixes in Swimmable
/// Cannot fly but excels at swimming, lives in colonies
#[class(extends(Bird), with(Swimmable))]
pub type Penguin = class<
    {
        // Colony size - Copy type, mutable, public
        #[vis(pub)]
        let mut colony_size: usize;

        /// Constructor
        pub fn new(
            name: String,
            age: i32,
            wingspan: f64,
            feather_color: String,
            swim_speed: f64,
            colony_size: usize,
        ) -> Self {
            let self = Self {
                colony_size,
                ..Super::new(name, age, wingspan, feather_color)
            };
            self.set().swim_speed(swim_speed);
            self
        }

        /// Override describe method to include Penguin-specific information
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "Penguin: {}, age {}, wingspan {:.2}m, {} feathers, swim speed {:.1} m/s, colony size {}",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().wingspan(),
                self.get().feather_color().as_ref().unwrap(),
                self.get().swim_speed(),
                self.get().colony_size()
            )
        }

        /// Override move_action method, penguins primarily move by swimming
        #[method(override(Animal))]
        pub fn move_action(&self) -> String {
            format!(
                "{} waddles on land and swims in water at {:.1} m/s",
                self.get().name().as_ref().unwrap(),
                self.get().swim_speed()
            )
        }

        /// Colony behavior
        pub fn huddle(&self) -> String {
            format!(
                "{} is huddling with {} other penguins",
                self.get().name().as_ref().unwrap(),
                self.get().colony_size()
            )
        }
    },
>;
