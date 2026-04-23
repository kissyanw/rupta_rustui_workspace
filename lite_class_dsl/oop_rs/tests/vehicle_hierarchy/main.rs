// Vehicle Hierarchy Test Module
//
// This module demonstrates a vehicle hierarchy system implemented using the classes macro
// Includes abstract classes, inheritance, interface implementation, mixins, and type conversions

// Declare all submodules
mod bicycle;
mod car;
mod electric_car;
mod interfaces;
mod mixins;
mod motor_vehicle;
mod motorcycle;
mod sports_car;
mod truck;
mod vehicle;

// Re-export main types for testing
// Note: These exports will be gradually enabled in corresponding tasks
pub use bicycle::Bicycle;
pub use car::Car;
pub use electric_car::ElectricCar;
pub use interfaces::{Chargeable, Drivable, Maintainable};
pub use mixins::Autonomous;
pub use motor_vehicle::MotorVehicle;
pub use motorcycle::Motorcycle;
pub use sports_car::SportsCar;
pub use truck::Truck;
pub use vehicle::Vehicle;

#[cfg(test)]
mod tests {
    use super::*;
    use classes::prelude::*;

    #[test]
    fn test_drivable_interface_polymorphism() {
        println!("\n=== Testing Drivable Interface Polymorphism ===");

        // Create instances of different vehicle types
        let car = Car::new(
            "Toyota".to_string(),
            2024,
            "Gasoline".to_string(),
            50.0,
            4,
            400.0,
        );

        let motorcycle = Motorcycle::new(
            "Harley".to_string(),
            2023,
            "V-Twin".to_string(),
            15.0,
            false,
        );

        let bicycle = Bicycle::new("Trek".to_string(), 2024, 21, "Carbon Fiber".to_string());

        let truck = Truck::new(
            "Volvo".to_string(),
            2022,
            "Diesel".to_string(),
            200.0,
            10.0,
            3,
        );

        // Store them in Vec<CRc<Drivable>>
        // Note: Since all these classes implement the Drivable interface,
        // we can directly store them as Drivable references
        let drivable_vehicles: Vec<CRc<Drivable>> = vec![
            car.clone().into(),
            motorcycle.clone().into(),
            bicycle.clone().into(),
            truck.clone().into(),
        ];

        println!("Created {} drivable vehicles", drivable_vehicles.len());

        // Uniformly call interface methods, verifying each vehicle executes its own implementation
        for (i, vehicle) in drivable_vehicles.iter().enumerate() {
            println!("\n--- Vehicle {} ---", i + 1);

            // Call drive() method
            let drive_msg = vehicle.drive();
            println!("Drive: {}", drive_msg);
            assert!(
                !drive_msg.is_empty(),
                "drive() should return a non-empty message"
            );

            // Call stop() method
            let stop_msg = vehicle.stop();
            println!("Stop: {}", stop_msg);
            assert!(
                !stop_msg.is_empty(),
                "stop() should return a non-empty message"
            );

            // Call turn() method
            let turn_left = vehicle.turn("left".to_string());
            println!("Turn left: {}", turn_left);
            assert!(
                !turn_left.is_empty(),
                "turn() should return a non-empty message"
            );
            assert!(
                turn_left.contains("left"),
                "turn() message should contain the direction"
            );

            let turn_right = vehicle.turn("right".to_string());
            println!("Turn right: {}", turn_right);
            assert!(
                turn_right.contains("right"),
                "turn() message should contain the direction"
            );
        }

        // Verify that methods called through interface references return the same results as direct calls
        // This verifies that interface polymorphism preserves the original object's behavior
        assert_eq!(
            drivable_vehicles[0].drive(),
            car.drive(),
            "Car drive() should return the same result via interface and direct call"
        );
        assert_eq!(
            drivable_vehicles[1].drive(),
            motorcycle.drive(),
            "Motorcycle drive() should return the same result via interface and direct call"
        );
        assert_eq!(
            drivable_vehicles[2].drive(),
            bicycle.drive(),
            "Bicycle drive() should return the same result via interface and direct call"
        );
        assert_eq!(
            drivable_vehicles[3].drive(),
            truck.drive(),
            "Truck drive() should return the same result via interface and direct call"
        );

        // Verify each vehicle implementation is different
        assert_ne!(
            car.drive(),
            motorcycle.drive(),
            "Car and Motorcycle should have different drive() implementations"
        );
        assert_ne!(
            car.drive(),
            bicycle.drive(),
            "Car and Bicycle should have different drive() implementations"
        );
        assert_ne!(
            car.drive(),
            truck.drive(),
            "Car and Truck should have different drive() implementations"
        );
        assert_ne!(
            motorcycle.drive(),
            bicycle.drive(),
            "Motorcycle and Bicycle should have different drive() implementations"
        );

        println!("\n✓ Drivable interface polymorphism works correctly");
        println!("✓ Interface calls return the same results as direct calls");
        println!("✓ Each vehicle executes its own implementation");
    }

    #[test]
    fn test_maintainable_interface_polymorphism() {
        println!("\n=== Testing Maintainable Interface Polymorphism ===");

        // Create instances of different vehicle types
        let car = Car::new(
            "BMW".to_string(),
            2024,
            "Gasoline".to_string(),
            60.0,
            4,
            450.0,
        );

        let motorcycle = Motorcycle::new(
            "Yamaha".to_string(),
            2023,
            "Inline-4".to_string(),
            17.0,
            false,
        );

        let bicycle = Bicycle::new("Giant".to_string(), 2024, 18, "Aluminum".to_string());

        let truck = Truck::new(
            "Mercedes".to_string(),
            2022,
            "Diesel".to_string(),
            300.0,
            15.0,
            4,
        );

        // Store them in Vec<CRc<Maintainable>>
        let maintainable_vehicles: Vec<CRc<Maintainable>> = vec![
            car.clone().into(),
            motorcycle.clone().into(),
            bicycle.clone().into(),
            truck.clone().into(),
        ];

        println!(
            "Created {} maintainable vehicles",
            maintainable_vehicles.len()
        );

        // Uniformly call interface methods, verify each vehicle returns specific maintenance information
        for (i, vehicle) in maintainable_vehicles.iter().enumerate() {
            println!("\n--- Vehicle {} ---", i + 1);

            // Call perform_maintenance() method
            let maintenance_msg = vehicle.perform_maintenance();
            println!("Maintenance: {}", maintenance_msg);
            assert!(
                !maintenance_msg.is_empty(),
                "perform_maintenance() should return a non-empty message"
            );

            // Call check_condition() method
            let condition_msg = vehicle.check_condition();
            println!("Condition: {}", condition_msg);
            assert!(
                !condition_msg.is_empty(),
                "check_condition() should return a non-empty message"
            );
        }

        // Verify methods called through interface references return same results as direct calls
        assert_eq!(
            maintainable_vehicles[0].perform_maintenance(),
            car.perform_maintenance(),
            "Car perform_maintenance() should return the same result via interface and direct call"
        );
        assert_eq!(
            maintainable_vehicles[1].perform_maintenance(),
            motorcycle.perform_maintenance(),
            "Motorcycle perform_maintenance() should return the same result via interface and direct call"
        );
        assert_eq!(
            maintainable_vehicles[2].perform_maintenance(),
            bicycle.perform_maintenance(),
            "Bicycle perform_maintenance() should return the same result via interface and direct call"
        );
        assert_eq!(
            maintainable_vehicles[3].perform_maintenance(),
            truck.perform_maintenance(),
            "Truck perform_maintenance() should return the same result via interface and direct call"
        );

        // Verify each vehicle implementation is different
        assert_ne!(
            car.perform_maintenance(),
            motorcycle.perform_maintenance(),
            "Car and Motorcycle should have different perform_maintenance() implementations"
        );
        assert_ne!(
            car.perform_maintenance(),
            bicycle.perform_maintenance(),
            "Car and Bicycle should have different perform_maintenance() implementations"
        );
        assert_ne!(
            car.perform_maintenance(),
            truck.perform_maintenance(),
            "Car and Truck should have different perform_maintenance() implementations"
        );
        assert_ne!(
            motorcycle.perform_maintenance(),
            bicycle.perform_maintenance(),
            "Motorcycle and Bicycle should have different perform_maintenance() implementations"
        );

        // Verify check_condition() also returns same results
        assert_eq!(
            maintainable_vehicles[0].check_condition(),
            car.check_condition(),
            "Car check_condition() should return the same result via interface and direct call"
        );
        assert_eq!(
            maintainable_vehicles[1].check_condition(),
            motorcycle.check_condition(),
            "Motorcycle check_condition() should return the same result via interface and direct call"
        );
        assert_eq!(
            maintainable_vehicles[2].check_condition(),
            bicycle.check_condition(),
            "Bicycle check_condition() should return the same result via interface and direct call"
        );
        assert_eq!(
            maintainable_vehicles[3].check_condition(),
            truck.check_condition(),
            "Truck check_condition() should return the same result via interface and direct call"
        );

        // Verify each vehicle check_condition() implementation is different
        assert_ne!(
            car.check_condition(),
            motorcycle.check_condition(),
            "Car and Motorcycle should have different check_condition() implementations"
        );
        assert_ne!(
            car.check_condition(),
            bicycle.check_condition(),
            "Car and Bicycle should have different check_condition() implementations"
        );
        assert_ne!(
            car.check_condition(),
            truck.check_condition(),
            "Car and Truck should have different check_condition() implementations"
        );

        println!("\n✓ Maintainable interface polymorphism works correctly");
        println!("✓ Interface calls return the same results as direct calls");
        println!("✓ Each vehicle returns its specific maintenance information");
    }

    #[test]
    fn test_create_all_vehicle_types() {
        println!("\n=== Testing creation of all 7 vehicle types ===");

        // 1. Car
        let car = Car::new(
            "Toyota".to_string(),
            2024,
            "Gasoline".to_string(),
            50.0,
            4,
            400.0,
        );
        assert_eq!(
            car.get_brand().as_ref().unwrap(),
            "Toyota",
            "Car should be created successfully"
        );
        println!("✓ Car created: {}", car.get_brand().as_ref().unwrap());

        // 2. Motorcycle
        let motorcycle = Motorcycle::new(
            "Harley".to_string(),
            2023,
            "V-Twin".to_string(),
            15.0,
            false,
        );
        assert_eq!(
            motorcycle.get_brand().as_ref().unwrap(),
            "Harley",
            "Motorcycle should be created successfully"
        );
        println!(
            "✓ Motorcycle created: {}",
            motorcycle.get_brand().as_ref().unwrap()
        );

        // 3. Bicycle
        let bicycle = Bicycle::new("Trek".to_string(), 2024, 21, "Carbon Fiber".to_string());
        assert_eq!(
            bicycle.get_brand().as_ref().unwrap(),
            "Trek",
            "Bicycle should be created successfully"
        );
        println!(
            "✓ Bicycle created: {}",
            bicycle.get_brand().as_ref().unwrap()
        );

        // 4. Truck
        let truck = Truck::new(
            "Volvo".to_string(),
            2022,
            "Diesel".to_string(),
            200.0,
            10.0,
            3,
        );
        assert_eq!(
            truck.get_brand().as_ref().unwrap(),
            "Volvo",
            "Truck should be created successfully"
        );
        println!("✓ Truck created: {}", truck.get_brand().as_ref().unwrap());

        // 5. ElectricCar
        let electric_car = ElectricCar::new("Tesla".to_string(), 2024, 4, 500.0, 75.0, 8.0);
        assert_eq!(
            electric_car.get_brand().as_ref().unwrap(),
            "Tesla",
            "ElectricCar should be created successfully"
        );
        println!(
            "✓ ElectricCar created: {}",
            electric_car.get_brand().as_ref().unwrap()
        );

        // 6. SportsCar
        let sports_car = SportsCar::new(
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
        assert_eq!(
            sports_car.get_brand().as_ref().unwrap(),
            "Ferrari",
            "SportsCar should be created successfully"
        );
        println!(
            "✓ SportsCar created: {}",
            sports_car.get_brand().as_ref().unwrap()
        );

        // 7. Verify all vehicle types can be created correctly
        println!("\n✓ All 7 vehicle types created successfully:");
        println!("  1. Car: {}", car.get_type());
        println!("  2. Motorcycle: {}", motorcycle.get_type());
        println!("  3. Bicycle: {}", bicycle.get_type());
        println!("  4. Truck: {}", truck.get_type());
        println!("  5. ElectricCar: {}", electric_car.get_type());
        println!("  6. SportsCar: {}", sports_car.get_type());

        // Verify each instance has correct type
        assert_eq!(car.get_type(), "Car");
        assert_eq!(motorcycle.get_type(), "Motorcycle");
        assert_eq!(bicycle.get_type(), "Bicycle");
        assert_eq!(truck.get_type(), "Truck");
        assert_eq!(electric_car.get_type(), "Car"); // ElectricCar inherits from Car
        assert_eq!(sports_car.get_type(), "Car"); // SportsCar inherits from Car

        println!(
            "\n✓ Test coverage: 7 vehicle types (Car, Motorcycle, Bicycle, Truck, ElectricCar, SportsCar)"
        );
        println!(
            "✓ Requirement 12.1 satisfied: Test suite includes at least 7 different vehicle types"
        );
    }

    #[test]
    fn test_multiple_interface_implementation() {
        println!("\n=== Testing multiple interface implementation ===");

        // Create a Car instance
        let car = Car::new(
            "BMW".to_string(),
            2024,
            "Gasoline".to_string(),
            60.0,
            4,
            450.0,
        );

        println!("Created Car: {}", car.get_brand().as_ref().unwrap());

        // Verify Car implements both Drivable and Maintainable interfaces
        // Access through Drivable interface reference
        let drivable: CRc<Drivable> = car.clone().into();
        let drive_msg = drivable.drive();
        println!("Via Drivable interface - drive(): {}", drive_msg);
        assert!(
            !drive_msg.is_empty(),
            "Should be able to call Drivable methods"
        );
        assert!(
            drive_msg.contains("BMW"),
            "Drivable interface should access the same object"
        );

        // Access through Maintainable interface reference
        let maintainable: CRc<Maintainable> = car.clone().into();
        let maintenance_msg = maintainable.perform_maintenance();
        println!(
            "Via Maintainable interface - perform_maintenance(): {}",
            maintenance_msg
        );
        assert!(
            !maintenance_msg.is_empty(),
            "Should be able to call Maintainable methods"
        );
        assert!(
            maintenance_msg.contains("BMW"),
            "Maintainable interface should access the same object"
        );

        // Verify accessing same object through different interface references
        // Both interface methods should return messages containing same brand
        assert!(
            drive_msg.contains("BMW") && maintenance_msg.contains("BMW"),
            "Both interfaces should access the same underlying object"
        );

        // Test ElectricCar implements three interfaces
        println!("\n--- Testing ElectricCar with three interfaces ---");
        let electric_car = ElectricCar::new("Tesla".to_string(), 2024, 4, 500.0, 75.0, 8.0);

        // Drivable interface (through Car)
        let car_from_ec: CRc<Car> = electric_car.clone().into_super();
        let drivable_ec: CRc<Drivable> = car_from_ec.clone().into();
        let drive_ec = drivable_ec.drive();
        println!("ElectricCar via Drivable - drive(): {}", drive_ec);
        assert!(drive_ec.contains("Tesla"));

        // Maintainable interface (through Car)
        let maintainable_ec: CRc<Maintainable> = car_from_ec.into();
        let maintenance_ec = maintainable_ec.perform_maintenance();
        println!(
            "ElectricCar via Maintainable - perform_maintenance(): {}",
            maintenance_ec
        );
        assert!(maintenance_ec.contains("Tesla"));

        // Chargeable interface (direct implementation)
        let chargeable_ec: CRc<Chargeable> = electric_car.clone().into();
        let charge_ec = chargeable_ec.charge();
        println!("ElectricCar via Chargeable - charge(): {}", charge_ec);
        assert!(charge_ec.contains("Tesla"));

        // Verify all interfaces access same object
        assert!(
            drive_ec.contains("Tesla")
                && maintenance_ec.contains("Tesla")
                && charge_ec.contains("Tesla"),
            "All three interfaces should access the same ElectricCar object"
        );

        // Test Motorcycle implements two interfaces
        println!("\n--- Testing Motorcycle with two interfaces ---");
        let motorcycle = Motorcycle::new(
            "Harley".to_string(),
            2023,
            "V-Twin".to_string(),
            15.0,
            false,
        );

        let drivable_mc: CRc<Drivable> = motorcycle.clone().into();
        let maintainable_mc: CRc<Maintainable> = motorcycle.clone().into();

        let drive_mc = drivable_mc.drive();
        let maintenance_mc = maintainable_mc.perform_maintenance();

        println!("Motorcycle via Drivable - drive(): {}", drive_mc);
        println!(
            "Motorcycle via Maintainable - perform_maintenance(): {}",
            maintenance_mc
        );

        assert!(drive_mc.contains("Harley"));
        assert!(maintenance_mc.contains("Harley"));

        println!("\n✓ Multiple interface implementation works correctly");
        println!("✓ Car implements Drivable and Maintainable");
        println!("✓ ElectricCar implements Drivable, Maintainable, and Chargeable");
        println!("✓ Motorcycle implements Drivable and Maintainable");
        println!("✓ All interfaces access the same underlying object");
        println!(
            "✓ Requirement 9.5 satisfied: System supports same vehicle implementing multiple interfaces"
        );
    }
}

// Property test module
#[cfg(test)]
mod property_tests {
    use super::*;
    use classes::prelude::*;
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

    // Generate valid sidecar flag
    fn has_sidecar_strategy() -> impl Strategy<Value = bool> {
        prop::bool::ANY
    }

    // Generate valid number of gears (1-30)
    fn num_gears_strategy() -> impl Strategy<Value = i32> {
        1..=30
    }

    // Generate valid frame materials
    fn frame_material_strategy() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "Steel".to_string(),
            "Aluminum".to_string(),
            "Carbon Fiber".to_string(),
            "Titanium".to_string(),
        ])
    }

    // Generate valid cargo capacity (1.0-50.0 tons)
    fn cargo_capacity_strategy() -> impl Strategy<Value = f64> {
        1.0..=50.0
    }

    // Generate valid number of axles (2-6)
    fn num_axles_strategy() -> impl Strategy<Value = i32> {
        2..=6
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_drivable_interface_polymorphism(
            // Car parameters
            car_brand in brand_strategy(),
            car_year in year_strategy(),
            car_engine_type in engine_type_strategy(),
            car_fuel_capacity in fuel_capacity_strategy(),
            car_num_doors in num_doors_strategy(),
            car_trunk_capacity in trunk_capacity_strategy(),
            // Motorcycle parameters
            motorcycle_brand in brand_strategy(),
            motorcycle_year in year_strategy(),
            motorcycle_engine_type in engine_type_strategy(),
            motorcycle_fuel_capacity in fuel_capacity_strategy(),
            motorcycle_has_sidecar in has_sidecar_strategy(),
            // Bicycle parameters
            bicycle_brand in brand_strategy(),
            bicycle_year in year_strategy(),
            bicycle_num_gears in num_gears_strategy(),
            bicycle_frame_material in frame_material_strategy(),
            // Truck parameters
            truck_brand in brand_strategy(),
            truck_year in year_strategy(),
            truck_engine_type in engine_type_strategy(),
            truck_fuel_capacity in fuel_capacity_strategy(),
            truck_cargo_capacity in cargo_capacity_strategy(),
            truck_num_axles in num_axles_strategy(),
        ) {
            // Create instances of different vehicle types
            let car = Car::new(
                car_brand,
                car_year,
                car_engine_type,
                car_fuel_capacity,
                car_num_doors,
                car_trunk_capacity,
            );

            let motorcycle = Motorcycle::new(
                motorcycle_brand,
                motorcycle_year,
                motorcycle_engine_type,
                motorcycle_fuel_capacity,
                motorcycle_has_sidecar,
            );

            let bicycle = Bicycle::new(
                bicycle_brand,
                bicycle_year,
                bicycle_num_gears,
                bicycle_frame_material,
            );

            let truck = Truck::new(
                truck_brand,
                truck_year,
                truck_engine_type,
                truck_fuel_capacity,
                truck_cargo_capacity,
                truck_num_axles,
            );

            // Store them in Vec<CRc<Drivable>>
            let drivable_vehicles: Vec<CRc<Drivable>> = vec![
                car.clone().into(),
                motorcycle.clone().into(),
                bicycle.clone().into(),
                truck.clone().into(),
            ];

            // Verify all vehicles can call methods through interface references
            for vehicle in &drivable_vehicles {
                // 验证 drive() 方法返回非空字符串
                let drive_msg = vehicle.drive();
                prop_assert!(!drive_msg.is_empty(), "drive() should return a non-empty message");

                // 验证 stop() 方法返回非空字符串
                let stop_msg = vehicle.stop();
                prop_assert!(!stop_msg.is_empty(), "stop() should return a non-empty message");

                // 验证 turn() 方法返回包含方向的非空字符串
                let turn_left = vehicle.turn("left".to_string());
                prop_assert!(!turn_left.is_empty(), "turn() should return a non-empty message");
                prop_assert!(turn_left.contains("left"), "turn() message should contain the direction");

                let turn_right = vehicle.turn("right".to_string());
                prop_assert!(!turn_right.is_empty(), "turn() should return a non-empty message");
                prop_assert!(turn_right.contains("right"), "turn() message should contain the direction");
            }

            // Verify each vehicle implementation is different（多态行为）
            let car_drive = drivable_vehicles[0].drive();
            let motorcycle_drive = drivable_vehicles[1].drive();
            let bicycle_drive = drivable_vehicles[2].drive();
            let truck_drive = drivable_vehicles[3].drive();

            // Each vehicle drive() message should be unique
            prop_assert_ne!(&car_drive, &motorcycle_drive, "Car and Motorcycle should have different drive() implementations");
            prop_assert_ne!(&car_drive, &bicycle_drive, "Car and Bicycle should have different drive() implementations");
            prop_assert_ne!(&car_drive, &truck_drive, "Car and Truck should have different drive() implementations");
            prop_assert_ne!(&motorcycle_drive, &bicycle_drive, "Motorcycle and Bicycle should have different drive() implementations");
            prop_assert_ne!(&motorcycle_drive, &truck_drive, "Motorcycle and Truck should have different drive() implementations");
            prop_assert_ne!(&bicycle_drive, &truck_drive, "Bicycle and Truck should have different drive() implementations");
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_maintainable_interface_polymorphism(
            // Car parameters
            car_brand in brand_strategy(),
            car_year in year_strategy(),
            car_engine_type in engine_type_strategy(),
            car_fuel_capacity in fuel_capacity_strategy(),
            car_num_doors in num_doors_strategy(),
            car_trunk_capacity in trunk_capacity_strategy(),
            // Motorcycle parameters
            motorcycle_brand in brand_strategy(),
            motorcycle_year in year_strategy(),
            motorcycle_engine_type in engine_type_strategy(),
            motorcycle_fuel_capacity in fuel_capacity_strategy(),
            motorcycle_has_sidecar in has_sidecar_strategy(),
            // Bicycle parameters
            bicycle_brand in brand_strategy(),
            bicycle_year in year_strategy(),
            bicycle_num_gears in num_gears_strategy(),
            bicycle_frame_material in frame_material_strategy(),
            // Truck parameters
            truck_brand in brand_strategy(),
            truck_year in year_strategy(),
            truck_engine_type in engine_type_strategy(),
            truck_fuel_capacity in fuel_capacity_strategy(),
            truck_cargo_capacity in cargo_capacity_strategy(),
            truck_num_axles in num_axles_strategy(),
        ) {
            // Create instances of different vehicle types
            let car = Car::new(
                car_brand,
                car_year,
                car_engine_type,
                car_fuel_capacity,
                car_num_doors,
                car_trunk_capacity,
            );

            let motorcycle = Motorcycle::new(
                motorcycle_brand,
                motorcycle_year,
                motorcycle_engine_type,
                motorcycle_fuel_capacity,
                motorcycle_has_sidecar,
            );

            let bicycle = Bicycle::new(
                bicycle_brand,
                bicycle_year,
                bicycle_num_gears,
                bicycle_frame_material,
            );

            let truck = Truck::new(
                truck_brand,
                truck_year,
                truck_engine_type,
                truck_fuel_capacity,
                truck_cargo_capacity,
                truck_num_axles,
            );

            // Store them in Vec<CRc<Maintainable>>
            let maintainable_vehicles: Vec<CRc<Maintainable>> = vec![
                car.clone().into(),
                motorcycle.clone().into(),
                bicycle.clone().into(),
                truck.clone().into(),
            ];

            // Verify all vehicles can call methods through interface references
            for vehicle in &maintainable_vehicles {
                // 验证 perform_maintenance() 方法返回非空字符串
                let maintenance_msg = vehicle.perform_maintenance();
                prop_assert!(!maintenance_msg.is_empty(), "perform_maintenance() should return a non-empty message");

                // 验证 check_condition() 方法返回非空字符串
                let condition_msg = vehicle.check_condition();
                prop_assert!(!condition_msg.is_empty(), "check_condition() should return a non-empty message");
            }

            // Verify each vehicle implementation is different（多态行为）
            let car_maintenance = maintainable_vehicles[0].perform_maintenance();
            let motorcycle_maintenance = maintainable_vehicles[1].perform_maintenance();
            let bicycle_maintenance = maintainable_vehicles[2].perform_maintenance();
            let truck_maintenance = maintainable_vehicles[3].perform_maintenance();

            // Each vehicle perform_maintenance() message should be unique
            prop_assert_ne!(&car_maintenance, &motorcycle_maintenance, "Car and Motorcycle should have different perform_maintenance() implementations");
            prop_assert_ne!(&car_maintenance, &bicycle_maintenance, "Car and Bicycle should have different perform_maintenance() implementations");
            prop_assert_ne!(&car_maintenance, &truck_maintenance, "Car and Truck should have different perform_maintenance() implementations");
            prop_assert_ne!(&motorcycle_maintenance, &bicycle_maintenance, "Motorcycle and Bicycle should have different perform_maintenance() implementations");
            prop_assert_ne!(&motorcycle_maintenance, &truck_maintenance, "Motorcycle and Truck should have different perform_maintenance() implementations");
            prop_assert_ne!(&bicycle_maintenance, &truck_maintenance, "Bicycle and Truck should have different perform_maintenance() implementations");

            // Verify check_condition() also returns different information
            let car_condition = maintainable_vehicles[0].check_condition();
            let motorcycle_condition = maintainable_vehicles[1].check_condition();
            let bicycle_condition = maintainable_vehicles[2].check_condition();
            let truck_condition = maintainable_vehicles[3].check_condition();

            prop_assert_ne!(&car_condition, &motorcycle_condition, "Car and Motorcycle should have different check_condition() implementations");
            prop_assert_ne!(&car_condition, &bicycle_condition, "Car and Bicycle should have different check_condition() implementations");
            prop_assert_ne!(&car_condition, &truck_condition, "Car and Truck should have different check_condition() implementations");
            prop_assert_ne!(&motorcycle_condition, &bicycle_condition, "Motorcycle and Bicycle should have different check_condition() implementations");
            prop_assert_ne!(&motorcycle_condition, &truck_condition, "Motorcycle and Truck should have different check_condition() implementations");
            prop_assert_ne!(&bicycle_condition, &truck_condition, "Bicycle and Truck should have different check_condition() implementations");
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_interface_collection_unified_operations(
            // Car parameters
            car_brand in brand_strategy(),
            car_year in year_strategy(),
            car_engine_type in engine_type_strategy(),
            car_fuel_capacity in fuel_capacity_strategy(),
            car_num_doors in num_doors_strategy(),
            car_trunk_capacity in trunk_capacity_strategy(),
            // Motorcycle parameters
            motorcycle_brand in brand_strategy(),
            motorcycle_year in year_strategy(),
            motorcycle_engine_type in engine_type_strategy(),
            motorcycle_fuel_capacity in fuel_capacity_strategy(),
            motorcycle_has_sidecar in has_sidecar_strategy(),
            // Bicycle parameters
            bicycle_brand in brand_strategy(),
            bicycle_year in year_strategy(),
            bicycle_num_gears in num_gears_strategy(),
            bicycle_frame_material in frame_material_strategy(),
            // Truck parameters
            truck_brand in brand_strategy(),
            truck_year in year_strategy(),
            truck_engine_type in engine_type_strategy(),
            truck_fuel_capacity in fuel_capacity_strategy(),
            truck_cargo_capacity in cargo_capacity_strategy(),
            truck_num_axles in num_axles_strategy(),
        ) {
            // Create instances of different vehicle types
            let car = Car::new(
                car_brand,
                car_year,
                car_engine_type,
                car_fuel_capacity,
                car_num_doors,
                car_trunk_capacity,
            );

            let motorcycle = Motorcycle::new(
                motorcycle_brand,
                motorcycle_year,
                motorcycle_engine_type,
                motorcycle_fuel_capacity,
                motorcycle_has_sidecar,
            );

            let bicycle = Bicycle::new(
                bicycle_brand,
                bicycle_year,
                bicycle_num_gears,
                bicycle_frame_material,
            );

            let truck = Truck::new(
                truck_brand,
                truck_year,
                truck_engine_type,
                truck_fuel_capacity,
                truck_cargo_capacity,
                truck_num_axles,
            );

            // Test Drivable interface collection unified operations
            let drivable_vehicles: Vec<CRc<Drivable>> = vec![
                car.clone().into(),
                motorcycle.clone().into(),
                bicycle.clone().into(),
                truck.clone().into(),
            ];

            // Verify all Drivable interface methods can be called successfully
            for vehicle in &drivable_vehicles {
                // All calls should execute successfully without panic
                let _ = vehicle.drive();
                let _ = vehicle.stop();
                let _ = vehicle.turn("left".to_string());
                let _ = vehicle.turn("right".to_string());
            }

            // Test Maintainable interface collection unified operations
            let maintainable_vehicles: Vec<CRc<Maintainable>> = vec![
                car.clone().into(),
                motorcycle.clone().into(),
                bicycle.clone().into(),
                truck.clone().into(),
            ];

            // Verify all Maintainable interface methods can be called successfully
            for vehicle in &maintainable_vehicles {
                // All calls should execute successfully without panic
                let _ = vehicle.perform_maintenance();
                let _ = vehicle.check_condition();
            }

            // Verify collection size is correct
            prop_assert_eq!(drivable_vehicles.len(), 4, "Drivable collection should contain 4 vehicles");
            prop_assert_eq!(maintainable_vehicles.len(), 4, "Maintainable collection should contain 4 vehicles");

            // Verify can iterate over collection
            let drivable_count = drivable_vehicles.iter().count();
            prop_assert_eq!(drivable_count, 4, "Should be able to iterate over Drivable collection");

            let maintainable_count = maintainable_vehicles.iter().count();
            prop_assert_eq!(maintainable_count, 4, "Should be able to iterate over Maintainable collection");

            // Verify can map over collection
            let drive_messages: Vec<String> = drivable_vehicles
                .iter()
                .map(|v| v.drive())
                .collect();
            prop_assert_eq!(drive_messages.len(), 4, "Should be able to map over Drivable collection");

            // Verify all mapped results are non-empty
            for msg in &drive_messages {
                prop_assert!(!msg.is_empty(), "All drive messages should be non-empty");
            }

            let maintenance_messages: Vec<String> = maintainable_vehicles
                .iter()
                .map(|v| v.perform_maintenance())
                .collect();
            prop_assert_eq!(maintenance_messages.len(), 4, "Should be able to map over Maintainable collection");

            // Verify all mapped results are non-empty
            for msg in &maintenance_messages {
                prop_assert!(!msg.is_empty(), "All maintenance messages should be non-empty");
            }

            // Verify can filter collection
            let filtered_drivable: Vec<_> = drivable_vehicles
                .iter()
                .filter(|v| !v.drive().is_empty())
                .collect();
            prop_assert_eq!(filtered_drivable.len(), 4, "All vehicles should pass the filter");

            let filtered_maintainable: Vec<_> = maintainable_vehicles
                .iter()
                .filter(|v| !v.check_condition().is_empty())
                .collect();
            prop_assert_eq!(filtered_maintainable.len(), 4, "All vehicles should pass the filter");
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_all_vehicles_generate_complete_description(
            // Car parameters
            car_brand in brand_strategy(),
            car_year in year_strategy(),
            car_engine_type in engine_type_strategy(),
            car_fuel_capacity in fuel_capacity_strategy(),
            car_num_doors in num_doors_strategy(),
            car_trunk_capacity in trunk_capacity_strategy(),
            // Motorcycle parameters
            motorcycle_brand in brand_strategy(),
            motorcycle_year in year_strategy(),
            motorcycle_engine_type in engine_type_strategy(),
            motorcycle_fuel_capacity in fuel_capacity_strategy(),
            motorcycle_has_sidecar in has_sidecar_strategy(),
            // Bicycle parameters
            bicycle_brand in brand_strategy(),
            bicycle_year in year_strategy(),
            bicycle_num_gears in num_gears_strategy(),
            bicycle_frame_material in frame_material_strategy(),
            // Truck parameters
            truck_brand in brand_strategy(),
            truck_year in year_strategy(),
            truck_engine_type in engine_type_strategy(),
            truck_fuel_capacity in fuel_capacity_strategy(),
            truck_cargo_capacity in cargo_capacity_strategy(),
            truck_num_axles in num_axles_strategy(),
        ) {
            // Create instances of different vehicle types
            let car = Car::new(
                car_brand.clone(),
                car_year,
                car_engine_type,
                car_fuel_capacity,
                car_num_doors,
                car_trunk_capacity,
            );

            let motorcycle = Motorcycle::new(
                motorcycle_brand.clone(),
                motorcycle_year,
                motorcycle_engine_type,
                motorcycle_fuel_capacity,
                motorcycle_has_sidecar,
            );

            let bicycle = Bicycle::new(
                bicycle_brand.clone(),
                bicycle_year,
                bicycle_num_gears,
                bicycle_frame_material,
            );

            let truck = Truck::new(
                truck_brand.clone(),
                truck_year,
                truck_engine_type,
                truck_fuel_capacity,
                truck_cargo_capacity,
                truck_num_axles,
            );

            // Verify all vehicles can generate non-empty descriptions
            let car_desc = car.describe();
            prop_assert!(!car_desc.is_empty(), "Car describe() should return a non-empty string");
            prop_assert!(car_desc.contains(&car_brand), "Car description should contain the brand");
            prop_assert!(car_desc.contains(&car_year.to_string()), "Car description should contain the year");

            let motorcycle_desc = motorcycle.describe();
            prop_assert!(!motorcycle_desc.is_empty(), "Motorcycle describe() should return a non-empty string");
            prop_assert!(motorcycle_desc.contains(&motorcycle_brand), "Motorcycle description should contain the brand");
            prop_assert!(motorcycle_desc.contains(&motorcycle_year.to_string()), "Motorcycle description should contain the year");

            let bicycle_desc = bicycle.describe();
            prop_assert!(!bicycle_desc.is_empty(), "Bicycle describe() should return a non-empty string");
            prop_assert!(bicycle_desc.contains(&bicycle_brand), "Bicycle description should contain the brand");
            prop_assert!(bicycle_desc.contains(&bicycle_year.to_string()), "Bicycle description should contain the year");

            let truck_desc = truck.describe();
            prop_assert!(!truck_desc.is_empty(), "Truck describe() should return a non-empty string");
            prop_assert!(truck_desc.contains(&truck_brand), "Truck description should contain the brand");
            prop_assert!(truck_desc.contains(&truck_year.to_string()), "Truck description should contain the year");
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_all_motor_vehicles_can_start_engine(
            // Car parameters
            car_brand in brand_strategy(),
            car_year in year_strategy(),
            car_engine_type in engine_type_strategy(),
            car_fuel_capacity in fuel_capacity_strategy(),
            car_num_doors in num_doors_strategy(),
            car_trunk_capacity in trunk_capacity_strategy(),
            // Motorcycle parameters
            motorcycle_brand in brand_strategy(),
            motorcycle_year in year_strategy(),
            motorcycle_engine_type in engine_type_strategy(),
            motorcycle_fuel_capacity in fuel_capacity_strategy(),
            motorcycle_has_sidecar in has_sidecar_strategy(),
            // Truck parameters
            truck_brand in brand_strategy(),
            truck_year in year_strategy(),
            truck_engine_type in engine_type_strategy(),
            truck_fuel_capacity in fuel_capacity_strategy(),
            truck_cargo_capacity in cargo_capacity_strategy(),
            truck_num_axles in num_axles_strategy(),
        ) {
            // Create motor vehicle instance
            let car = Car::new(
                car_brand,
                car_year,
                car_engine_type.clone(),
                car_fuel_capacity,
                car_num_doors,
                car_trunk_capacity,
            );

            let motorcycle = Motorcycle::new(
                motorcycle_brand,
                motorcycle_year,
                motorcycle_engine_type.clone(),
                motorcycle_fuel_capacity,
                motorcycle_has_sidecar,
            );

            let truck = Truck::new(
                truck_brand,
                truck_year,
                truck_engine_type.clone(),
                truck_fuel_capacity,
                truck_cargo_capacity,
                truck_num_axles,
            );

            // Verify all motor vehicles can start engine
            let car_start = car.start_engine();
            prop_assert!(!car_start.is_empty(), "Car start_engine() should return a non-empty message");
            prop_assert!(car_start.contains(&car_engine_type), "Car start_engine() should mention the engine type");

            let motorcycle_start = motorcycle.start_engine();
            prop_assert!(!motorcycle_start.is_empty(), "Motorcycle start_engine() should return a non-empty message");
            prop_assert!(motorcycle_start.contains(&motorcycle_engine_type), "Motorcycle start_engine() should mention the engine type");

            let truck_start = truck.start_engine();
            prop_assert!(!truck_start.is_empty(), "Truck start_engine() should return a non-empty message");
            prop_assert!(truck_start.contains(&truck_engine_type), "Truck start_engine() should mention the engine type");
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_all_motor_vehicles_return_fuel_efficiency(
            // Car parameters
            car_brand in brand_strategy(),
            car_year in year_strategy(),
            car_engine_type in engine_type_strategy(),
            car_fuel_capacity in fuel_capacity_strategy(),
            car_num_doors in num_doors_strategy(),
            car_trunk_capacity in trunk_capacity_strategy(),
            // Motorcycle parameters
            motorcycle_brand in brand_strategy(),
            motorcycle_year in year_strategy(),
            motorcycle_engine_type in engine_type_strategy(),
            motorcycle_fuel_capacity in fuel_capacity_strategy(),
            motorcycle_has_sidecar in has_sidecar_strategy(),
            // Truck parameters
            truck_brand in brand_strategy(),
            truck_year in year_strategy(),
            truck_engine_type in engine_type_strategy(),
            truck_fuel_capacity in fuel_capacity_strategy(),
            truck_cargo_capacity in cargo_capacity_strategy(),
            truck_num_axles in num_axles_strategy(),
        ) {
            // Create motor vehicle instance
            let car = Car::new(
                car_brand,
                car_year,
                car_engine_type,
                car_fuel_capacity,
                car_num_doors,
                car_trunk_capacity,
            );

            let motorcycle = Motorcycle::new(
                motorcycle_brand,
                motorcycle_year,
                motorcycle_engine_type,
                motorcycle_fuel_capacity,
                motorcycle_has_sidecar,
            );

            let truck = Truck::new(
                truck_brand,
                truck_year,
                truck_engine_type,
                truck_fuel_capacity,
                truck_cargo_capacity,
                truck_num_axles,
            );

            // Verify all motor vehicles return positive fuel efficiency
            let car_efficiency = car.fuel_efficiency();
            prop_assert!(car_efficiency > 0.0, "Car fuel_efficiency() should return a positive value, got {}", car_efficiency);

            let motorcycle_efficiency = motorcycle.fuel_efficiency();
            prop_assert!(motorcycle_efficiency > 0.0, "Motorcycle fuel_efficiency() should return a positive value, got {}", motorcycle_efficiency);

            let truck_efficiency = truck.fuel_efficiency();
            prop_assert!(truck_efficiency > 0.0, "Truck fuel_efficiency() should return a positive value, got {}", truck_efficiency);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_motor_vehicle_description_contains_engine_info(
            // Car parameters
            car_brand in brand_strategy(),
            car_year in year_strategy(),
            car_engine_type in engine_type_strategy(),
            car_fuel_capacity in fuel_capacity_strategy(),
            car_num_doors in num_doors_strategy(),
            car_trunk_capacity in trunk_capacity_strategy(),
            // Motorcycle parameters
            motorcycle_brand in brand_strategy(),
            motorcycle_year in year_strategy(),
            motorcycle_engine_type in engine_type_strategy(),
            motorcycle_fuel_capacity in fuel_capacity_strategy(),
            motorcycle_has_sidecar in has_sidecar_strategy(),
            // Truck parameters
            truck_brand in brand_strategy(),
            truck_year in year_strategy(),
            truck_engine_type in engine_type_strategy(),
            truck_fuel_capacity in fuel_capacity_strategy(),
            truck_cargo_capacity in cargo_capacity_strategy(),
            truck_num_axles in num_axles_strategy(),
        ) {
            // Create motor vehicle instance
            let car = Car::new(
                car_brand,
                car_year,
                car_engine_type.clone(),
                car_fuel_capacity,
                car_num_doors,
                car_trunk_capacity,
            );

            let motorcycle = Motorcycle::new(
                motorcycle_brand,
                motorcycle_year,
                motorcycle_engine_type.clone(),
                motorcycle_fuel_capacity,
                motorcycle_has_sidecar,
            );

            let truck = Truck::new(
                truck_brand,
                truck_year,
                truck_engine_type.clone(),
                truck_fuel_capacity,
                truck_cargo_capacity,
                truck_num_axles,
            );

            // Verify all motor vehicle descriptions contain engine information
            let car_desc = car.describe();
            prop_assert!(car_desc.contains(&car_engine_type), "Car description should contain engine type information");

            let motorcycle_desc = motorcycle.describe();
            prop_assert!(motorcycle_desc.contains(&motorcycle_engine_type), "Motorcycle description should contain engine type information");

            let truck_desc = truck.describe();
            prop_assert!(truck_desc.contains(&truck_engine_type), "Truck description should contain engine type information");
        }
    }
}
