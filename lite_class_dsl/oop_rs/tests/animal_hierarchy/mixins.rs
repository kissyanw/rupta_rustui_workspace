// Mixin definitions
//
// Contains all mixins: Feathered, Scaled, Flyable, Swimmable

use oop_rs::prelude::*;

use super::animal::{Animal, IAnimal};

/// Feathered mixin
///
/// Provides feather-related properties and behaviors for birds
#[class(on(Animal))]
pub type Feathered = mixin<
    {
        // Feather color - mutable to allow modification
        #[vis(pub)]
        let ref mut feather_color: Option<String> = None;

        /// Preening behavior
        ///
        /// # Returns
        /// String describing the preening behavior
        pub fn preen_feathers(&self) -> String {
            format!(
                "{} is preening its {} feathers",
                (self as &Animal).get().name().as_ref().unwrap(),
                self.get().feather_color().as_ref().unwrap()
            )
        }

        /// Change feather color
        ///
        /// # Parameters
        /// * `new_color` - New feather color
        pub fn change_feather_color(&self, new_color: String) {
            self.set().feather_color(Some(new_color));
        }
    },
>;

/// Scaled mixin
///
/// Provides scale-related properties and behaviors for fish
#[class(on(Animal))]
pub type Scaled = mixin<
    {
        // Scale pattern - mutable to allow modification
        #[vis(pub)]
        let ref mut scale_pattern: Option<String> = None;

        /// Scale shedding behavior
        ///
        /// # Returns
        /// String describing the scale shedding behavior
        pub fn shed_scales(&self) -> String {
            format!(
                "{} is shedding its {} scales",
                (self as &Animal).get().name().as_ref().unwrap(),
                self.get().scale_pattern().as_ref().unwrap()
            )
        }

        /// Change scale pattern
        ///
        /// # Parameters
        /// * `new_pattern` - New scale pattern
        pub fn change_scale_pattern(&self, new_pattern: String) {
            self.set().scale_pattern(Some(new_pattern));
        }
    },
>;

/// Flyable mixin
///
/// Provides flying capability for animals
#[class(on(Animal))]
pub type Flyable = mixin<
    {
        // Maximum flying altitude (meters) - Copy type, mutable
        #[vis(pub)]
        let mut max_altitude: f64 = 0.0;

        /// Flying behavior description
        ///
        /// # Returns
        /// String describing the flying behavior
        pub fn fly(&self) -> String {
            format!(
                "{} is flying at altitude {:.1}m",
                (self as &Animal).get().name().as_ref().unwrap(),
                self.get().max_altitude()
            )
        }

        /// Set maximum flying altitude
        ///
        /// # Parameters
        /// * `altitude` - New maximum flying altitude (meters)
        pub fn set_flying_altitude(&self, altitude: f64) {
            self.set().max_altitude(altitude);
        }

        /// Increase flying altitude
        ///
        /// # Parameters
        /// * `increment` - Altitude increase (meters)
        pub fn increase_altitude(&self, increment: f64) {
            let current = self.get().max_altitude();
            self.set().max_altitude(current + increment);
        }
    },
>;

/// Swimmable mixin
///
/// Provides swimming capability for animals
#[class(on(Animal))]
pub type Swimmable = mixin<
    {
        // Swimming speed (meters/second) - Copy type, mutable
        #[vis(pub)]
        let mut swim_speed: f64 = 0.0;

        /// Swimming behavior description
        ///
        /// # Returns
        /// String describing the swimming behavior
        pub fn swim(&self) -> String {
            format!(
                "{} is swimming at {:.1} m/s",
                (self as &Animal).get().name().as_ref().unwrap(),
                self.get().swim_speed()
            )
        }

        /// Set swimming speed
        ///
        /// # Parameters
        /// * `speed` - New swimming speed (meters/second)
        pub fn set_swimming_speed(&self, speed: f64) {
            self.set().swim_speed(speed);
        }

        /// Accelerate swimming
        ///
        /// # Parameters
        /// * `acceleration` - Speed increment (meters/second)
        pub fn accelerate(&self, acceleration: f64) {
            let current = self.get().swim_speed();
            self.set().swim_speed(current + acceleration);
        }

        /// Decelerate swimming
        ///
        /// # Parameters
        /// * `deceleration` - Speed decrement (meters/second)
        pub fn decelerate(&self, deceleration: f64) {
            let current = self.get().swim_speed();
            self.set().swim_speed((current - deceleration).max(0.0));
        }
    },
>;
