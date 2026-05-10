#[path = "../vehicle_hierarchy/bicycle.rs"]
mod bicycle;
#[path = "../vehicle_hierarchy/car.rs"]
mod car;
#[path = "../vehicle_hierarchy/electric_car.rs"]
mod electric_car;
#[path = "../vehicle_hierarchy/interfaces.rs"]
mod interfaces;
#[path = "../vehicle_hierarchy/mixins.rs"]
mod mixins;
#[path = "../vehicle_hierarchy/motor_vehicle.rs"]
mod motor_vehicle;
#[path = "../vehicle_hierarchy/motorcycle.rs"]
mod motorcycle;
#[path = "../vehicle_hierarchy/sports_car.rs"]
mod sports_car;
#[path = "../vehicle_hierarchy/truck.rs"]
mod truck;
#[path = "../vehicle_hierarchy/vehicle.rs"]
mod vehicle;

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

use classes::prelude::*;

fn build_drivables() -> Vec<CRc<Drivable>> {
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
    vec![
        car.clone().into(),
        motorcycle.clone().into(),
        bicycle.clone().into(),
        truck.clone().into(),
    ]
}

#[test]
fn test_rcpta_min_iter_next_unwrap_drive() {
    let drivables = build_drivables();
    let mut iter = drivables.iter();
    let vehicle = iter.next().unwrap();
    let msg = vehicle.drive();
    assert!(!msg.is_empty(), "drive() should return a non-empty message");
}

#[test]
fn test_rcpta_min_index_drive() {
    let drivables = build_drivables();
    let vehicle = &drivables[0];
    let msg = vehicle.drive();
    assert!(!msg.is_empty(), "drive() should return a non-empty message");
}
