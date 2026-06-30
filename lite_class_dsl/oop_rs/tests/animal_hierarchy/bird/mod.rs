// Bird module
//
// Contains Bird base class and all bird derived classes

use oop_rs::prelude::*;

use super::animal::{Animal, IAnimal};
use super::mixins::{Feathered, IFeathered};

/// Bird class
///
/// Bird base implementation, inherits from Animal and mixes in Feathered
/// All concrete bird classes should inherit from this class
#[class(extends(Animal), with(Feathered))]
pub type Bird = class<
    {
        // Wingspan (meters) - Copy type, mutable, public
        #[vis(pub)]
        let mut wingspan: f64;

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Bird's name
        /// * `age` - Bird's age
        /// * `wingspan` - Wingspan (meters)
        /// * `feather_color` - Feather color
        pub fn new(name: String, age: i32, wingspan: f64, feather_color: String) -> Self {
            let self = Self {
                wingspan,
                ..Super::new(name, age)
            };
            self.set().feather_color(Some(feather_color));
            self
        }

        /// Override make_sound method
        #[method(override(Animal))]
        pub fn make_sound(&self) -> String {
            format!("{} chirps", self.get().name().as_ref().unwrap())
        }

        /// Override move_action method
        #[method(override(Animal))]
        pub fn move_action(&self) -> String {
            format!("{} hops on the ground", self.get().name().as_ref().unwrap())
        }

        /// Override describe method
        #[method(override(Animal))]
        pub fn describe(&self) -> String {
            format!(
                "Bird: {}, age {}, wingspan {:.2}m, {} feathers",
                self.get().name().as_ref().unwrap(),
                self.get().age(),
                self.get().wingspan(),
                self.get().feather_color().as_ref().unwrap()
            )
        }
    },
>;

// Declare submodules
mod duck;
mod eagle;
mod ostrich;
mod penguin;

// Re-export types
pub use duck::Duck;
pub use eagle::Eagle;
pub use ostrich::Ostrich;
pub use penguin::Penguin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bird() {
        // Test creating Bird instance
        let bird = Bird::new("Tweety".to_string(), 2, 0.3, "Yellow".to_string());

        // Verify field initialization
        assert_eq!(bird.get().name().as_ref().unwrap(), "Tweety");
        assert_eq!(bird.get().age(), 2);
        assert_eq!(bird.get().wingspan(), 0.3);
        assert_eq!(bird.get().feather_color().as_ref().unwrap(), "Yellow");

        println!("Bird created successfully: {}", bird.describe());
    }

    #[test]
    fn test_bird_methods() {
        // Test Bird methods
        let bird = Bird::new("Robin".to_string(), 1, 0.25, "Red".to_string());

        let sound = bird.make_sound();
        let movement = bird.move_action();
        let description = bird.describe();

        // Verify methods return non-empty strings
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // Verify returned content contains expected information
        assert!(sound.contains("Robin"));
        assert!(sound.contains("chirps"));
        assert!(movement.contains("Robin"));
        assert!(movement.contains("hops"));
        assert!(description.contains("Robin"));
        assert!(description.contains("0.25"));
        assert!(description.contains("Red"));

        println!("Bird sound: {}", sound);
        println!("Bird movement: {}", movement);
        println!("Bird description: {}", description);
    }

    #[test]
    fn test_bird_feathered_mixin() {
        // Test Bird's Feathered mixin methods
        let bird = Bird::new("Sparrow".to_string(), 1, 0.2, "Brown".to_string());

        let preen = bird.preen_feathers();

        // Verify mixin method returns non-empty string
        assert!(!preen.is_empty());

        // Verify returned content contains expected information
        assert!(preen.contains("Sparrow"));
        assert!(preen.contains("preening"));
        assert!(preen.contains("Brown"));

        println!("Bird preen feathers: {}", preen);

        // Test changing feather color
        bird.change_feather_color("Blue".to_string());
        assert_eq!(bird.get().feather_color().as_ref().unwrap(), "Blue");

        let preen_after = bird.preen_feathers();
        assert!(preen_after.contains("Blue"));
        println!("After color change: {}", preen_after);
    }

    #[test]
    fn test_mixin_field_modification() {
        // Test mixin field modification methods
        let bird = Bird::new("Robin".to_string(), 2, 0.25, "Red".to_string());

        // Test Feathered mixin field modification
        assert_eq!(bird.get().feather_color().as_ref().unwrap(), "Red");
        bird.change_feather_color("Orange".to_string());
        assert_eq!(bird.get().feather_color().as_ref().unwrap(), "Orange");

        println!("Feather color changed from Red to Orange");
    }

    #[test]
    fn test_create_eagle() {
        // Test creating Eagle instance
        let eagle = Eagle::new(
            "Baldy".to_string(),
            8,
            2.5,
            "Brown".to_string(),
            1000.0,
            50.0,
        );

        // Verify field initialization
        assert_eq!(eagle.get().name().as_ref().unwrap(), "Baldy");
        assert_eq!(eagle.get().age(), 8);
        assert_eq!(eagle.get().wingspan(), 2.5);
        assert_eq!(eagle.get().feather_color().as_ref().unwrap(), "Brown");
        assert_eq!(eagle.get().max_altitude(), 1000.0);
        assert_eq!(eagle.get().hunting_territory_size(), 50.0);

        println!("Eagle created successfully: {}", eagle.describe());
    }

    #[test]
    fn test_eagle_methods() {
        // Test Eagle methods
        let eagle = Eagle::new(
            "Sky".to_string(),
            7,
            2.3,
            "Dark Brown".to_string(),
            1200.0,
            60.0,
        );

        let sound = eagle.make_sound();
        let movement = eagle.move_action();
        let description = eagle.describe();

        // Verify methods return non-empty strings
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // Verify returned content contains expected information
        assert!(sound.contains("Sky"));
        assert!(movement.contains("Sky"));
        assert!(description.contains("Sky"));
        assert!(description.contains("Eagle"));
        assert!(description.contains("60"));

        println!("Eagle sound: {}", sound);
        println!("Eagle movement: {}", movement);
        println!("Eagle description: {}", description);
    }

    #[test]
    fn test_eagle_flyable_mixin() {
        // Test Eagle's Flyable mixin methods
        let eagle = Eagle::new(
            "Soar".to_string(),
            9,
            2.4,
            "Brown".to_string(),
            1500.0,
            70.0,
        );

        let fly_result = eagle.fly();

        // Verify mixin method returns non-empty string
        assert!(!fly_result.is_empty());

        // Verify returned content contains expected information
        assert!(fly_result.contains("Soar"));
        assert!(fly_result.contains("flying"));
        assert!(fly_result.contains("1500"));

        println!("Eagle fly: {}", fly_result);
    }

    #[test]
    fn test_create_penguin() {
        // 测试创建 Penguin 实例
        let penguin = Penguin::new(
            "Pingu".to_string(),
            5,
            0.8,
            "Black and White".to_string(),
            5.0,
            100,
        );

        // 验证字段初始化
        assert_eq!(penguin.get().name().as_ref().unwrap(), "Pingu");
        assert_eq!(penguin.get().age(), 5);
        assert_eq!(penguin.get().wingspan(), 0.8);
        assert_eq!(
            penguin.get().feather_color().as_ref().unwrap(),
            "Black and White"
        );
        assert_eq!(penguin.get().swim_speed(), 5.0);
        assert_eq!(penguin.get().colony_size(), 100);

        println!("Penguin created successfully: {}", penguin.describe());
    }

    #[test]
    fn test_penguin_methods() {
        // 测试 Penguin 的方法
        let penguin = Penguin::new(
            "Waddles".to_string(),
            4,
            0.7,
            "Black and White".to_string(),
            6.0,
            150,
        );

        let sound = penguin.make_sound();
        let movement = penguin.move_action();
        let description = penguin.describe();

        // 验证方法返回非空字符串
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // 验证返回内容包含预期信息
        assert!(sound.contains("Waddles"));
        assert!(movement.contains("Waddles"));
        assert!(description.contains("Waddles"));
        assert!(description.contains("Penguin"));
        assert!(description.contains("150"));

        println!("Penguin sound: {}", sound);
        println!("Penguin movement: {}", movement);
        println!("Penguin description: {}", description);
    }

    #[test]
    fn test_penguin_swimmable_mixin() {
        // 测试 Penguin 的 Swimmable mixin 方法
        let penguin = Penguin::new(
            "Swimmer".to_string(),
            6,
            0.9,
            "Black and White".to_string(),
            7.0,
            200,
        );

        let swim_result = penguin.swim();

        // 验证 mixin 方法返回非空字符串
        assert!(!swim_result.is_empty());

        // 验证返回内容包含预期信息
        assert!(swim_result.contains("Swimmer"));
        assert!(swim_result.contains("swimming"));
        assert!(swim_result.contains("7"));

        println!("Penguin swim: {}", swim_result);
    }

    #[test]
    fn test_create_duck() {
        // 测试创建 Duck 实例
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            0.6,
            "White".to_string(),
            500.0,
            3.0,
            2000.0,
        );

        // 验证字段初始化
        assert_eq!(duck.get().name().as_ref().unwrap(), "Donald");
        assert_eq!(duck.get().age(), 3);
        assert_eq!(duck.get().wingspan(), 0.6);
        assert_eq!(duck.get().feather_color().as_ref().unwrap(), "White");
        assert_eq!(duck.get().max_altitude(), 500.0);
        assert_eq!(duck.get().swim_speed(), 3.0);
        assert_eq!(duck.get().migration_distance(), 2000.0);

        println!("Duck created successfully: {}", duck.describe());
    }

    #[test]
    fn test_duck_methods() {
        // 测试 Duck 的方法
        let duck = Duck::new(
            "Quackers".to_string(),
            2,
            0.5,
            "Brown".to_string(),
            400.0,
            4.0,
            1500.0,
        );

        let sound = duck.make_sound();
        let movement = duck.move_action();
        let description = duck.describe();

        // 验证方法返回非空字符串
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // 验证返回内容包含预期信息
        assert!(sound.contains("Quackers"));
        assert!(movement.contains("Quackers"));
        assert!(description.contains("Quackers"));
        assert!(description.contains("Duck"));
        assert!(description.contains("1500"));

        println!("Duck sound: {}", sound);
        println!("Duck movement: {}", movement);
        println!("Duck description: {}", description);
    }

    #[test]
    fn test_duck_flyable_mixin() {
        // 测试 Duck 的 Flyable mixin 方法
        let duck = Duck::new(
            "Flyer".to_string(),
            4,
            0.7,
            "Green".to_string(),
            600.0,
            5.0,
            3000.0,
        );

        let fly_result = duck.fly();

        // 验证 mixin 方法返回非空字符串
        assert!(!fly_result.is_empty());

        // 验证返回内容包含预期信息
        assert!(fly_result.contains("Flyer"));
        assert!(fly_result.contains("flying"));
        assert!(fly_result.contains("600"));

        println!("Duck fly: {}", fly_result);
    }

    #[test]
    fn test_duck_swimmable_mixin() {
        // 测试 Duck 的 Swimmable mixin 方法
        let duck = Duck::new(
            "Paddler".to_string(),
            3,
            0.6,
            "Brown".to_string(),
            500.0,
            4.5,
            2500.0,
        );

        let swim_result = duck.swim();

        // 验证 mixin 方法返回非空字符串
        assert!(!swim_result.is_empty());

        // 验证返回内容包含预期信息
        assert!(swim_result.contains("Paddler"));
        assert!(swim_result.contains("swimming"));
        assert!(swim_result.contains("4.5"));

        println!("Duck swim: {}", swim_result);
    }

    #[test]
    fn test_duck_both_mixins() {
        // 测试 Duck 同时使用 Flyable 和 Swimmable mixin
        let duck = Duck::new(
            "Versatile".to_string(),
            2,
            0.55,
            "White".to_string(),
            450.0,
            3.5,
            1800.0,
        );

        let fly_result = duck.fly();
        let swim_result = duck.swim();

        // 验证两个 mixin 方法都可以调用
        assert!(!fly_result.is_empty());
        assert!(!swim_result.is_empty());

        assert!(fly_result.contains("Versatile"));
        assert!(swim_result.contains("Versatile"));

        println!(
            "Duck can both fly: {} and swim: {}",
            fly_result, swim_result
        );
    }

    #[test]
    fn test_create_ostrich() {
        // 测试创建 Ostrich 实例
        let ostrich = Ostrich::new("Ozzy".to_string(), 6, 1.5, "Black".to_string(), 70.0);

        // 验证字段初始化
        assert_eq!(ostrich.get().name().as_ref().unwrap(), "Ozzy");
        assert_eq!(ostrich.get().age(), 6);
        assert_eq!(ostrich.get().wingspan(), 1.5);
        assert_eq!(ostrich.get().feather_color().as_ref().unwrap(), "Black");
        assert_eq!(ostrich.get().running_speed(), 70.0);

        println!("Ostrich created successfully: {}", ostrich.describe());
    }

    #[test]
    fn test_ostrich_methods() {
        // 测试 Ostrich 的方法
        let ostrich = Ostrich::new(
            "Speedy".to_string(),
            5,
            1.8,
            "Black and White".to_string(),
            65.0,
        );

        let sound = ostrich.make_sound();
        let movement = ostrich.move_action();
        let description = ostrich.describe();

        // 验证方法返回非空字符串
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // 验证返回内容包含预期信息
        assert!(sound.contains("Speedy"));
        assert!(movement.contains("Speedy"));
        assert!(description.contains("Speedy"));
        assert!(description.contains("Ostrich"));
        assert!(description.contains("65"));

        println!("Ostrich sound: {}", sound);
        println!("Ostrich movement: {}", movement);
        println!("Ostrich description: {}", description);
    }

    // ========== Tests with upcast/downcast (API may differ in oop_rs) ==========
    // Note: The following tests use classes! API (into_superclass, try_into_subtype, to_mixin, cast_mixin)
    // These may need to be updated for oop_rs API once we understand the equivalent methods

    // TODO: Update these tests to use oop_rs API for upcasting/downcasting
    // #[test]
    // fn test_eagle_multilevel_upcast() { ... }
    // ... (other upcast/downcast tests commented out for now)
}
