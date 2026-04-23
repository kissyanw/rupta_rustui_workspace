use classes::*;

classes! {
    pub abstract class Vehicle {
        struct {
            pub final brand: Option<String> = None,
            pub year: i32,
        }

        pub fn new(brand: String, year: i32) -> Self {
            Self {
                brand: Some(brand),
                year,
            }
        }

        pub fn get_type(&self) -> String;

        pub fn max_speed(&self) -> f64;

        pub fn describe(&self) -> String {
            format!(
                "{} {} (Year: {})",
                self.get_brand().as_ref().unwrap(),
                self.get_type(),
                self.get_year()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vehicle;
    use classes::*;

    // 创建一个简单的测试用具体类来测试 Vehicle 的功能
    classes! {
        class TestVehicle extends Vehicle {
            struct {}

            pub fn new(brand: String, year: i32) -> Self {
                Self {
                    super: Super::new(brand, year),
                }
            }

            pub override fn Vehicle::get_type(&self) -> String {
                "TestVehicle".to_string()
            }

            pub override fn Vehicle::max_speed(&self) -> f64 {
                100.0
            }
        }
    }

    #[test]
    fn test_vehicle_field_access() {
        println!("\n=== Testing Vehicle field access through derived class ===");

        // Create test vehicle instance
        let vehicle = TestVehicle::new("Toyota".to_string(), 2024);

        // 测试字段访问
        assert_eq!(
            vehicle.get_brand().as_ref().unwrap(),
            "Toyota",
            "Brand should be accessible and match the initialized value"
        );
        assert_eq!(
            vehicle.get_year(),
            2024,
            "Year should be accessible and match the initialized value"
        );

        println!("✓ Vehicle fields are accessible through derived class");
    }

    #[test]
    fn test_vehicle_describe_method() {
        println!("\n=== Testing Vehicle describe() method ===");

        // Create test vehicle instance
        let vehicle = TestVehicle::new("Honda".to_string(), 2023);

        // 测试 describe() 方法
        let description = vehicle.describe();
        println!("Description: {}", description);

        // 验证描述包含品牌、类型和年份信息
        assert!(
            description.contains("Honda"),
            "Description should contain the brand"
        );
        assert!(
            description.contains("TestVehicle"),
            "Description should contain the vehicle type"
        );
        assert!(
            description.contains("2023"),
            "Description should contain the year"
        );

        // 验证完整格式
        assert_eq!(
            description, "Honda TestVehicle (Year: 2023)",
            "Description should match the expected format"
        );

        println!("✓ Vehicle describe() method works correctly");
    }

    #[test]
    fn test_vehicle_abstract_methods_implemented() {
        println!("\n=== Testing Vehicle abstract methods are implemented ===");

        // Create test vehicle instance
        let vehicle = TestVehicle::new("Ford".to_string(), 2022);

        // 测试抽象方法实现
        let vehicle_type = vehicle.get_type();
        assert_eq!(
            vehicle_type, "TestVehicle",
            "get_type() should return the vehicle type"
        );

        let max_speed = vehicle.max_speed();
        assert_eq!(
            max_speed, 100.0,
            "max_speed() should return the maximum speed"
        );

        println!("✓ Vehicle abstract methods are properly implemented in derived class");
    }

    #[test]
    fn test_vehicle_upcast() {
        println!("\n=== Testing Vehicle upcast ===");

        // Create test vehicle instance
        let test_vehicle = TestVehicle::new("BMW".to_string(), 2025);

        // Record original values
        let original_brand = test_vehicle.get_brand().as_ref().unwrap().clone();
        let original_year = test_vehicle.get_year();
        let original_type = test_vehicle.get_type();
        let original_speed = test_vehicle.max_speed();

        // Upcast to Vehicle
        let vehicle: CRc<Vehicle> = test_vehicle.into_super();

        // Verify fields and methods are still accessible after upcast
        assert_eq!(
            vehicle.get_brand().as_ref().unwrap(),
            &original_brand,
            "Brand should be preserved after upcast"
        );
        assert_eq!(
            vehicle.get_year(),
            original_year,
            "Year should be preserved after upcast"
        );
        assert_eq!(
            vehicle.get_type(),
            original_type,
            "Type should be preserved after upcast"
        );
        assert_eq!(
            vehicle.max_speed(),
            original_speed,
            "Max speed should be preserved after upcast"
        );

        println!("✓ Vehicle upcast preserves all fields and methods");
    }
}
