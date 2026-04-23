// Car class definition
//
// Inherits from MotorVehicle, implements Drivable and Maintainable interfaces
// Represents a car type vehicle

use super::interfaces::{Drivable, Maintainable};
use super::motor_vehicle::MotorVehicle;
use super::vehicle::Vehicle;
use classes::*;

classes! {
    /// Car class
    ///
    /// Inherits from MotorVehicle, implements Drivable and Maintainable interfaces
    /// Represents a standard car type vehicle
    pub class Car extends MotorVehicle implements Drivable, Maintainable {
        struct {
            // Number of doors - Copy types can be used directly
            pub num_doors: i32,
            // Trunk capacity (liters) - Copy types can be used directly
            pub trunk_capacity: f64,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `brand` - Vehicle brand
        /// * `year` - Vehicle year
        /// * `engine_type` - Engine type
        /// * `fuel_capacity` - Fuel capacity (liters)
        /// * `num_doors` - Number of doors
        /// * `trunk_capacity` - Trunk capacity (liters)
        pub fn new(
            brand: String,
            year: i32,
            engine_type: String,
            fuel_capacity: f64,
            num_doors: i32,
            trunk_capacity: f64,
        ) -> Self {
            Self {
                super: Super::new(brand, year, engine_type, fuel_capacity),
                num_doors,
                trunk_capacity,
            }
        }

        /// Implements Vehicle abstract method: returns vehicle type description
        pub override fn Vehicle::get_type(&self) -> String {
            "Car".to_string()
        }

        /// Implements Vehicle abstract method: returns maximum speed (km/h)
        pub override fn Vehicle::max_speed(&self) -> f64 {
            180.0
        }

        /// Implements MotorVehicle abstract method: returns engine start description
        pub override fn MotorVehicle::start_engine(&self) -> String {
            format!(
                "Starting {} car engine",
                self.get_engine_type().as_ref().unwrap()
            )
        }

        /// Implements MotorVehicle abstract method: returns fuel efficiency (km/L)
        pub override fn MotorVehicle::fuel_efficiency(&self) -> f64 {
            15.0
        }

        // Drivable interface implementation

        /// Implements Drivable interface: drive vehicle
        pub override fn Drivable::drive(&self) -> String {
            format!(
                "Driving {} car with {} doors",
                self.get_brand().as_ref().unwrap(),
                self.get_num_doors()
            )
        }

        /// Implements Drivable interface: stop vehicle
        pub override fn Drivable::stop(&self) -> String {
            "Car stopped smoothly".to_string()
        }

        /// Implements Drivable interface: turn
        pub override fn Drivable::turn(&self, direction: String) -> String {
            format!("Car turning {}", direction)
        }

        // Maintainable interface implementation

        /// Implements Maintainable interface: perform maintenance
        pub override fn Maintainable::perform_maintenance(&self) -> String {
            format!(
                "Performing maintenance on {} car: oil change, tire rotation, brake inspection",
                self.get_brand().as_ref().unwrap()
            )
        }

        /// Implements Maintainable interface: check vehicle condition
        pub override fn Maintainable::check_condition(&self) -> String {
            format!(
                "Car condition: Engine {}, {} doors functional, trunk capacity {:.1}L",
                self.get_engine_type().as_ref().unwrap(),
                self.get_num_doors(),
                self.get_trunk_capacity()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_instance_creation_and_field_access() {
        println!("\n=== Testing Car instance creation and field access ===");

        // Create Car instance
        let car = Car::new(
            "Toyota".to_string(),
            2024,
            "Gasoline".to_string(),
            50.0,
            4,
            400.0,
        );

        // Test Car own field access
        assert_eq!(
            car.get_num_doors(),
            4,
            "Number of doors should be accessible and match the initialized value"
        );
        assert_eq!(
            car.get_trunk_capacity(),
            400.0,
            "Trunk capacity should be accessible and match the initialized value"
        );

        // Test inherited field access from MotorVehicle
        assert_eq!(
            car.get_engine_type().as_ref().unwrap(),
            "Gasoline",
            "Engine type should be accessible and match the initialized value"
        );
        assert_eq!(
            car.get_fuel_capacity(),
            50.0,
            "Fuel capacity should be accessible and match the initialized value"
        );

        // Test inherited field access from Vehicle
        assert_eq!(
            car.get_brand().as_ref().unwrap(),
            "Toyota",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            car.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        println!("✓ Car instance created successfully with all fields accessible");
    }

    #[test]
    fn test_car_abstract_methods_implementation() {
        println!("\n=== Testing Car abstract methods implementation ===");

        // Create Car instance
        let car = Car::new(
            "Honda".to_string(),
            2023,
            "Hybrid".to_string(),
            45.0,
            4,
            350.0,
        );

        // Test Vehicle abstract method implementation
        let vehicle_type = car.get_type();
        assert_eq!(vehicle_type, "Car", "get_type() should return 'Car'");

        let max_speed = car.max_speed();
        assert_eq!(
            max_speed, 180.0,
            "max_speed() should return the expected speed"
        );
        assert!(
            max_speed > 0.0,
            "max_speed() should return a positive value"
        );

        // Test MotorVehicle abstract method implementation
        let start_message = car.start_engine();
        assert!(
            !start_message.is_empty(),
            "start_engine() should return a non-empty message"
        );
        assert!(
            start_message.contains("Hybrid"),
            "start_engine() message should contain the engine type"
        );

        let efficiency = car.fuel_efficiency();
        assert_eq!(
            efficiency, 15.0,
            "fuel_efficiency() should return the expected efficiency"
        );
        assert!(
            efficiency > 0.0,
            "fuel_efficiency() should return a positive value"
        );

        println!("✓ Car abstract methods are properly implemented");
    }

    #[test]
    fn test_car_drivable_interface_implementation() {
        println!("\n=== Testing Car Drivable interface implementation ===");

        // Create Car instance
        let car = Car::new(
            "Ford".to_string(),
            2022,
            "Diesel".to_string(),
            60.0,
            2,
            300.0,
        );

        // Test Drivable interface methods
        let drive_message = car.drive();
        assert!(
            !drive_message.is_empty(),
            "drive() should return a non-empty message"
        );
        assert!(
            drive_message.contains("Ford"),
            "drive() message should contain the brand"
        );
        assert!(
            drive_message.contains("2"),
            "drive() message should contain the number of doors"
        );

        let stop_message = car.stop();
        assert!(
            !stop_message.is_empty(),
            "stop() should return a non-empty message"
        );

        let turn_left = car.turn("left".to_string());
        assert!(
            !turn_left.is_empty(),
            "turn() should return a non-empty message"
        );
        assert!(
            turn_left.contains("left"),
            "turn() message should contain the direction"
        );

        let turn_right = car.turn("right".to_string());
        assert!(
            turn_right.contains("right"),
            "turn() message should contain the direction"
        );

        println!("✓ Car Drivable interface is properly implemented");
    }

    #[test]
    fn test_car_maintainable_interface_implementation() {
        println!("\n=== Testing Car Maintainable interface implementation ===");

        // Create Car instance
        let car = Car::new(
            "BMW".to_string(),
            2025,
            "Electric".to_string(),
            0.0,
            4,
            450.0,
        );

        // Test Maintainable interface methods
        let maintenance_message = car.perform_maintenance();
        assert!(
            !maintenance_message.is_empty(),
            "perform_maintenance() should return a non-empty message"
        );
        assert!(
            maintenance_message.contains("BMW"),
            "perform_maintenance() message should contain the brand"
        );

        let condition_message = car.check_condition();
        assert!(
            !condition_message.is_empty(),
            "check_condition() should return a non-empty message"
        );
        assert!(
            condition_message.contains("Electric"),
            "check_condition() message should contain the engine type"
        );
        assert!(
            condition_message.contains("4"),
            "check_condition() message should contain the number of doors"
        );
        assert!(
            condition_message.contains("450.0"),
            "check_condition() message should contain the trunk capacity"
        );

        println!("✓ Car Maintainable interface is properly implemented");
    }

    #[test]
    fn test_car_describe_method() {
        println!("\n=== Testing Car describe() method ===");

        // Create Car instance
        let car = Car::new(
            "Mercedes".to_string(),
            2024,
            "V8".to_string(),
            70.0,
            4,
            500.0,
        );

        // Test describe() method (inherited from MotorVehicle)
        let description = car.describe();
        println!("Description: {}", description);

        // Verify description contains all necessary information
        assert!(
            description.contains("Mercedes"),
            "Description should contain the brand"
        );
        assert!(
            description.contains("Car"),
            "Description should contain the vehicle type"
        );
        assert!(
            description.contains("2024"),
            "Description should contain the year"
        );
        assert!(
            description.contains("V8"),
            "Description should contain the engine type"
        );
        assert!(
            description.contains("70.0"),
            "Description should contain the fuel capacity"
        );

        println!("✓ Car describe() method works correctly");
    }

    #[test]
    fn test_car_upcast_to_motor_vehicle() {
        println!("\n=== Testing Car upcast to MotorVehicle ===");

        // Create Car instance
        let car = Car::new(
            "Audi".to_string(),
            2023,
            "Turbocharged".to_string(),
            55.0,
            4,
            380.0,
        );

        // Record original values
        let original_brand = car.get_brand().as_ref().unwrap().clone();
        let original_type = car.get_type();
        let original_speed = car.max_speed();

        // Upcast to MotorVehicle
        let motor_vehicle: CRc<MotorVehicle> = car.clone().into_super();

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

        println!("✓ Car upcast to MotorVehicle preserves all fields and methods");
    }

    #[test]
    fn test_car_upcast_to_vehicle() {
        println!("\n=== Testing Car upcast to Vehicle ===");

        // Create Car instance
        let car = Car::new(
            "Volkswagen".to_string(),
            2022,
            "TDI".to_string(),
            58.0,
            4,
            420.0,
        );

        // Record original values
        let original_brand = car.get_brand().as_ref().unwrap().clone();
        let original_year = car.get_year();

        // Upcast to MotorVehicle，然后到 Vehicle
        let motor_vehicle: CRc<MotorVehicle> = car.clone().into_super();
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

        println!("✓ Car upcast to Vehicle preserves all fields");
    }

    #[test]
    fn test_car_interface_polymorphism() {
        println!("\n=== Testing Car interface polymorphism ===");

        // Create Car instance
        let car = Car::new(
            "Tesla".to_string(),
            2024,
            "Electric".to_string(),
            0.0,
            4,
            400.0,
        );

        // Test through Drivable interface reference
        let drivable = car.clone();
        let drive_msg = drivable.drive();
        assert!(
            !drive_msg.is_empty(),
            "Drivable interface should work through interface reference"
        );

        // Test through Maintainable interface reference
        let maintainable = car.clone();
        let maintenance_msg = maintainable.perform_maintenance();
        assert!(
            !maintenance_msg.is_empty(),
            "Maintainable interface should work through interface reference"
        );

        println!("✓ Car interface polymorphism works correctly");
    }
}

// Property test module
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Generate valid brand names
    fn brand_strategy() -> impl Strategy<Value = String> {
        prop::string::string_regex("[A-Z][a-z]{2,10}")
            .unwrap()
            .prop_map(|s| s)
    }

    // Generate valid years (1900-2030)
    fn year_strategy() -> impl Strategy<Value = i32> {
        1900..=2030
    }

    // Generate valid engine types
    fn engine_type_strategy() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "Gasoline".to_string(),
            "Diesel".to_string(),
            "Hybrid".to_string(),
            "Electric".to_string(),
            "V6".to_string(),
            "V8".to_string(),
        ])
    }

    // Generate valid fuel capacity (10.0-100.0 liters)
    fn fuel_capacity_strategy() -> impl Strategy<Value = f64> {
        10.0..=100.0
    }

    // Generate valid number of doors (2-5)
    fn num_doors_strategy() -> impl Strategy<Value = i32> {
        2..=5
    }

    // Generate valid trunk capacity (100.0-1000.0 liters)
    fn trunk_capacity_strategy() -> impl Strategy<Value = f64> {
        100.0..=1000.0
    }

    proptest! {
        #[test]
        fn prop_car_get_type_returns_non_empty_string(
            brand in brand_strategy(),
            year in year_strategy(),
            engine_type in engine_type_strategy(),
            fuel_capacity in fuel_capacity_strategy(),
            num_doors in num_doors_strategy(),
            trunk_capacity in trunk_capacity_strategy(),
        ) {
            // Create Car instance
            let car = Car::new(
                brand,
                year,
                engine_type,
                fuel_capacity,
                num_doors,
                trunk_capacity,
            );

            // Verify get_type() returns non-empty string
            let vehicle_type = car.get_type();
            prop_assert!(!vehicle_type.is_empty(), "get_type() should return a non-empty string");
            prop_assert_eq!(vehicle_type, "Car", "get_type() should return 'Car'");
        }
    }

    proptest! {
        #[test]
        fn prop_car_max_speed_returns_positive_number(
            brand in brand_strategy(),
            year in year_strategy(),
            engine_type in engine_type_strategy(),
            fuel_capacity in fuel_capacity_strategy(),
            num_doors in num_doors_strategy(),
            trunk_capacity in trunk_capacity_strategy(),
        ) {
            // Create Car instance
            let car = Car::new(
                brand,
                year,
                engine_type,
                fuel_capacity,
                num_doors,
                trunk_capacity,
            );

            // Verify max_speed() returns positive number
            let speed = car.max_speed();
            prop_assert!(speed > 0.0, "max_speed() should return a positive number, got {}", speed);
        }
    }

    proptest! {
        #[test]
        fn prop_car_field_initialization_completeness(
            brand in brand_strategy(),
            year in year_strategy(),
            engine_type in engine_type_strategy(),
            fuel_capacity in fuel_capacity_strategy(),
            num_doors in num_doors_strategy(),
            trunk_capacity in trunk_capacity_strategy(),
        ) {
            // Create Car instance
            let car = Car::new(
                brand.clone(),
                year,
                engine_type.clone(),
                fuel_capacity,
                num_doors,
                trunk_capacity,
            );

            // Verify all fields can be accessed through getter methods without panic

            // Vehicle fields
            let retrieved_brand = car.get_brand();
            prop_assert!(retrieved_brand.is_some(), "Brand should be initialized");
            prop_assert_eq!(retrieved_brand.as_ref().unwrap(), &brand, "Brand should match initialized value");

            let retrieved_year = car.get_year();
            prop_assert_eq!(retrieved_year, year, "Year should match initialized value");

            // MotorVehicle fields
            let retrieved_engine_type = car.get_engine_type();
            prop_assert!(retrieved_engine_type.is_some(), "Engine type should be initialized");
            prop_assert_eq!(retrieved_engine_type.as_ref().unwrap(), &engine_type, "Engine type should match initialized value");

            let retrieved_fuel_capacity = car.get_fuel_capacity();
            prop_assert_eq!(retrieved_fuel_capacity, fuel_capacity, "Fuel capacity should match initialized value");

            // Car fields
            let retrieved_num_doors = car.get_num_doors();
            prop_assert_eq!(retrieved_num_doors, num_doors, "Number of doors should match initialized value");

            let retrieved_trunk_capacity = car.get_trunk_capacity();
            prop_assert_eq!(retrieved_trunk_capacity, trunk_capacity, "Trunk capacity should match initialized value");

            // Verify all abstract methods can be called
            let _ = car.get_type();
            let _ = car.max_speed();
            let _ = car.start_engine();
            let _ = car.fuel_efficiency();
            let _ = car.describe();

            // Verify all interface methods can be called
            let _ = car.drive();
            let _ = car.stop();
            let _ = car.turn("left".to_string());
            let _ = car.perform_maintenance();
            let _ = car.check_condition();
        }
    }
}
