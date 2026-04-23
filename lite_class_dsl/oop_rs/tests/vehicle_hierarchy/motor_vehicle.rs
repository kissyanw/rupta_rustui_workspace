use super::vehicle::Vehicle;
use classes::*;

classes! {
    pub abstract class MotorVehicle extends Vehicle {
        struct {
            pub final engine_type: Option<String> = None,
            pub fuel_capacity: f64,
        }

        pub fn new(brand: String, year: i32, engine_type: String, fuel_capacity: f64) -> Self {
            Self {
                super: Super::new(brand, year),
                engine_type: Some(engine_type),
                fuel_capacity,
            }
        }

        pub fn start_engine(&self) -> String;

        pub fn fuel_efficiency(&self) -> f64;

        pub override fn Vehicle::describe(&self) -> String {
            format!(
                "{} {} (Year: {}, Engine: {}, Fuel Capacity: {:.1}L)",
                self.get_brand().as_ref().unwrap(),
                self.get_type(),
                self.get_year(),
                self.get_engine_type().as_ref().unwrap(),
                self.get_fuel_capacity()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MotorVehicle;
    use super::Vehicle;
    use classes::*;

    // 创建一个简单的测试用具体类来测试 MotorVehicle 的功能
    classes! {
        class TestMotorVehicle extends MotorVehicle {
            struct {}

            pub fn new(brand: String, year: i32, engine_type: String, fuel_capacity: f64) -> Self {
                Self {
                    super: Super::new(brand, year, engine_type, fuel_capacity),
                }
            }

            pub override fn Vehicle::get_type(&self) -> String {
                "TestMotorVehicle".to_string()
            }

            pub override fn Vehicle::max_speed(&self) -> f64 {
                150.0
            }

            pub override fn MotorVehicle::start_engine(&self) -> String {
                format!("Starting {} engine", self.get_engine_type().as_ref().unwrap())
            }

            pub override fn MotorVehicle::fuel_efficiency(&self) -> f64 {
                12.5
            }
        }
    }

    #[test]
    fn test_motor_vehicle_field_access() {
        println!("\n=== Testing MotorVehicle field access through derived class ===");

        // 创建测试机动车辆实例
        let motor_vehicle =
            TestMotorVehicle::new("Toyota".to_string(), 2024, "Gasoline".to_string(), 50.0);

        // Test inherited field access from Vehicle
        assert_eq!(
            motor_vehicle.get_brand().as_ref().unwrap(),
            "Toyota",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            motor_vehicle.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        // 测试 MotorVehicle 自己的字段访问
        assert_eq!(
            motor_vehicle.get_engine_type().as_ref().unwrap(),
            "Gasoline",
            "Engine type should be accessible and match the initialized value"
        );
        assert_eq!(
            motor_vehicle.get_fuel_capacity(),
            50.0,
            "Fuel capacity should be accessible and match the initialized value"
        );

        println!("✓ MotorVehicle fields are accessible through derived class");
    }

    #[test]
    fn test_motor_vehicle_describe_includes_engine_info() {
        println!("\n=== Testing MotorVehicle describe() method includes engine information ===");

        // 创建测试机动车辆实例
        let motor_vehicle =
            TestMotorVehicle::new("Honda".to_string(), 2023, "Hybrid".to_string(), 45.0);

        // 测试 describe() 方法
        let description = motor_vehicle.describe();
        println!("Description: {}", description);

        // 验证描述包含品牌、类型、年份、引擎类型和燃料容量信息
        assert!(
            description.contains("Honda"),
            "Description should contain the brand"
        );
        assert!(
            description.contains("TestMotorVehicle"),
            "Description should contain the vehicle type"
        );
        assert!(
            description.contains("2023"),
            "Description should contain the year"
        );
        assert!(
            description.contains("Hybrid"),
            "Description should contain the engine type"
        );
        assert!(
            description.contains("45.0"),
            "Description should contain the fuel capacity"
        );

        // 验证完整格式
        assert_eq!(
            description,
            "Honda TestMotorVehicle (Year: 2023, Engine: Hybrid, Fuel Capacity: 45.0L)",
            "Description should match the expected format with engine information"
        );

        println!("✓ MotorVehicle describe() method includes engine information");
    }

    #[test]
    fn test_motor_vehicle_abstract_methods_implemented() {
        println!("\n=== Testing MotorVehicle abstract methods are implemented ===");

        // 创建测试机动车辆实例
        let motor_vehicle =
            TestMotorVehicle::new("Ford".to_string(), 2022, "Diesel".to_string(), 60.0);

        // 测试 start_engine() 抽象方法实现
        let start_message = motor_vehicle.start_engine();
        assert!(
            start_message.contains("Diesel"),
            "start_engine() should return a message containing the engine type"
        );
        assert_eq!(
            start_message, "Starting Diesel engine",
            "start_engine() should return the expected message"
        );

        // 测试 fuel_efficiency() 抽象方法实现
        let efficiency = motor_vehicle.fuel_efficiency();
        assert_eq!(
            efficiency, 12.5,
            "fuel_efficiency() should return the expected efficiency value"
        );

        println!("✓ MotorVehicle abstract methods are properly implemented in derived class");
    }

    #[test]
    fn test_motor_vehicle_inherits_vehicle_methods() {
        println!("\n=== Testing MotorVehicle inherits Vehicle methods ===");

        // 创建测试机动车辆实例
        let motor_vehicle =
            TestMotorVehicle::new("BMW".to_string(), 2025, "Electric".to_string(), 0.0);

        // 测试继承自 Vehicle 的抽象方法
        let vehicle_type = motor_vehicle.get_type();
        assert_eq!(
            vehicle_type, "TestMotorVehicle",
            "get_type() should return the vehicle type"
        );

        let max_speed = motor_vehicle.max_speed();
        assert_eq!(
            max_speed, 150.0,
            "max_speed() should return the maximum speed"
        );

        println!("✓ MotorVehicle properly inherits and implements Vehicle methods");
    }

    #[test]
    fn test_motor_vehicle_upcast_to_vehicle() {
        println!("\n=== Testing MotorVehicle upcast to Vehicle ===");

        // 创建测试机动车辆实例
        let test_motor_vehicle =
            TestMotorVehicle::new("Mercedes".to_string(), 2024, "V8".to_string(), 70.0);

        // Record original values
        let original_brand = test_motor_vehicle.get_brand().as_ref().unwrap().clone();
        let original_year = test_motor_vehicle.get_year();
        let original_type = test_motor_vehicle.get_type();
        let original_speed = test_motor_vehicle.max_speed();

        // Upcast to Vehicle
        let vehicle: CRc<Vehicle> = test_motor_vehicle.clone().into_super().into();

        // Verify fields and methods are still accessible after upcast
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
        assert_eq!(
            vehicle.get_type(),
            original_type,
            "Type should be preserved after upcast to Vehicle"
        );
        assert_eq!(
            vehicle.max_speed(),
            original_speed,
            "Max speed should be preserved after upcast to Vehicle"
        );

        println!("✓ MotorVehicle upcast to Vehicle preserves all fields and methods");
    }

    #[test]
    fn test_motor_vehicle_describe_overrides_vehicle_describe() {
        println!("\n=== Testing MotorVehicle describe() overrides Vehicle describe() ===");

        // 创建测试机动车辆实例
        let motor_vehicle =
            TestMotorVehicle::new("Audi".to_string(), 2023, "Turbocharged".to_string(), 55.0);

        // 获取 MotorVehicle 的描述
        let motor_description = motor_vehicle.describe();

        // Upcast to Vehicle
        let vehicle: CRc<Vehicle> = motor_vehicle.clone().into_super().into();
        let vehicle_description = vehicle.describe();

        // 验证描述相同（因为 MotorVehicle 重写了 describe()）
        assert_eq!(
            motor_description, vehicle_description,
            "MotorVehicle's describe() should be used even after upcast to Vehicle"
        );

        // 验证描述包含引擎信息（这是 MotorVehicle 特有的）
        assert!(
            vehicle_description.contains("Turbocharged"),
            "Vehicle reference should use MotorVehicle's overridden describe() method"
        );

        println!("✓ MotorVehicle describe() properly overrides Vehicle describe()");
    }
}
