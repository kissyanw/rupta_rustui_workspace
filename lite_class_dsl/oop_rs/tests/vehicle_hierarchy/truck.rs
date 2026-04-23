// Truck class definition
//
// Inherits from MotorVehicle, implements Drivable and Maintainable interfaces
// Represents a truck type vehicle

use super::interfaces::{Drivable, Maintainable};
use super::motor_vehicle::MotorVehicle;
use super::vehicle::Vehicle;
use classes::*;

classes! {
    /// Truck class
    ///
    /// Inherits from MotorVehicle, implements Drivable and Maintainable interfaces
    /// Represents a truck type vehicle
    pub class Truck extends MotorVehicle implements Drivable, Maintainable {
        struct {
            // Cargo capacity (tons) - Copy types can be used directly
            pub cargo_capacity: f64,
            // Number of axles - Copy types can be used directly
            pub num_axles: i32,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `brand` - Vehicle brand
        /// * `year` - Vehicle year
        /// * `engine_type` - Engine type
        /// * `fuel_capacity` - Fuel capacity (liters)
        /// * `cargo_capacity` - Cargo capacity (tons)
        /// * `num_axles` - Number of axles
        pub fn new(
            brand: String,
            year: i32,
            engine_type: String,
            fuel_capacity: f64,
            cargo_capacity: f64,
            num_axles: i32,
        ) -> Self {
            Self {
                super: Super::new(brand, year, engine_type, fuel_capacity),
                cargo_capacity,
                num_axles,
            }
        }

        /// Implements Vehicle abstract method: returns vehicle type description
        pub override fn Vehicle::get_type(&self) -> String {
            "Truck".to_string()
        }

        /// Implements Vehicle abstract method: returns maximum speed (km/h)
        pub override fn Vehicle::max_speed(&self) -> f64 {
            120.0
        }

        /// Implements MotorVehicle abstract method: returns engine start description
        pub override fn MotorVehicle::start_engine(&self) -> String {
            format!(
                "Starting {} truck engine with {} axles",
                self.get_engine_type().as_ref().unwrap(),
                self.get_num_axles()
            )
        }

        /// Implements MotorVehicle abstract method: returns fuel efficiency (km/L)
        pub override fn MotorVehicle::fuel_efficiency(&self) -> f64 {
            8.0
        }

        // Drivable interface implementation

        /// Implements Drivable interface: drive vehicle
        pub override fn Drivable::drive(&self) -> String {
            format!(
                "Driving {} truck with {:.1} ton cargo capacity",
                self.get_brand().as_ref().unwrap(),
                self.get_cargo_capacity()
            )
        }

        /// Implements Drivable interface: stop vehicle
        pub override fn Drivable::stop(&self) -> String {
            "Truck stopped with air brakes".to_string()
        }

        /// Implements Drivable interface: turn
        pub override fn Drivable::turn(&self, direction: String) -> String {
            format!("Truck turning {} with wide radius", direction)
        }

        // Maintainable interface implementation

        /// Implements Maintainable interface: perform maintenance
        pub override fn Maintainable::perform_maintenance(&self) -> String {
            format!(
                "Performing maintenance on {} truck: checking {} axles, brake system, cargo bed inspection",
                self.get_brand().as_ref().unwrap(),
                self.get_num_axles()
            )
        }

        /// Implements Maintainable interface: check vehicle condition
        pub override fn Maintainable::check_condition(&self) -> String {
            format!(
                "Truck condition: Engine {}, {} axles, cargo capacity {:.1} tons - Good",
                self.get_engine_type().as_ref().unwrap(),
                self.get_num_axles(),
                self.get_cargo_capacity()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truck_instance_creation_and_field_access() {
        println!("\n=== Testing Truck instance creation and field access ===");

        // Create Truck instance
        let truck = Truck::new(
            "Volvo".to_string(),
            2024,
            "Diesel".to_string(),
            300.0,
            20.0,
            3,
        );

        // Test Truck own field access
        assert_eq!(
            truck.get_cargo_capacity(),
            20.0,
            "Cargo capacity should be accessible and match the initialized value"
        );
        assert_eq!(
            truck.get_num_axles(),
            3,
            "Number of axles should be accessible and match the initialized value"
        );

        // Test inherited field access from MotorVehicle
        assert_eq!(
            truck.get_engine_type().as_ref().unwrap(),
            "Diesel",
            "Engine type should be accessible and match the initialized value"
        );
        assert_eq!(
            truck.get_fuel_capacity(),
            300.0,
            "Fuel capacity should be accessible and match the initialized value"
        );

        // Test inherited field access from Vehicle
        assert_eq!(
            truck.get_brand().as_ref().unwrap(),
            "Volvo",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            truck.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        println!("✓ Truck instance created successfully with all fields accessible");
    }

    #[test]
    fn test_truck_abstract_methods_implementation() {
        println!("\n=== Testing Truck abstract methods implementation ===");

        // Create Truck instance
        let truck = Truck::new(
            "Scania".to_string(),
            2023,
            "Turbo Diesel".to_string(),
            400.0,
            25.0,
            4,
        );

        // Test Vehicle abstract method implementation
        let vehicle_type = truck.get_type();
        assert_eq!(vehicle_type, "Truck", "get_type() should return 'Truck'");

        let max_speed = truck.max_speed();
        assert_eq!(
            max_speed, 120.0,
            "max_speed() should return the expected speed"
        );
        assert!(
            max_speed > 0.0,
            "max_speed() should return a positive value"
        );

        // Test MotorVehicle abstract method implementation
        let start_message = truck.start_engine();
        assert!(
            !start_message.is_empty(),
            "start_engine() should return a non-empty message"
        );
        assert!(
            start_message.contains("Turbo Diesel"),
            "start_engine() message should contain the engine type"
        );
        assert!(
            start_message.contains("4"),
            "start_engine() message should contain the number of axles"
        );

        let efficiency = truck.fuel_efficiency();
        assert_eq!(
            efficiency, 8.0,
            "fuel_efficiency() should return the expected efficiency"
        );
        assert!(
            efficiency > 0.0,
            "fuel_efficiency() should return a positive value"
        );

        println!("✓ Truck abstract methods are properly implemented");
    }

    #[test]
    fn test_truck_drivable_interface_implementation() {
        println!("\n=== Testing Truck Drivable interface implementation ===");

        // Create Truck instance
        let truck = Truck::new(
            "MAN".to_string(),
            2022,
            "Heavy Duty Diesel".to_string(),
            350.0,
            18.0,
            3,
        );

        // Test Drivable interface methods
        let drive_message = truck.drive();
        assert!(
            !drive_message.is_empty(),
            "drive() should return a non-empty message"
        );
        assert!(
            drive_message.contains("MAN"),
            "drive() message should contain the brand"
        );
        assert!(
            drive_message.contains("18.0"),
            "drive() message should contain the cargo capacity"
        );

        let stop_message = truck.stop();
        assert!(
            !stop_message.is_empty(),
            "stop() should return a non-empty message"
        );
        assert!(
            stop_message.contains("air brakes"),
            "stop() message should mention air brakes for truck"
        );

        let turn_left = truck.turn("left".to_string());
        assert!(
            !turn_left.is_empty(),
            "turn() should return a non-empty message"
        );
        assert!(
            turn_left.contains("left"),
            "turn() message should contain the direction"
        );
        assert!(
            turn_left.contains("wide radius"),
            "turn() message should mention wide radius for truck"
        );

        let turn_right = truck.turn("right".to_string());
        assert!(
            turn_right.contains("right"),
            "turn() message should contain the direction"
        );

        println!("✓ Truck Drivable interface is properly implemented");
    }

    #[test]
    fn test_truck_maintainable_interface_implementation() {
        println!("\n=== Testing Truck Maintainable interface implementation ===");

        // Create Truck instance
        let truck = Truck::new(
            "Mercedes-Benz".to_string(),
            2025,
            "Euro 6 Diesel".to_string(),
            380.0,
            22.0,
            4,
        );

        // Test Maintainable interface methods
        let maintenance_message = truck.perform_maintenance();
        assert!(
            !maintenance_message.is_empty(),
            "perform_maintenance() should return a non-empty message"
        );
        assert!(
            maintenance_message.contains("Mercedes-Benz"),
            "perform_maintenance() message should contain the brand"
        );
        assert!(
            maintenance_message.contains("4"),
            "perform_maintenance() message should mention the number of axles"
        );
        assert!(
            maintenance_message.contains("axles"),
            "perform_maintenance() message should mention axle checking"
        );

        let condition_message = truck.check_condition();
        assert!(
            !condition_message.is_empty(),
            "check_condition() should return a non-empty message"
        );
        assert!(
            condition_message.contains("Euro 6 Diesel"),
            "check_condition() message should contain the engine type"
        );
        assert!(
            condition_message.contains("4"),
            "check_condition() message should contain the number of axles"
        );
        assert!(
            condition_message.contains("22.0"),
            "check_condition() message should contain the cargo capacity"
        );

        println!("✓ Truck Maintainable interface is properly implemented");
    }

    #[test]
    fn test_truck_describe_method() {
        println!("\n=== Testing Truck describe() method ===");

        // Create Truck instance
        let truck = Truck::new(
            "Kenworth".to_string(),
            2024,
            "Cummins Diesel".to_string(),
            500.0,
            30.0,
            5,
        );

        // Test describe() method (inherited from MotorVehicle)
        let description = truck.describe();
        println!("Description: {}", description);

        // Verify description contains all necessary information
        assert!(
            description.contains("Kenworth"),
            "Description should contain the brand"
        );
        assert!(
            description.contains("Truck"),
            "Description should contain the vehicle type"
        );
        assert!(
            description.contains("2024"),
            "Description should contain the year"
        );
        assert!(
            description.contains("Cummins Diesel"),
            "Description should contain the engine type"
        );
        assert!(
            description.contains("500.0"),
            "Description should contain the fuel capacity"
        );

        println!("✓ Truck describe() method works correctly");
    }

    #[test]
    fn test_truck_upcast_to_motor_vehicle() {
        println!("\n=== Testing Truck upcast to MotorVehicle ===");

        // Create Truck instance
        let truck = Truck::new(
            "Freightliner".to_string(),
            2023,
            "Detroit Diesel".to_string(),
            450.0,
            28.0,
            4,
        );

        // Record original values
        let original_brand = truck.get_brand().as_ref().unwrap().clone();
        let original_type = truck.get_type();
        let original_speed = truck.max_speed();

        // Upcast to MotorVehicle
        let motor_vehicle: CRc<MotorVehicle> = truck.clone().into_super();

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

        println!("✓ Truck upcast to MotorVehicle preserves all fields and methods");
    }

    #[test]
    fn test_truck_upcast_to_vehicle() {
        println!("\n=== Testing Truck upcast to Vehicle ===");

        // Create Truck instance
        let truck = Truck::new(
            "Peterbilt".to_string(),
            2022,
            "Paccar Diesel".to_string(),
            420.0,
            26.0,
            4,
        );

        // Record original values
        let original_brand = truck.get_brand().as_ref().unwrap().clone();
        let original_year = truck.get_year();

        // Upcast to MotorVehicle，然后到 Vehicle
        let motor_vehicle: CRc<MotorVehicle> = truck.clone().into_super();
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

        println!("✓ Truck upcast to Vehicle preserves all fields");
    }

    #[test]
    fn test_truck_interface_polymorphism() {
        println!("\n=== Testing Truck interface polymorphism ===");

        // Create Truck instance
        let truck = Truck::new(
            "Mack".to_string(),
            2024,
            "MP8 Diesel".to_string(),
            400.0,
            24.0,
            3,
        );

        // Test through Drivable interface reference
        let drivable = truck.clone();
        let drive_msg = drivable.drive();
        assert!(
            !drive_msg.is_empty(),
            "Drivable interface should work through interface reference"
        );

        // Test through Maintainable interface reference
        let maintainable = truck.clone();
        let maintenance_msg = maintainable.perform_maintenance();
        assert!(
            !maintenance_msg.is_empty(),
            "Maintainable interface should work through interface reference"
        );

        println!("✓ Truck interface polymorphism works correctly");
    }
}
