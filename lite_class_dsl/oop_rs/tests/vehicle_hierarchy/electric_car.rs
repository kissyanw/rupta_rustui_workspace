// ElectricCar class definition
//
// Inherits from Car (second-level derived class), implements Chargeable interface
// Represents an electric car type vehicle

use super::car::Car;
use super::interfaces::Chargeable;
use super::motor_vehicle::MotorVehicle;
use classes::*;

classes! {
    /// ElectricCar class
    ///
    /// Inherits from Car (second-level derived class), implements Chargeable interface
    /// Represents an electric car type vehicle
    pub class ElectricCar extends Car implements Chargeable {
        struct {
            // Battery capacity (kWh) - Copy types can be used directly
            pub battery_capacity: f64,
            // Charging time (hours) - Copy types can be used directly
            pub charging_time: f64,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `brand` - Vehicle brand
        /// * `year` - Vehicle year
        /// * `num_doors` - Number of doors
        /// * `trunk_capacity` - Trunk capacity (liters)
        /// * `battery_capacity` - Battery capacity (kWh)
        /// * `charging_time` - Charging time (hours)
        ///
        /// Note: engine_type is automatically set to "Electric", fuel_capacity is set to 0.0
        pub fn new(
            brand: String,
            year: i32,
            num_doors: i32,
            trunk_capacity: f64,
            battery_capacity: f64,
            charging_time: f64,
        ) -> Self {
            Self {
                super: Super::new(
                    brand,
                    year,
                    "Electric".to_string(),  // Electric car engine type is fixed to "Electric"
                    0.0,                      // Electric cars have no traditional fuel capacity
                    num_doors,
                    trunk_capacity,
                ),
                battery_capacity,
                charging_time,
            }
        }

        /// Override MotorVehicle fuel_efficiency() method
        ///
        /// For electric cars, returns electric efficiency (km/kWh) instead of fuel efficiency
        ///
        /// # Returns
        ///
        /// Electric efficiency (km/kWh)
        pub override fn MotorVehicle::fuel_efficiency(&self) -> f64 {
            // Electric car efficiency: approximately 6.0 km/kWh
            6.0
        }

        // Chargeable interface implementation

        /// Implements Chargeable interface: charge
        pub override fn Chargeable::charge(&self) -> String {
            format!(
                "Charging {} ElectricCar with {:.1} kWh battery, estimated time: {:.1} hours",
                self.get_brand().as_ref().unwrap(),
                self.get_battery_capacity(),
                self.get_charging_time()
            )
        }

        /// Implements Chargeable interface: get current battery percentage
        pub override fn Chargeable::battery_level(&self) -> f64 {
            // Simplified implementation: returns a fixed battery percentage
            // In real applications, this should be a mutable state
            85.0
        }

        /// Implements Chargeable interface: get charging status
        pub override fn Chargeable::charging_status(&self) -> String {
            let level = self.battery_level();
            if level >= 80.0 {
                format!("Battery at {:.1}% - Fully charged", level)
            } else if level >= 20.0 {
                format!("Battery at {:.1}% - Normal", level)
            } else {
                format!("Battery at {:.1}% - Low, charging recommended", level)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vehicle;

    #[test]
    fn test_electric_car_instance_creation_and_field_access() {
        println!("\n=== Testing ElectricCar instance creation and field access ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("Tesla".to_string(), 2024, 4, 500.0, 75.0, 8.0);

        // Test ElectricCar own field access
        assert_eq!(
            electric_car.get_battery_capacity(),
            75.0,
            "Battery capacity should be accessible and match the initialized value"
        );
        assert_eq!(
            electric_car.get_charging_time(),
            8.0,
            "Charging time should be accessible and match the initialized value"
        );

        // Test inherited field access from Car
        assert_eq!(
            electric_car.get_num_doors(),
            4,
            "Number of doors should be accessible and match the initialized value"
        );
        assert_eq!(
            electric_car.get_trunk_capacity(),
            500.0,
            "Trunk capacity should be accessible and match the initialized value"
        );

        // Test inherited field access from MotorVehicle
        assert_eq!(
            electric_car.get_engine_type().as_ref().unwrap(),
            "Electric",
            "Engine type should be 'Electric' for electric cars"
        );
        assert_eq!(
            electric_car.get_fuel_capacity(),
            0.0,
            "Fuel capacity should be 0.0 for electric cars"
        );

        // Test inherited field access from Vehicle
        assert_eq!(
            electric_car.get_brand().as_ref().unwrap(),
            "Tesla",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            electric_car.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        println!("✓ ElectricCar instance created successfully with all fields accessible");
    }

    #[test]
    fn test_electric_car_ancestor_field_access() {
        println!("\n=== Testing ElectricCar ancestor field access ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("Rivian".to_string(), 2023, 4, 450.0, 135.0, 10.0);

        // Verify can access all ancestor class fields
        // Vehicle fields
        let brand = electric_car.get_brand();
        assert!(
            brand.is_some(),
            "Should be able to access Vehicle's brand field"
        );
        assert_eq!(brand.as_ref().unwrap(), "Rivian");

        let year = electric_car.get_year();
        assert_eq!(year, 2023, "Should be able to access Vehicle's year field");

        // MotorVehicle fields
        let engine_type = electric_car.get_engine_type();
        assert!(
            engine_type.is_some(),
            "Should be able to access MotorVehicle's engine_type field"
        );
        assert_eq!(engine_type.as_ref().unwrap(), "Electric");

        let fuel_capacity = electric_car.get_fuel_capacity();
        assert_eq!(
            fuel_capacity, 0.0,
            "Should be able to access MotorVehicle's fuel_capacity field"
        );

        // Car fields
        let num_doors = electric_car.get_num_doors();
        assert_eq!(
            num_doors, 4,
            "Should be able to access Car's num_doors field"
        );

        let trunk_capacity = electric_car.get_trunk_capacity();
        assert_eq!(
            trunk_capacity, 450.0,
            "Should be able to access Car's trunk_capacity field"
        );

        println!("✓ ElectricCar can access all ancestor class fields");
    }

    #[test]
    fn test_electric_car_chargeable_interface_implementation() {
        println!("\n=== Testing ElectricCar Chargeable interface implementation ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("Lucid".to_string(), 2024, 4, 480.0, 112.0, 9.5);

        // Test Chargeable interface methods
        let charge_message = electric_car.charge();
        assert!(
            !charge_message.is_empty(),
            "charge() should return a non-empty message"
        );
        assert!(
            charge_message.contains("Lucid"),
            "charge() message should contain the brand"
        );
        assert!(
            charge_message.contains("112.0"),
            "charge() message should contain the battery capacity"
        );
        assert!(
            charge_message.contains("9.5"),
            "charge() message should contain the charging time"
        );

        let battery_level = electric_car.battery_level();
        assert!(
            battery_level >= 0.0 && battery_level <= 100.0,
            "battery_level() should return a value between 0.0 and 100.0"
        );

        let charging_status = electric_car.charging_status();
        assert!(
            !charging_status.is_empty(),
            "charging_status() should return a non-empty message"
        );
        assert!(
            charging_status.contains(&format!("{:.1}", battery_level)),
            "charging_status() message should contain the battery level"
        );

        println!("✓ ElectricCar Chargeable interface is properly implemented");
    }

    #[test]
    fn test_electric_car_fuel_efficiency_override() {
        println!("\n=== Testing ElectricCar fuel_efficiency() override ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("Polestar".to_string(), 2024, 4, 420.0, 78.0, 7.5);

        // Test overridden fuel_efficiency() method
        let efficiency = electric_car.fuel_efficiency();
        assert_eq!(
            efficiency, 6.0,
            "ElectricCar should override fuel_efficiency() to return electric efficiency"
        );
        assert!(
            efficiency > 0.0,
            "fuel_efficiency() should return a positive value"
        );

        println!("✓ ElectricCar fuel_efficiency() override works correctly");
    }

    #[test]
    fn test_electric_car_inherited_methods() {
        println!("\n=== Testing ElectricCar inherited methods ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("BYD".to_string(), 2024, 4, 400.0, 60.0, 6.0);

        // Test methods inherited from Vehicle
        let vehicle_type = electric_car.get_type();
        assert_eq!(vehicle_type, "Car", "get_type() should return 'Car'");

        let max_speed = electric_car.max_speed();
        assert!(
            max_speed > 0.0,
            "max_speed() should return a positive value"
        );

        let description = electric_car.describe();
        assert!(
            !description.is_empty(),
            "describe() should return a non-empty string"
        );
        assert!(
            description.contains("BYD"),
            "describe() should contain the brand"
        );

        // Test methods inherited from MotorVehicle
        let start_message = electric_car.start_engine();
        assert!(
            !start_message.is_empty(),
            "start_engine() should return a non-empty message"
        );
        assert!(
            start_message.contains("Electric"),
            "start_engine() should mention electric engine"
        );

        // Test Drivable interface methods inherited from Car
        let drive_message = electric_car.drive();
        assert!(
            !drive_message.is_empty(),
            "drive() should return a non-empty message"
        );

        let stop_message = electric_car.stop();
        assert!(
            !stop_message.is_empty(),
            "stop() should return a non-empty message"
        );

        let turn_message = electric_car.turn("left".to_string());
        assert!(
            !turn_message.is_empty(),
            "turn() should return a non-empty message"
        );

        // Test Maintainable interface methods inherited from Car
        let maintenance_message = electric_car.perform_maintenance();
        assert!(
            !maintenance_message.is_empty(),
            "perform_maintenance() should return a non-empty message"
        );

        let condition_message = electric_car.check_condition();
        assert!(
            !condition_message.is_empty(),
            "check_condition() should return a non-empty message"
        );

        println!("✓ ElectricCar inherited methods work correctly");
    }

    #[test]
    fn test_electric_car_upcast_to_car() {
        println!("\n=== Testing ElectricCar upcast to Car ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("NIO".to_string(), 2024, 4, 460.0, 100.0, 8.5);

        // Record original values
        let original_brand = electric_car.get_brand().as_ref().unwrap().clone();
        let original_type = electric_car.get_type();

        // Upcast to Car
        let car: CRc<Car> = electric_car.clone().into_super();

        // Verify fields and methods are still accessible after upcast
        assert_eq!(
            car.get_brand().as_ref().unwrap(),
            &original_brand,
            "Brand should be preserved after upcast to Car"
        );
        assert_eq!(
            car.get_type(),
            original_type,
            "Type should be preserved after upcast to Car"
        );

        println!("✓ ElectricCar upcast to Car preserves all fields and methods");
    }

    #[test]
    fn test_electric_car_upcast_to_motor_vehicle() {
        println!("\n=== Testing ElectricCar upcast to MotorVehicle ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("Xpeng".to_string(), 2024, 4, 440.0, 80.0, 7.0);

        // Upcast to Car，然后到 MotorVehicle
        let car: CRc<Car> = electric_car.clone().into_super();
        let motor_vehicle: CRc<MotorVehicle> = car.into_super();

        // Verify fields are still accessible after multi-level upcast
        assert_eq!(
            motor_vehicle.get_brand().as_ref().unwrap(),
            "Xpeng",
            "Brand should be preserved after upcast to MotorVehicle"
        );
        assert_eq!(
            motor_vehicle.get_engine_type().as_ref().unwrap(),
            "Electric",
            "Engine type should be preserved after upcast to MotorVehicle"
        );

        println!("✓ ElectricCar upcast to MotorVehicle preserves all fields");
    }

    #[test]
    fn test_electric_car_upcast_to_vehicle() {
        println!("\n=== Testing ElectricCar upcast to Vehicle ===");

        // Create ElectricCar instance
        let electric_car = ElectricCar::new("Audi".to_string(), 2024, 4, 430.0, 95.0, 8.0);

        // Upcast to Car，然后到 MotorVehicle，最后到 Vehicle
        let car: CRc<Car> = electric_car.clone().into_super();
        let motor_vehicle: CRc<MotorVehicle> = car.into_super();
        let vehicle: CRc<Vehicle> = motor_vehicle.into_super().into();

        // Verify fields are still accessible after multi-level upcast
        assert_eq!(
            vehicle.get_brand().as_ref().unwrap(),
            "Audi",
            "Brand should be preserved after upcast to Vehicle"
        );
        assert_eq!(
            vehicle.get_year(),
            2024,
            "Year should be preserved after upcast to Vehicle"
        );

        println!("✓ ElectricCar upcast to Vehicle preserves all fields");
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

    // Generate valid number of doors (2-5)
    fn num_doors_strategy() -> impl Strategy<Value = i32> {
        2..=5
    }

    // Generate valid trunk capacity (100.0-1000.0 liters)
    fn trunk_capacity_strategy() -> impl Strategy<Value = f64> {
        100.0..=1000.0
    }

    // Generate valid battery capacity (30.0-200.0 kWh)
    fn battery_capacity_strategy() -> impl Strategy<Value = f64> {
        30.0..=200.0
    }

    // Generate valid charging time (1.0-15.0 hours)
    fn charging_time_strategy() -> impl Strategy<Value = f64> {
        1.0..=15.0
    }

    proptest! {
        #[test]
        fn prop_electric_car_can_access_all_ancestor_fields(
            brand in brand_strategy(),
            year in year_strategy(),
            num_doors in num_doors_strategy(),
            trunk_capacity in trunk_capacity_strategy(),
            battery_capacity in battery_capacity_strategy(),
            charging_time in charging_time_strategy(),
        ) {
            // Create ElectricCar instance
            let electric_car = ElectricCar::new(
                brand.clone(),
                year,
                num_doors,
                trunk_capacity,
                battery_capacity,
                charging_time,
            );

            // Verify can access all ancestor class fields，且不会 panic

            // Vehicle fields（祖先类 - 3级）
            let retrieved_brand = electric_car.get_brand();
            prop_assert!(retrieved_brand.is_some(), "Should be able to access Vehicle's brand field");
            prop_assert_eq!(retrieved_brand.as_ref().unwrap(), &brand, "Brand should match initialized value");

            let retrieved_year = electric_car.get_year();
            prop_assert_eq!(retrieved_year, year, "Should be able to access Vehicle's year field");

            // MotorVehicle fields（祖先类 - 2级）
            let retrieved_engine_type = electric_car.get_engine_type();
            prop_assert!(retrieved_engine_type.is_some(), "Should be able to access MotorVehicle's engine_type field");
            prop_assert_eq!(retrieved_engine_type.as_ref().unwrap(), "Electric", "Engine type should be 'Electric'");

            let retrieved_fuel_capacity = electric_car.get_fuel_capacity();
            prop_assert_eq!(retrieved_fuel_capacity, 0.0, "Should be able to access MotorVehicle's fuel_capacity field");

            // Car fields（父类 - 1级）
            let retrieved_num_doors = electric_car.get_num_doors();
            prop_assert_eq!(retrieved_num_doors, num_doors, "Should be able to access Car's num_doors field");

            let retrieved_trunk_capacity = electric_car.get_trunk_capacity();
            prop_assert_eq!(retrieved_trunk_capacity, trunk_capacity, "Should be able to access Car's trunk_capacity field");

            // ElectricCar own fields
            let retrieved_battery_capacity = electric_car.get_battery_capacity();
            prop_assert_eq!(retrieved_battery_capacity, battery_capacity, "Battery capacity should match initialized value");

            let retrieved_charging_time = electric_car.get_charging_time();
            prop_assert_eq!(retrieved_charging_time, charging_time, "Charging time should match initialized value");

            // Verify all ancestor class methods can be called
            // Vehicle methods
            let _ = electric_car.get_type();
            let _ = electric_car.max_speed();
            let _ = electric_car.describe();

            // MotorVehicle methods
            let _ = electric_car.start_engine();
            let _ = electric_car.fuel_efficiency();

            // Car methods (Drivable interface)
            let _ = electric_car.drive();
            let _ = electric_car.stop();
            let _ = electric_car.turn("left".to_string());

            // Car methods (Maintainable interface)
            let _ = electric_car.perform_maintenance();
            let _ = electric_car.check_condition();

            // ElectricCar methods (Chargeable interface)
            let _ = electric_car.charge();
            let _ = electric_car.battery_level();
            let _ = electric_car.charging_status();
        }
    }
}
