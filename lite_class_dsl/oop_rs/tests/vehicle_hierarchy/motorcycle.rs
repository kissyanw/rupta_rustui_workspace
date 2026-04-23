// Motorcycle class definition
//
// Inherits from MotorVehicle, implements Drivable and Maintainable interfaces
// Represents a motorcycle type vehicle

use super::interfaces::{Drivable, Maintainable};
use super::motor_vehicle::MotorVehicle;
use super::vehicle::Vehicle;
use classes::*;

classes! {
    /// Motorcycle class
    ///
    /// Inherits from MotorVehicle, implements Drivable and Maintainable interfaces
    /// Represents a motorcycle type vehicle
    pub class Motorcycle extends MotorVehicle implements Drivable, Maintainable {
        struct {
            // Has sidecar - Copy types can be used directly
            pub has_sidecar: bool,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `brand` - Vehicle brand
        /// * `year` - Vehicle year
        /// * `engine_type` - Engine type
        /// * `fuel_capacity` - Fuel capacity (liters)
        /// * `has_sidecar` - Has sidecar
        pub fn new(
            brand: String,
            year: i32,
            engine_type: String,
            fuel_capacity: f64,
            has_sidecar: bool,
        ) -> Self {
            Self {
                super: Super::new(brand, year, engine_type, fuel_capacity),
                has_sidecar,
            }
        }

        /// Implements Vehicle abstract method: returns vehicle type description
        pub override fn Vehicle::get_type(&self) -> String {
            "Motorcycle".to_string()
        }

        /// Implements Vehicle abstract method: returns maximum speed (km/h)
        pub override fn Vehicle::max_speed(&self) -> f64 {
            200.0
        }

        /// Implements MotorVehicle abstract method: returns engine start description
        pub override fn MotorVehicle::start_engine(&self) -> String {
            format!(
                "Starting {} motorcycle engine with a roar",
                self.get_engine_type().as_ref().unwrap()
            )
        }

        /// Implements MotorVehicle abstract method: returns fuel efficiency (km/L)
        pub override fn MotorVehicle::fuel_efficiency(&self) -> f64 {
            25.0
        }

        // Drivable interface implementation

        /// Implements Drivable interface: drive vehicle
        pub override fn Drivable::drive(&self) -> String {
            if self.get_has_sidecar() {
                format!(
                    "Riding {} motorcycle with sidecar",
                    self.get_brand().as_ref().unwrap()
                )
            } else {
                format!(
                    "Riding {} motorcycle",
                    self.get_brand().as_ref().unwrap()
                )
            }
        }

        /// Implements Drivable interface: stop vehicle
        pub override fn Drivable::stop(&self) -> String {
            "Motorcycle stopped".to_string()
        }

        /// Implements Drivable interface: turn
        pub override fn Drivable::turn(&self, direction: String) -> String {
            format!("Motorcycle leaning {}", direction)
        }

        // Maintainable interface implementation

        /// Implements Maintainable interface: perform maintenance
        pub override fn Maintainable::perform_maintenance(&self) -> String {
            format!(
                "Performing maintenance on {} motorcycle: chain lubrication, tire pressure check, brake inspection",
                self.get_brand().as_ref().unwrap()
            )
        }

        /// Implements Maintainable interface: check vehicle condition
        pub override fn Maintainable::check_condition(&self) -> String {
            let sidecar_status = if self.get_has_sidecar() {
                "with sidecar"
            } else {
                "without sidecar"
            };
            format!(
                "Motorcycle condition: Engine {}, {} - Good",
                self.get_engine_type().as_ref().unwrap(),
                sidecar_status
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motorcycle_instance_creation_and_field_access() {
        println!("\n=== Testing Motorcycle instance creation and field access ===");

        // Create Motorcycle instance（无边车）
        let motorcycle = Motorcycle::new(
            "Harley-Davidson".to_string(),
            2024,
            "V-Twin".to_string(),
            20.0,
            false,
        );

        // Test Motorcycle own field access
        assert_eq!(
            motorcycle.get_has_sidecar(),
            false,
            "has_sidecar should be accessible and match the initialized value"
        );

        // Test inherited field access from MotorVehicle
        assert_eq!(
            motorcycle.get_engine_type().as_ref().unwrap(),
            "V-Twin",
            "Engine type should be accessible and match the initialized value"
        );
        assert_eq!(
            motorcycle.get_fuel_capacity(),
            20.0,
            "Fuel capacity should be accessible and match the initialized value"
        );

        // Test inherited field access from Vehicle
        assert_eq!(
            motorcycle.get_brand().as_ref().unwrap(),
            "Harley-Davidson",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            motorcycle.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        println!("✓ Motorcycle instance created successfully with all fields accessible");
    }

    #[test]
    fn test_motorcycle_with_sidecar() {
        println!("\n=== Testing Motorcycle with sidecar ===");

        // Create Motorcycle instance with sidecar
        let motorcycle = Motorcycle::new("Ural".to_string(), 2023, "Boxer".to_string(), 18.0, true);

        // Verify sidecar field
        assert_eq!(
            motorcycle.get_has_sidecar(),
            true,
            "has_sidecar should be true for motorcycle with sidecar"
        );

        // Verify drive message contains sidecar information
        let drive_message = motorcycle.drive();
        assert!(
            drive_message.contains("sidecar"),
            "Drive message should mention sidecar when has_sidecar is true"
        );

        println!("✓ Motorcycle with sidecar works correctly");
    }

    #[test]
    fn test_motorcycle_abstract_methods_implementation() {
        println!("\n=== Testing Motorcycle abstract methods implementation ===");

        // Create Motorcycle instance
        let motorcycle = Motorcycle::new(
            "Yamaha".to_string(),
            2023,
            "Inline-4".to_string(),
            17.0,
            false,
        );

        // Test Vehicle abstract method implementation
        let vehicle_type = motorcycle.get_type();
        assert_eq!(
            vehicle_type, "Motorcycle",
            "get_type() should return 'Motorcycle'"
        );

        let max_speed = motorcycle.max_speed();
        assert_eq!(
            max_speed, 200.0,
            "max_speed() should return the expected speed"
        );
        assert!(
            max_speed > 0.0,
            "max_speed() should return a positive value"
        );

        // Test MotorVehicle abstract method implementation
        let start_message = motorcycle.start_engine();
        assert!(
            !start_message.is_empty(),
            "start_engine() should return a non-empty message"
        );
        assert!(
            start_message.contains("Inline-4"),
            "start_engine() message should contain the engine type"
        );
        assert!(
            start_message.contains("roar"),
            "start_engine() message should contain 'roar' for motorcycle"
        );

        let efficiency = motorcycle.fuel_efficiency();
        assert_eq!(
            efficiency, 25.0,
            "fuel_efficiency() should return the expected efficiency"
        );
        assert!(
            efficiency > 0.0,
            "fuel_efficiency() should return a positive value"
        );

        println!("✓ Motorcycle abstract methods are properly implemented");
    }

    #[test]
    fn test_motorcycle_drivable_interface_implementation() {
        println!("\n=== Testing Motorcycle Drivable interface implementation ===");

        // Create Motorcycle instance（无边车）
        let motorcycle = Motorcycle::new(
            "Kawasaki".to_string(),
            2022,
            "Parallel-Twin".to_string(),
            15.0,
            false,
        );

        // Test Drivable interface methods
        let drive_message = motorcycle.drive();
        assert!(
            !drive_message.is_empty(),
            "drive() should return a non-empty message"
        );
        assert!(
            drive_message.contains("Kawasaki"),
            "drive() message should contain the brand"
        );
        assert!(
            !drive_message.contains("sidecar"),
            "drive() message should not mention sidecar when has_sidecar is false"
        );

        let stop_message = motorcycle.stop();
        assert!(
            !stop_message.is_empty(),
            "stop() should return a non-empty message"
        );

        let turn_left = motorcycle.turn("left".to_string());
        assert!(
            !turn_left.is_empty(),
            "turn() should return a non-empty message"
        );
        assert!(
            turn_left.contains("left"),
            "turn() message should contain the direction"
        );
        assert!(
            turn_left.contains("leaning"),
            "turn() message should contain 'leaning' for motorcycle"
        );

        let turn_right = motorcycle.turn("right".to_string());
        assert!(
            turn_right.contains("right"),
            "turn() message should contain the direction"
        );

        println!("✓ Motorcycle Drivable interface is properly implemented");
    }

    #[test]
    fn test_motorcycle_maintainable_interface_implementation() {
        println!("\n=== Testing Motorcycle Maintainable interface implementation ===");

        // Create Motorcycle instance
        let motorcycle = Motorcycle::new(
            "Ducati".to_string(),
            2025,
            "L-Twin".to_string(),
            16.0,
            false,
        );

        // Test Maintainable interface methods
        let maintenance_message = motorcycle.perform_maintenance();
        assert!(
            !maintenance_message.is_empty(),
            "perform_maintenance() should return a non-empty message"
        );
        assert!(
            maintenance_message.contains("Ducati"),
            "perform_maintenance() message should contain the brand"
        );
        assert!(
            maintenance_message.contains("chain"),
            "perform_maintenance() message should mention chain lubrication"
        );

        let condition_message = motorcycle.check_condition();
        assert!(
            !condition_message.is_empty(),
            "check_condition() should return a non-empty message"
        );
        assert!(
            condition_message.contains("L-Twin"),
            "check_condition() message should contain the engine type"
        );
        assert!(
            condition_message.contains("without sidecar"),
            "check_condition() message should indicate no sidecar"
        );

        println!("✓ Motorcycle Maintainable interface is properly implemented");
    }

    #[test]
    fn test_motorcycle_describe_method() {
        println!("\n=== Testing Motorcycle describe() method ===");

        // Create Motorcycle instance
        let motorcycle = Motorcycle::new("BMW".to_string(), 2024, "Boxer".to_string(), 19.0, true);

        // Test describe() method (inherited from MotorVehicle)
        let description = motorcycle.describe();
        println!("Description: {}", description);

        // Verify description contains all necessary information
        assert!(
            description.contains("BMW"),
            "Description should contain the brand"
        );
        assert!(
            description.contains("Motorcycle"),
            "Description should contain the vehicle type"
        );
        assert!(
            description.contains("2024"),
            "Description should contain the year"
        );
        assert!(
            description.contains("Boxer"),
            "Description should contain the engine type"
        );
        assert!(
            description.contains("19.0"),
            "Description should contain the fuel capacity"
        );

        println!("✓ Motorcycle describe() method works correctly");
    }

    #[test]
    fn test_motorcycle_upcast_to_motor_vehicle() {
        println!("\n=== Testing Motorcycle upcast to MotorVehicle ===");

        // Create Motorcycle instance
        let motorcycle = Motorcycle::new(
            "Suzuki".to_string(),
            2023,
            "Inline-4".to_string(),
            17.5,
            false,
        );

        // Record original values
        let original_brand = motorcycle.get_brand().as_ref().unwrap().clone();
        let original_type = motorcycle.get_type();
        let original_speed = motorcycle.max_speed();

        // Upcast to MotorVehicle
        let motor_vehicle: CRc<MotorVehicle> = motorcycle.clone().into_super();

        // Verify fields and methods are still accessible after upcast
        assert_eq!(
            motor_vehicle.get_brand().as_ref().unwrap(),
            &original_brand,
            "Brand should be preserved after upcast to MotorVehicle"
        );
        assert_eq!(
            motor_vehicle.get_type(),
            original_type,
            "Type should be preserved after upcast to MotorVehicle"
        );
        assert_eq!(
            motor_vehicle.max_speed(),
            original_speed,
            "Max speed should be preserved after upcast to MotorVehicle"
        );

        println!("✓ Motorcycle upcast to MotorVehicle preserves all fields and methods");
    }

    #[test]
    fn test_motorcycle_upcast_to_vehicle() {
        println!("\n=== Testing Motorcycle upcast to Vehicle ===");

        // Create Motorcycle instance
        let motorcycle =
            Motorcycle::new("Honda".to_string(), 2022, "Single".to_string(), 12.0, false);

        // Record original values
        let original_brand = motorcycle.get_brand().as_ref().unwrap().clone();
        let original_year = motorcycle.get_year();

        // Upcast to MotorVehicle，然后到 Vehicle
        let motor_vehicle: CRc<MotorVehicle> = motorcycle.clone().into_super();
        let vehicle: CRc<Vehicle> = motor_vehicle.into_super().into();

        // Verify fields are still accessible after multi-level upcast
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

        println!("✓ Motorcycle upcast to Vehicle preserves all fields");
    }

    #[test]
    fn test_motorcycle_interface_polymorphism() {
        println!("\n=== Testing Motorcycle interface polymorphism ===");

        // Create Motorcycle instance
        let motorcycle = Motorcycle::new(
            "Triumph".to_string(),
            2024,
            "Triple".to_string(),
            18.0,
            false,
        );

        // Test through Drivable interface reference
        let drivable = motorcycle.clone();
        let drive_msg = drivable.drive();
        assert!(
            !drive_msg.is_empty(),
            "Drivable interface should work through interface reference"
        );

        // Test through Maintainable interface reference
        let maintainable = motorcycle.clone();
        let maintenance_msg = maintainable.perform_maintenance();
        assert!(
            !maintenance_msg.is_empty(),
            "Maintainable interface should work through interface reference"
        );

        println!("✓ Motorcycle interface polymorphism works correctly");
    }
}
