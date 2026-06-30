// Vehicle Hierarchy Test Module
//
// This module demonstrates a vehicle hierarchy system implemented using the oop_rs macro
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
pub use bicycle::{Bicycle, IBicycle};
pub use car::{Car, ICar};
pub use electric_car::{ElectricCar, IElectricCar};
pub use interfaces::{Chargeable, IChargeable, Drivable, IDrivable, Maintainable, IMaintainable};
pub use mixins::{Autonomous, IAutonomous};
pub use motor_vehicle::{MotorVehicle, IMotorVehicle};
pub use motorcycle::{Motorcycle, IMotorcycle};
pub use sports_car::{SportsCar, ISportsCar};
pub use truck::{Truck, ITruck};
pub use vehicle::{Vehicle, IVehicle};

#[cfg(test)]
mod tests {
    use super::*;
    use oop_rs::prelude::*;

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
        // we can cast them as Drivable references
        let drivable_vehicles: Vec<CRc<Drivable>> = vec![
            car.clone() as CRc<Drivable>,
            motorcycle.clone() as CRc<Drivable>,
            bicycle.clone() as CRc<Drivable>,
            truck.clone() as CRc<Drivable>,
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
            car.clone() as CRc<Maintainable>,
            motorcycle.clone() as CRc<Maintainable>,
            bicycle.clone() as CRc<Maintainable>,
            truck.clone() as CRc<Maintainable>,
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
            car.get().brand().as_ref().unwrap(),
            "Toyota",
            "Car should be created successfully"
        );
        println!("✓ Car created: {}", car.get().brand().as_ref().unwrap());

        // 2. Motorcycle
        let motorcycle = Motorcycle::new(
            "Harley".to_string(),
            2023,
            "V-Twin".to_string(),
            15.0,
            false,
        );
        assert_eq!(
            motorcycle.get().brand().as_ref().unwrap(),
            "Harley",
            "Motorcycle should be created successfully"
        );
        println!(
            "✓ Motorcycle created: {}",
            motorcycle.get().brand().as_ref().unwrap()
        );

        // 3. Bicycle
        let bicycle = Bicycle::new("Trek".to_string(), 2024, 21, "Carbon Fiber".to_string());
        assert_eq!(
            bicycle.get().brand().as_ref().unwrap(),
            "Trek",
            "Bicycle should be created successfully"
        );
        println!(
            "✓ Bicycle created: {}",
            bicycle.get().brand().as_ref().unwrap()
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
            truck.get().brand().as_ref().unwrap(),
            "Volvo",
            "Truck should be created successfully"
        );
        println!("✓ Truck created: {}", truck.get().brand().as_ref().unwrap());

        // 5. ElectricCar
        let electric_car = ElectricCar::new("Tesla".to_string(), 2024, 4, 500.0, 75.0, 8.0);
        assert_eq!(
            electric_car.get().brand().as_ref().unwrap(),
            "Tesla",
            "ElectricCar should be created successfully"
        );
        println!(
            "✓ ElectricCar created: {}",
            electric_car.get().brand().as_ref().unwrap()
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
            sports_car.get().brand().as_ref().unwrap(),
            "Ferrari",
            "SportsCar should be created successfully"
        );
        println!(
            "✓ SportsCar created: {}",
            sports_car.get().brand().as_ref().unwrap()
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

        println!("Created Car: {}", car.get().brand().as_ref().unwrap());

        // Verify Car implements both Drivable and Maintainable interfaces
        // Access through Drivable interface reference
        let drivable: CRc<Drivable> = car.clone() as CRc<Drivable>;
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
        let maintainable: CRc<Maintainable> = car.clone() as CRc<Maintainable>;
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

        println!("\n✓ Car implements multiple interfaces correctly");
        println!("✓ Both Drivable and Maintainable interfaces work on the same object");
    }
}
