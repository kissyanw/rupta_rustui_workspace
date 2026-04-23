// Mixin definitions
//
// Contains all mixins: Feathered, Scaled, Flyable, Swimmable

use super::animal::Animal;
use super::bird::Bird;
use super::fish::Fish;
use classes::*;

classes! {
    /// Feathered mixin
    ///
    /// Provides feather-related properties and behaviors for birds
    #[with(Animal, Bird)]
    pub mixin Feathered on Animal {
        struct {
            // Feather color - uses pub mutable modifier to allow modification
            pub mutable feather_color: Option<String> = None,
        }

        /// Preening behavior
        ///
        /// # Returns
        /// String describing the preening behavior
        pub fn preen_feathers(&self) -> String {
            format!("{} is preening its {} feathers",
                    self.get_name().as_ref().unwrap(),
                    self.get_feather_color().as_ref().unwrap())
        }

        /// Change feather color
        ///
        /// # Parameters
        /// * `new_color` - New feather color
        pub fn change_feather_color(&self, new_color: String) {
            self.set_feather_color(Some(new_color));
        }
    }

    /// Scaled mixin
    ///
    /// Provides scale-related properties and behaviors for fish
    #[with(Animal, Fish)]
    pub mixin Scaled on Animal {
        struct {
            // Scale pattern - uses pub mutable modifier to allow modification
            pub mutable scale_pattern: Option<String> = None,
        }

        /// Scale shedding behavior
        ///
        /// # Returns
        /// String describing the scale shedding behavior
        pub fn shed_scales(&self) -> String {
            format!("{} is shedding its {} scales",
                    self.get_name().as_ref().unwrap(),
                    self.get_scale_pattern().as_ref().unwrap())
        }

        /// Change scale pattern
        ///
        /// # Parameters
        /// * `new_pattern` - New scale pattern
        pub fn change_scale_pattern(&self, new_pattern: String) {
            self.set_scale_pattern(Some(new_pattern));
        }
    }

    /// Flyable mixin
    ///
    /// Provides flying capability for animals
    #[with(Animal, Bird, Bird/Feathered, Fish)]
    pub mixin Flyable on Animal {
        struct {
            // Maximum flying altitude (meters) - Copy type used directly, pub makes it public
            pub max_altitude: f64 = 0.0,
        }

        /// Flying behavior description
        ///
        /// # Returns
        /// String describing the flying behavior
        pub fn fly(&self) -> String {
            format!("{} is flying at altitude {:.1}m",
                    self.get_name().as_ref().unwrap(),
                    self.get_max_altitude())
        }

        /// Set maximum flying altitude
        ///
        /// # Parameters
        /// * `altitude` - New maximum flying altitude (meters)
        pub fn set_flying_altitude(&self, altitude: f64) {
            self.set_max_altitude(altitude);
        }

        /// Increase flying altitude
        ///
        /// # Parameters
        /// * `increment` - Altitude increase (meters)
        pub fn increase_altitude(&self, increment: f64) {
            let current = self.get_max_altitude();
            self.set_max_altitude(current + increment);
        }
    }

    /// Swimmable mixin
    ///
    /// Provides swimming capability for animals
    #[with(Animal, Bird, Bird/Feathered, Bird/Flyable, Bird/Feathered/Flyable, Fish, Fish/Flyable)]
    pub mixin Swimmable on Animal {
        struct {
            // Swimming speed (meters/second) - Copy type used directly, pub makes it public
            pub swim_speed: f64 = 0.0,
        }

        /// Swimming behavior description
        ///
        /// # Returns
        /// String describing the swimming behavior
        pub fn swim(&self) -> String {
            format!("{} is swimming at {:.1} m/s",
                    self.get_name().as_ref().unwrap(),
                    self.get_swim_speed())
        }

        /// Set swimming speed
        ///
        /// # Parameters
        /// * `speed` - New swimming speed (meters/second)
        pub fn set_swimming_speed(&self, speed: f64) {
            self.set_swim_speed(speed);
        }

        /// Accelerate swimming
        ///
        /// # Parameters
        /// * `acceleration` - Speed increment (meters/second)
        pub fn accelerate(&self, acceleration: f64) {
            let current = self.get_swim_speed();
            self.set_swim_speed(current + acceleration);
        }

        /// Decelerate swimming
        ///
        /// # Parameters
        /// * `deceleration` - Speed decrement (meters/second)
        pub fn decelerate(&self, deceleration: f64) {
            let current = self.get_swim_speed();
            self.set_swim_speed((current - deceleration).max(0.0));
        }
    }
}
