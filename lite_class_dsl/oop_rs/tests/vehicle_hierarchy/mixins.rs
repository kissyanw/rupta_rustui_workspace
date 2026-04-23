use super::car::Car;
use super::vehicle::Vehicle;
use classes::*;

classes! {
    #[with(Vehicle, Car)]
    pub mixin Autonomous on Vehicle {
        struct {
            pub autonomy_level: i32 = 0,
            pub sensor_count: i32 = 0,
        }

        pub fn enable_autopilot(&self) -> String {
            format!(
                "Enabling Level {} autopilot with {} sensors",
                self.get_autonomy_level(),
                self.get_sensor_count()
            )
        }

        pub fn disable_autopilot(&self) -> String {
            "Disabling autopilot".to_string()
        }
    }
}

#[cfg(test)]
mod test_vehicle {
    use super::{Autonomous, Vehicle};
    use classes::*;

    classes! {
        pub class TestAutonomousVehicle extends Vehicle with Autonomous {
            struct {}

            pub fn new(brand: String, year: i32, autonomy_level: i32, sensor_count: i32) -> Self {
                let vehicle: CRc<Self> = Self {
                    super: Super::new(brand, year),
                    ..
                };
                vehicle.set_autonomy_level(autonomy_level);
                vehicle.set_sensor_count(sensor_count);
                vehicle
            }

            pub override fn Vehicle::get_type(&self) -> String {
                "TestAutonomousVehicle".to_string()
            }

            pub override fn Vehicle::max_speed(&self) -> f64 {
                150.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_vehicle::TestAutonomousVehicle;

    #[test]
    fn test_autonomous_mixin_fields() {
        println!("\n=== Testing Autonomous mixin field access ===");

        // Create test vehicle with Autonomous mixin
        let vehicle = TestAutonomousVehicle::new(
            "Tesla".to_string(),
            2024,
            5,  // Level 5 autonomy
            12, // 12 sensors
        );

        // 测试 mixin 字段访问
        assert_eq!(
            vehicle.get_autonomy_level(),
            5,
            "Autonomy level should be accessible and match the initialized value"
        );
        assert_eq!(
            vehicle.get_sensor_count(),
            12,
            "Sensor count should be accessible and match the initialized value"
        );

        println!("✓ Autonomous mixin fields are accessible");
    }

    #[test]
    fn test_autonomous_enable_autopilot() {
        println!("\n=== Testing Autonomous mixin enable_autopilot() method ===");

        // Create test vehicle with Autonomous mixin
        let vehicle = TestAutonomousVehicle::new(
            "Tesla".to_string(),
            2024,
            4,  // Level 4 autonomy
            10, // 10 sensors
        );

        // Test enable_autopilot() method
        let message = vehicle.enable_autopilot();
        println!("Enable autopilot message: {}", message);

        // Verify message contains autonomy level and sensor count
        assert!(
            message.contains("Level 4"),
            "Message should contain the autonomy level"
        );
        assert!(
            message.contains("10 sensors"),
            "Message should contain the sensor count"
        );
        assert!(
            message.contains("Enabling"),
            "Message should indicate enabling action"
        );

        println!("✓ enable_autopilot() method works correctly");
    }

    #[test]
    fn test_autonomous_disable_autopilot() {
        println!("\n=== Testing Autonomous mixin disable_autopilot() method ===");

        // Create test vehicle with Autonomous mixin
        let vehicle = TestAutonomousVehicle::new(
            "Tesla".to_string(),
            2024,
            3, // Level 3 autonomy
            8, // 8 sensors
        );

        // Test disable_autopilot() method
        let message = vehicle.disable_autopilot();
        println!("Disable autopilot message: {}", message);

        // Verify message content
        assert_eq!(
            message, "Disabling autopilot",
            "Message should indicate disabling action"
        );

        println!("✓ disable_autopilot() method works correctly");
    }

    #[test]
    fn test_autonomous_mixin_with_vehicle_methods() {
        println!("\n=== Testing Autonomous mixin coexists with Vehicle methods ===");

        // Create test vehicle with Autonomous mixin
        let vehicle = TestAutonomousVehicle::new(
            "Mercedes".to_string(),
            2025,
            5,  // Level 5 autonomy
            15, // 15 sensors
        );

        // 测试 Vehicle 基类方法仍然可用
        assert_eq!(
            vehicle.get_brand().as_ref().unwrap(),
            "Mercedes",
            "Vehicle brand should be accessible"
        );
        assert_eq!(
            vehicle.get_year(),
            2025,
            "Vehicle year should be accessible"
        );
        assert_eq!(
            vehicle.get_type(),
            "TestAutonomousVehicle",
            "Vehicle get_type() should work"
        );
        assert_eq!(
            vehicle.max_speed(),
            150.0,
            "Vehicle max_speed() should work"
        );

        // 测试 Autonomous mixin 方法
        assert_eq!(
            vehicle.get_autonomy_level(),
            5,
            "Autonomous autonomy_level should be accessible"
        );
        assert_eq!(
            vehicle.get_sensor_count(),
            15,
            "Autonomous sensor_count should be accessible"
        );

        let enable_msg = vehicle.enable_autopilot();
        assert!(
            enable_msg.contains("Level 5"),
            "Autonomous enable_autopilot() should work"
        );

        let disable_msg = vehicle.disable_autopilot();
        assert_eq!(
            disable_msg, "Disabling autopilot",
            "Autonomous disable_autopilot() should work"
        );

        println!("✓ Autonomous mixin coexists properly with Vehicle methods");
    }

    #[test]
    fn test_autonomous_mixin_different_levels() {
        println!("\n=== Testing Autonomous mixin with different autonomy levels ===");

        // 测试不同的自动驾驶等级
        let levels = vec![1, 2, 3, 4, 5];
        let sensor_counts = vec![4, 6, 8, 10, 12];

        for (level, sensors) in levels.iter().zip(sensor_counts.iter()) {
            let vehicle =
                TestAutonomousVehicle::new(format!("Brand{}", level), 2024, *level, *sensors);

            assert_eq!(
                vehicle.get_autonomy_level(),
                *level,
                "Autonomy level {} should be correctly stored",
                level
            );
            assert_eq!(
                vehicle.get_sensor_count(),
                *sensors,
                "Sensor count {} should be correctly stored",
                sensors
            );

            let message = vehicle.enable_autopilot();
            assert!(
                message.contains(&format!("Level {}", level)),
                "Message should contain Level {}",
                level
            );
            assert!(
                message.contains(&format!("{} sensors", sensors)),
                "Message should contain {} sensors",
                sensors
            );
        }

        println!("✓ Autonomous mixin works correctly with different autonomy levels");
    }
}
