// Bicycle class definition
//
// Inherits from Vehicle (non-motorized), implements Drivable and Maintainable interfaces
// Represents a bicycle type vehicle

use super::interfaces::{Drivable, Maintainable};
use super::vehicle::Vehicle;
use classes::*;

classes! {
    /// Bicycle class
    ///
    /// Inherits from Vehicle (non-motorized), implements Drivable and Maintainable interfaces
    /// Represents a bicycle type vehicle
    pub class Bicycle extends Vehicle implements Drivable, Maintainable {
        struct {
            // Number of gears - Copy types can be used directly
            pub num_gears: i32,
            // Frame material - Non-Copy types use final + Option
            pub final frame_material: Option<String> = None,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `brand` - Vehicle brand
        /// * `year` - Vehicle year
        /// * `num_gears` - Number of gears
        /// * `frame_material` - Frame material
        pub fn new(
            brand: String,
            year: i32,
            num_gears: i32,
            frame_material: String,
        ) -> Self {
            Self {
                super: Super::new(brand, year),
                num_gears,
                frame_material: Some(frame_material),
            }
        }

        /// Implements Vehicle abstract method: returns vehicle type description
        pub override fn Vehicle::get_type(&self) -> String {
            "Bicycle".to_string()
        }

        /// Implements Vehicle abstract method: returns maximum speed (km/h)
        pub override fn Vehicle::max_speed(&self) -> f64 {
            40.0
        }

        // Drivable interface implementation

        /// Implements Drivable interface: drive vehicle
        pub override fn Drivable::drive(&self) -> String {
            format!(
                "Riding {} bicycle with {} gears",
                self.get_brand().as_ref().unwrap(),
                self.get_num_gears()
            )
        }

        /// Implements Drivable interface: stop vehicle
        pub override fn Drivable::stop(&self) -> String {
            "Bicycle stopped using brakes".to_string()
        }

        /// Implements Drivable interface: turn
        pub override fn Drivable::turn(&self, direction: String) -> String {
            format!("Bicycle turning {} by leaning", direction)
        }

        // Maintainable interface implementation

        /// Implements Maintainable interface: perform maintenance
        pub override fn Maintainable::perform_maintenance(&self) -> String {
            format!(
                "Performing maintenance on {} bicycle: chain lubrication, tire pressure check, brake adjustment",
                self.get_brand().as_ref().unwrap()
            )
        }

        /// Implements Maintainable interface: check vehicle condition
        pub override fn Maintainable::check_condition(&self) -> String {
            format!(
                "Bicycle condition: {} frame, {} gears functional",
                self.get_frame_material().as_ref().unwrap(),
                self.get_num_gears()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bicycle_instance_creation_and_field_access() {
        println!("\n=== Testing Bicycle instance creation and field access ===");

        // Create Bicycle instance
        let bicycle = Bicycle::new("Trek".to_string(), 2024, 21, "Carbon Fiber".to_string());

        // Test Bicycle own field access
        assert_eq!(
            bicycle.get_num_gears(),
            21,
            "Number of gears should be accessible and match the initialized value"
        );
        assert_eq!(
            bicycle.get_frame_material().as_ref().unwrap(),
            "Carbon Fiber",
            "Frame material should be accessible and match the initialized value"
        );

        // Test inherited field access from Vehicle
        assert_eq!(
            bicycle.get_brand().as_ref().unwrap(),
            "Trek",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            bicycle.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        println!("✓ Bicycle instance created successfully with all fields accessible");
    }

    #[test]
    fn test_bicycle_abstract_methods_implementation() {
        println!("\n=== Testing Bicycle abstract methods implementation ===");

        // Create Bicycle instance
        let bicycle = Bicycle::new("Giant".to_string(), 2023, 18, "Aluminum".to_string());

        // Test Vehicle abstract method implementation
        let vehicle_type = bicycle.get_type();
        assert_eq!(
            vehicle_type, "Bicycle",
            "get_type() should return 'Bicycle'"
        );

        let max_speed = bicycle.max_speed();
        assert_eq!(
            max_speed, 40.0,
            "max_speed() should return the expected speed"
        );
        assert!(
            max_speed > 0.0,
            "max_speed() should return a positive value"
        );

        println!("✓ Bicycle abstract methods are properly implemented");
    }

    #[test]
    fn test_bicycle_drivable_interface_implementation() {
        println!("\n=== Testing Bicycle Drivable interface implementation ===");

        // Create Bicycle instance
        let bicycle = Bicycle::new("Specialized".to_string(), 2022, 24, "Steel".to_string());

        // Test Drivable interface methods
        let drive_message = bicycle.drive();
        assert!(
            !drive_message.is_empty(),
            "drive() should return a non-empty message"
        );
        assert!(
            drive_message.contains("Specialized"),
            "drive() message should contain the brand"
        );
        assert!(
            drive_message.contains("24"),
            "drive() message should contain the number of gears"
        );

        let stop_message = bicycle.stop();
        assert!(
            !stop_message.is_empty(),
            "stop() should return a non-empty message"
        );

        let turn_left = bicycle.turn("left".to_string());
        assert!(
            !turn_left.is_empty(),
            "turn() should return a non-empty message"
        );
        assert!(
            turn_left.contains("left"),
            "turn() message should contain the direction"
        );

        let turn_right = bicycle.turn("right".to_string());
        assert!(
            turn_right.contains("right"),
            "turn() message should contain the direction"
        );

        println!("✓ Bicycle Drivable interface is properly implemented");
    }

    #[test]
    fn test_bicycle_maintainable_interface_implementation() {
        println!("\n=== Testing Bicycle Maintainable interface implementation ===");

        // Create Bicycle instance
        let bicycle = Bicycle::new("Cannondale".to_string(), 2025, 27, "Titanium".to_string());

        // Test Maintainable interface methods
        let maintenance_message = bicycle.perform_maintenance();
        assert!(
            !maintenance_message.is_empty(),
            "perform_maintenance() should return a non-empty message"
        );
        assert!(
            maintenance_message.contains("Cannondale"),
            "perform_maintenance() message should contain the brand"
        );

        let condition_message = bicycle.check_condition();
        assert!(
            !condition_message.is_empty(),
            "check_condition() should return a non-empty message"
        );
        assert!(
            condition_message.contains("Titanium"),
            "check_condition() message should contain the frame material"
        );
        assert!(
            condition_message.contains("27"),
            "check_condition() message should contain the number of gears"
        );

        println!("✓ Bicycle Maintainable interface is properly implemented");
    }

    #[test]
    fn test_bicycle_describe_method() {
        println!("\n=== Testing Bicycle describe() method ===");

        // Create Bicycle instance
        let bicycle = Bicycle::new("Bianchi".to_string(), 2024, 22, "Carbon Fiber".to_string());

        // Test describe() method (inherited from Vehicle)
        let description = bicycle.describe();
        println!("Description: {}", description);

        // Verify description contains all necessary information
        assert!(
            description.contains("Bianchi"),
            "Description should contain the brand"
        );
        assert!(
            description.contains("Bicycle"),
            "Description should contain the vehicle type"
        );
        assert!(
            description.contains("2024"),
            "Description should contain the year"
        );

        println!("✓ Bicycle describe() method works correctly");
    }

    #[test]
    fn test_bicycle_upcast_to_vehicle() {
        println!("\n=== Testing Bicycle upcast to Vehicle ===");

        // Create Bicycle instance
        let bicycle = Bicycle::new("Scott".to_string(), 2023, 20, "Aluminum".to_string());

        // Record original values
        let original_brand = bicycle.get_brand().as_ref().unwrap().clone();
        let original_year = bicycle.get_year();
        let original_type = bicycle.get_type();
        let original_speed = bicycle.max_speed();

        // Upcast to Vehicle
        let vehicle: CRc<Vehicle> = bicycle.into_super();

        // Verify fields and methods are still accessible after upcast
        assert_eq!(
            vehicle.get_brand().as_ref().unwrap(),
            &original_brand,
            "Brand should be preserved after upcast to Vehicle"
        );
        assert_eq!(
            vehicle.get_year(),
            original_year,
            "Year should be preserved after upcast to Vehicle"
        );
        assert_eq!(
            vehicle.get_type(),
            original_type,
            "Type should be preserved after upcast to Vehicle"
        );
        assert_eq!(
            vehicle.max_speed(),
            original_speed,
            "Max speed should be preserved after upcast to Vehicle"
        );

        println!("✓ Bicycle upcast to Vehicle preserves all fields and methods");
    }

    #[test]
    fn test_bicycle_interface_polymorphism() {
        println!("\n=== Testing Bicycle interface polymorphism ===");

        // Create Bicycle instance
        let bicycle = Bicycle::new("Merida".to_string(), 2024, 21, "Carbon Fiber".to_string());

        // Test through Drivable interface reference
        let drivable = bicycle.clone();
        let drive_msg = drivable.drive();
        assert!(
            !drive_msg.is_empty(),
            "Drivable interface should work through interface reference"
        );

        // Test through Maintainable interface reference
        let maintainable = bicycle.clone();
        let maintenance_msg = maintainable.perform_maintenance();
        assert!(
            !maintenance_msg.is_empty(),
            "Maintainable interface should work through interface reference"
        );

        println!("✓ Bicycle interface polymorphism works correctly");
    }
}
