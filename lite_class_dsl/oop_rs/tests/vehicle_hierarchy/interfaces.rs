use classes::*;

classes! {
    pub abstract class Drivable {
        pub fn drive(&self) -> String;
        pub fn stop(&self) -> String;
        pub fn turn(&self, direction: String) -> String;
    }

    pub abstract class Maintainable {
        pub fn perform_maintenance(&self) -> String;
        pub fn check_condition(&self) -> String;
    }

    pub abstract class Chargeable {
        pub fn charge(&self) -> String;
        pub fn battery_level(&self) -> f64;
        pub fn charging_status(&self) -> String;
    }
}
