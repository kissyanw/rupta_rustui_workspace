// SportsCar class definition
//
// Inherits from Car and mixes in Autonomous mixin
// Represents a high-performance sports car type vehicle

use super::car::Car;
use super::mixins::Autonomous;
use super::vehicle::Vehicle;
use classes::*;

classes! {
    /// SportsCar class
    ///
    /// Inherits from Car and mixes in Autonomous mixin
    /// Represents a high-performance sports car with autonomous driving capabilities
    pub class SportsCar extends Car with Autonomous {
        struct {
            // Top speed (km/h) - Copy types can be used directly
            pub top_speed: f64,
            // Acceleration (0-100 km/h seconds) - Copy types can be used directly
            pub acceleration: f64,
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
        /// * `top_speed` - Top speed (km/h)
        /// * `acceleration` - Acceleration (0-100 km/h seconds)
        /// * `autonomy_level` - Autonomous driving level (1-5)
        /// * `sensor_count` - Sensor count
        pub fn new(
            brand: String,
            year: i32,
            engine_type: String,
            fuel_capacity: f64,
            num_doors: i32,
            trunk_capacity: f64,
            top_speed: f64,
            acceleration: f64,
            autonomy_level: i32,
            sensor_count: i32,
        ) -> Self {
            let sports_car: CRc<Self> = Self {
                super: Super::new(brand, year, engine_type, fuel_capacity, num_doors, trunk_capacity),
                top_speed,
                acceleration,
                ..
            };
            // Set mixin fields
            sports_car.set_autonomy_level(autonomy_level);
            sports_car.set_sensor_count(sensor_count);
            sports_car
        }

        /// Override Vehicle max_speed() method
        /// Returns SportsCar top_speed field value
        pub override fn Vehicle::max_speed(&self) -> f64 {
            self.get_top_speed()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sports_car_instance_creation_and_field_access() {
        println!("\n=== Testing SportsCar instance creation and field access ===");

        // Create SportsCar instance
        let sports_car = SportsCar::new(
            "Ferrari".to_string(),
            2024,
            "V12".to_string(),
            80.0,
            2,
            200.0,
            350.0, // top_speed
            2.9,   // acceleration (0-100 km/h in 2.9 seconds)
            5,     // autonomy_level
            20,    // sensor_count
        );

        // Test SportsCar own field access
        assert_eq!(
            sports_car.get_top_speed(),
            350.0,
            "Top speed should be accessible and match the initialized value"
        );
        assert_eq!(
            sports_car.get_acceleration(),
            2.9,
            "Acceleration should be accessible and match the initialized value"
        );

        // Test Autonomous mixin field access
        assert_eq!(
            sports_car.get_autonomy_level(),
            5,
            "Autonomy level should be accessible and match the initialized value"
        );
        assert_eq!(
            sports_car.get_sensor_count(),
            20,
            "Sensor count should be accessible and match the initialized value"
        );

        // Test inherited field access from Car
        assert_eq!(
            sports_car.get_num_doors(),
            2,
            "Number of doors should be accessible and match the initialized value"
        );
        assert_eq!(
            sports_car.get_trunk_capacity(),
            200.0,
            "Trunk capacity should be accessible and match the initialized value"
        );

        // Test inherited field access from MotorVehicle
        assert_eq!(
            sports_car.get_engine_type().as_ref().unwrap(),
            "V12",
            "Engine type should be accessible and match the initialized value"
        );
        assert_eq!(
            sports_car.get_fuel_capacity(),
            80.0,
            "Fuel capacity should be accessible and match the initialized value"
        );

        // Test inherited field access from Vehicle
        assert_eq!(
            sports_car.get_brand().as_ref().unwrap(),
            "Ferrari",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            sports_car.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        println!("✓ SportsCar instance created successfully with all fields accessible");
    }

    #[test]
    fn test_sports_car_autonomous_mixin_functionality() {
        println!("\n=== Testing SportsCar Autonomous mixin functionality ===");

        // Create SportsCar instance
        let sports_car = SportsCar::new(
            "Porsche".to_string(),
            2025,
            "Turbocharged".to_string(),
            70.0,
            2,
            150.0,
            320.0,
            3.2,
            4,  // Level 4 autonomy
            16, // 16 sensors
        );

        // Test enable_autopilot() method
        let enable_message = sports_car.enable_autopilot();
        println!("Enable autopilot message: {}", enable_message);

        assert!(
            !enable_message.is_empty(),
            "enable_autopilot() should return a non-empty message"
        );
        assert!(
            enable_message.contains("Level 4"),
            "enable_autopilot() message should contain the autonomy level"
        );
        assert!(
            enable_message.contains("16 sensors"),
            "enable_autopilot() message should contain the sensor count"
        );
        assert!(
            enable_message.contains("Enabling"),
            "enable_autopilot() message should indicate enabling action"
        );

        // Test disable_autopilot() method
        let disable_message = sports_car.disable_autopilot();
        println!("Disable autopilot message: {}", disable_message);

        assert_eq!(
            disable_message, "Disabling autopilot",
            "disable_autopilot() should return the expected message"
        );

        println!("✓ SportsCar Autonomous mixin functionality works correctly");
    }

    #[test]
    fn test_sports_car_method_override() {
        println!("\n=== Testing SportsCar method override ===");

        // Create SportsCar instance
        let sports_car = SportsCar::new(
            "Lamborghini".to_string(),
            2024,
            "V10".to_string(),
            75.0,
            2,
            180.0,
            340.0, // top_speed
            2.8,
            5,
            18,
        );

        // 测试 max_speed() 方法重写
        // SportsCar 应该返回 top_speed 字段的值，而不是 Car 的默认值 180.0
        let max_speed = sports_car.max_speed();
        assert_eq!(
            max_speed, 340.0,
            "max_speed() should return the top_speed field value"
        );
        assert_eq!(
            max_speed,
            sports_car.get_top_speed(),
            "max_speed() should match get_top_speed()"
        );

        // 验证这与 Car 的默认 max_speed 不同
        assert_ne!(
            max_speed, 180.0,
            "SportsCar max_speed should be different from Car's default"
        );

        println!("✓ SportsCar method override works correctly");
    }

    #[test]
    fn test_sports_car_inherited_methods() {
        println!("\n=== Testing SportsCar inherited methods ===");

        // Create SportsCar instance
        let sports_car = SportsCar::new(
            "McLaren".to_string(),
            2024,
            "Twin-Turbo V8".to_string(),
            72.0,
            2,
            160.0,
            330.0,
            3.0,
            5,
            22,
        );

        // Test methods inherited from Vehicle
        let vehicle_type = sports_car.get_type();
        assert_eq!(
            vehicle_type, "Car",
            "get_type() should return 'Car' (inherited from Car)"
        );

        // Test methods inherited from MotorVehicle
        let start_message = sports_car.start_engine();
        assert!(
            !start_message.is_empty(),
            "start_engine() should return a non-empty message"
        );
        assert!(
            start_message.contains("Twin-Turbo V8"),
            "start_engine() message should contain the engine type"
        );

        let efficiency = sports_car.fuel_efficiency();
        assert!(
            efficiency > 0.0,
            "fuel_efficiency() should return a positive value"
        );

        // 测试继承自 Car 的接口方法
        let drive_message = sports_car.drive();
        assert!(
            !drive_message.is_empty(),
            "drive() should return a non-empty message"
        );
        assert!(
            drive_message.contains("McLaren"),
            "drive() message should contain the brand"
        );

        let stop_message = sports_car.stop();
        assert!(
            !stop_message.is_empty(),
            "stop() should return a non-empty message"
        );

        let turn_message = sports_car.turn("left".to_string());
        assert!(
            !turn_message.is_empty(),
            "turn() should return a non-empty message"
        );
        assert!(
            turn_message.contains("left"),
            "turn() message should contain the direction"
        );

        let maintenance_message = sports_car.perform_maintenance();
        assert!(
            !maintenance_message.is_empty(),
            "perform_maintenance() should return a non-empty message"
        );

        let condition_message = sports_car.check_condition();
        assert!(
            !condition_message.is_empty(),
            "check_condition() should return a non-empty message"
        );

        println!("✓ SportsCar inherited methods work correctly");
    }

    #[test]
    fn test_sports_car_describe_method() {
        println!("\n=== Testing SportsCar describe() method ===");

        // Create SportsCar instance
        let sports_car = SportsCar::new(
            "Bugatti".to_string(),
            2024,
            "W16".to_string(),
            100.0,
            2,
            100.0,
            420.0,
            2.5,
            5,
            25,
        );

        // Test describe() method (inherited from MotorVehicle)
        let description = sports_car.describe();
        println!("Description: {}", description);

        // Verify description contains all necessary information
        assert!(
            description.contains("Bugatti"),
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
            description.contains("W16"),
            "Description should contain the engine type"
        );

        println!("✓ SportsCar describe() method works correctly");
    }

    #[test]
    fn test_sports_car_upcast_to_car() {
        println!("\n=== Testing SportsCar upcast to Car ===");

        // Create SportsCar instance
        let sports_car = SportsCar::new(
            "Aston Martin".to_string(),
            2024,
            "V12".to_string(),
            78.0,
            2,
            170.0,
            335.0,
            3.5,
            4,
            15,
        );

        // Record original values
        let original_brand = sports_car.get_brand().as_ref().unwrap().clone();
        let original_max_speed = sports_car.max_speed();

        // Upcast to Car
        // Note: Classes using mixins need to use into_superclass instead of into_super
        let car = sports_car.clone().into_superclass::<CRc<Car>>();

        // Verify fields and methods are still accessible after upcast
        assert_eq!(
            car.get_brand().as_ref().unwrap(),
            &original_brand,
            "Brand should be preserved after upcast to Car"
        );

        // Note: After upcast, max_speed() will call Car implementation, not SportsCar override
        // This is because upcast changes the object type
        let car_max_speed = car.max_speed();
        println!(
            "Original max_speed: {}, Car max_speed: {}",
            original_max_speed, car_max_speed
        );

        println!("✓ SportsCar upcast to Car works correctly");
    }

    #[test]
    fn test_sports_car_multiple_instances() {
        println!("\n=== Testing multiple SportsCar instances ===");

        // Create multiple SportsCar instances with different configurations
        let ferrari = SportsCar::new(
            "Ferrari".to_string(),
            2024,
            "V12".to_string(),
            80.0,
            2,
            200.0,
            350.0,
            2.9,
            5,
            20,
        );

        let porsche = SportsCar::new(
            "Porsche".to_string(),
            2024,
            "Flat-6".to_string(),
            64.0,
            2,
            130.0,
            310.0,
            3.7,
            3,
            12,
        );

        let lamborghini = SportsCar::new(
            "Lamborghini".to_string(),
            2024,
            "V10".to_string(),
            75.0,
            2,
            180.0,
            340.0,
            2.8,
            4,
            18,
        );

        // Verify each instance has independent state
        assert_eq!(ferrari.get_brand().as_ref().unwrap(), "Ferrari");
        assert_eq!(ferrari.max_speed(), 350.0);
        assert_eq!(ferrari.get_autonomy_level(), 5);

        assert_eq!(porsche.get_brand().as_ref().unwrap(), "Porsche");
        assert_eq!(porsche.max_speed(), 310.0);
        assert_eq!(porsche.get_autonomy_level(), 3);

        assert_eq!(lamborghini.get_brand().as_ref().unwrap(), "Lamborghini");
        assert_eq!(lamborghini.max_speed(), 340.0);
        assert_eq!(lamborghini.get_autonomy_level(), 4);

        println!("✓ Multiple SportsCar instances maintain independent state");
    }
}
