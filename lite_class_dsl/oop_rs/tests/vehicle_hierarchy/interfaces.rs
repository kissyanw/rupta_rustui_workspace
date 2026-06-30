use oop_rs::prelude::*;

#[class]
pub type Drivable = interface<{
    fn drive(&self) -> String;
    fn stop(&self) -> String;
    fn turn(&self, direction: String) -> String;
}>;

#[class]
pub type Maintainable = interface<{
    fn perform_maintenance(&self) -> String;
    fn check_condition(&self) -> String;
}>;

#[class]
pub type Chargeable = interface<{
    fn charge(&self) -> String;
    fn battery_level(&self) -> f64;
    fn charging_status(&self) -> String;
}>;
