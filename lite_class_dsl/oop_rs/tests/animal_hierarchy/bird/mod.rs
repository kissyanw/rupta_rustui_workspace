// Bird module
//
// Contains Bird base class and all bird derived classes

use classes::*;

use super::animal::Animal;
use super::mixins::Feathered;

classes! {
    /// Bird class
    ///
    /// Bird base implementation, inherits from Animal and mixes in Feathered
    /// All concrete bird classes should inherit from this class
    pub class Bird extends Animal with Feathered {
        struct {
            // Wingspan (meters) - Copy type used directly
            pub wingspan: f64 = 0.0,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Bird's name
        /// * `age` - Bird's age
        /// * `wingspan` - Wingspan (meters)
        /// * `feather_color` - Feather color
        pub fn new(name: String, age: i32, wingspan: f64, feather_color: String) -> Self {
            let bird: CRc<Self> = Self {
                super: Super::new(name, age),
                wingspan,
                ..
            };
            bird.set_feather_color(Some(feather_color));
            bird
        }

        /// Override make_sound method
        pub override fn Animal::make_sound(&self) -> String {
            format!("{} chirps", self.get_name().as_ref().unwrap())
        }

        /// Override move_action method
        pub override fn Animal::move_action(&self) -> String {
            format!("{} hops on the ground", self.get_name().as_ref().unwrap())
        }

        /// Override describe method
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Bird: {}, age {}, wingspan {:.2}m, {} feathers",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_wingspan(),
                self.get_feather_color().as_ref().unwrap()
            )
        }
    }
}

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
        assert_eq!(bird.get_name().as_ref().unwrap(), "Tweety");
        assert_eq!(bird.get_age(), 2);
        assert_eq!(bird.get_wingspan(), 0.3);
        assert_eq!(bird.get_feather_color().as_ref().unwrap(), "Yellow");

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
        assert_eq!(bird.get_feather_color().as_ref().unwrap(), "Blue");

        let preen_after = bird.preen_feathers();
        assert!(preen_after.contains("Blue"));
        println!("After color change: {}", preen_after);
    }

    #[test]
    fn test_mixin_field_modification() {
        // Test mixin field modification methods
        let bird = Bird::new("Robin".to_string(), 2, 0.25, "Red".to_string());

        // Test Feathered mixin field modification
        assert_eq!(bird.get_feather_color().as_ref().unwrap(), "Red");
        bird.change_feather_color("Orange".to_string());
        assert_eq!(bird.get_feather_color().as_ref().unwrap(), "Orange");

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
        assert_eq!(eagle.get_name().as_ref().unwrap(), "Baldy");
        assert_eq!(eagle.get_age(), 8);
        assert_eq!(eagle.get_wingspan(), 2.5);
        assert_eq!(eagle.get_feather_color().as_ref().unwrap(), "Brown");
        assert_eq!(eagle.get_max_altitude(), 1000.0);
        assert_eq!(eagle.get_hunting_territory_size(), 50.0);

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
        assert_eq!(penguin.get_name().as_ref().unwrap(), "Pingu");
        assert_eq!(penguin.get_age(), 5);
        assert_eq!(penguin.get_wingspan(), 0.8);
        assert_eq!(
            penguin.get_feather_color().as_ref().unwrap(),
            "Black and White"
        );
        assert_eq!(penguin.get_swim_speed(), 5.0);
        assert_eq!(penguin.get_colony_size(), 100);

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
        assert_eq!(duck.get_name().as_ref().unwrap(), "Donald");
        assert_eq!(duck.get_age(), 3);
        assert_eq!(duck.get_wingspan(), 0.6);
        assert_eq!(duck.get_feather_color().as_ref().unwrap(), "White");
        assert_eq!(duck.get_max_altitude(), 500.0);
        assert_eq!(duck.get_swim_speed(), 3.0);
        assert_eq!(duck.get_migration_distance(), 2000.0);

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
        assert_eq!(ostrich.get_name().as_ref().unwrap(), "Ozzy");
        assert_eq!(ostrich.get_age(), 6);
        assert_eq!(ostrich.get_wingspan(), 1.5);
        assert_eq!(ostrich.get_feather_color().as_ref().unwrap(), "Black");
        assert_eq!(ostrich.get_running_speed(), 70.0);

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

    // ========== 从 main.rs 移动过来的测试 ==========

    #[test]
    fn test_eagle_multilevel_upcast() {
        use crate::Animal;
        use classes::prelude::*;

        println!("\n=== Testing Eagle multilevel upcast: Eagle -> Bird -> Animal ===");

        // 创建 Eagle 实例
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        // 记录原始行为
        let original_sound = eagle.make_sound();
        let _original_move = eagle.move_action();
        let original_desc = eagle.describe();

        println!("Original Eagle - Sound: {}", original_sound);
        println!("Original Eagle - Desc: {}", original_desc);

        // 第一级：Eagle -> Bird (with Flyable)
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        let bird_sound = bird.make_sound();
        let bird_desc = bird.describe();

        println!("After Eagle->Bird - Sound: {}", bird_sound);
        println!("After Eagle->Bird - Desc: {}", bird_desc);

        assert_eq!(
            original_sound, bird_sound,
            "Sound should remain same after Eagle->Bird"
        );
        assert_eq!(bird.get_name().as_ref().unwrap(), "Sky");
        assert_eq!(bird.get_age(), 7);

        // 第二级：Bird -> Animal (with Feathered)
        let animal = bird.into_superclass::<CRc<Animal>>();
        let animal_sound = animal.make_sound();
        let animal_desc = animal.describe();

        println!("After Bird->Animal - Sound: {}", animal_sound);
        println!("After Bird->Animal - Desc: {}", animal_desc);

        assert_eq!(
            original_sound, animal_sound,
            "Sound should remain same after Bird->Animal"
        );
        assert_eq!(
            original_desc, animal_desc,
            "Desc should remain same through all conversions"
        );
        assert_eq!(animal.get_name().as_ref().unwrap(), "Sky");
        assert_eq!(animal.get_age(), 7);

        println!("✓ Eagle multilevel upcast: behavior preserved through Eagle->Bird->Animal");
    }

    #[test]
    fn test_penguin_multilevel_upcast() {
        use crate::Animal;
        use classes::prelude::*;

        println!("\n=== Testing Penguin multilevel upcast: Penguin -> Bird -> Animal ===");

        let penguin = Penguin::new(
            "Pingu".to_string(),
            4,
            0.8,
            "Black and White".to_string(),
            5.0,
            1000,
        );

        let original_sound = penguin.make_sound();
        let original_desc = penguin.describe();

        println!("Original Penguin - Sound: {}", original_sound);

        // Penguin -> Bird (with Swimmable)
        let bird = penguin.clone().into_superclass::<CRc<Bird>>();
        let bird_sound = bird.make_sound();

        assert_eq!(original_sound, bird_sound);
        assert_eq!(bird.get_name().as_ref().unwrap(), "Pingu");

        // Bird -> Animal (with Feathered)
        let animal = bird.into_superclass::<CRc<Animal>>();
        let animal_sound = animal.make_sound();
        let animal_desc = animal.describe();

        assert_eq!(original_sound, animal_sound);
        assert_eq!(original_desc, animal_desc);
        assert_eq!(animal.get_name().as_ref().unwrap(), "Pingu");
        assert_eq!(animal.get_age(), 4);

        println!("✓ Penguin multilevel upcast: behavior preserved");
    }

    #[test]
    fn test_duck_multilevel_upcast() {
        use crate::Animal;
        use classes::prelude::*;

        println!("\n=== Testing Duck multilevel upcast: Duck -> Bird -> Animal ===");

        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );

        let original_sound = duck.make_sound();
        let original_desc = duck.describe();

        println!("Original Duck - Sound: {}", original_sound);

        // Duck -> Bird (with Flyable + Swimmable)
        let bird = duck.clone().into_superclass::<CRc<Bird>>();
        let bird_sound = bird.make_sound();

        assert_eq!(original_sound, bird_sound);
        assert_eq!(bird.get_name().as_ref().unwrap(), "Donald");

        // Bird -> Animal (with Feathered)
        let animal = bird.into_superclass::<CRc<Animal>>();
        let animal_sound = animal.make_sound();
        let animal_desc = animal.describe();

        assert_eq!(original_sound, animal_sound);
        assert_eq!(original_desc, animal_desc);
        assert_eq!(animal.get_name().as_ref().unwrap(), "Donald");
        assert_eq!(animal.get_age(), 3);

        println!("✓ Duck multilevel upcast: behavior preserved");
    }

    #[test]
    fn test_ostrich_multilevel_upcast() {
        use crate::Animal;
        use classes::prelude::*;

        println!("\n=== Testing Ostrich multilevel upcast: Ostrich -> Bird -> Animal ===");

        let ostrich = Ostrich::new("Ozzy".to_string(), 6, 2.0, "Black".to_string(), 70.0);

        let original_sound = ostrich.make_sound();
        let original_move = ostrich.move_action();
        let original_desc = ostrich.describe();

        println!("Original Ostrich - Move: {}", original_move);

        // Ostrich -> Bird (no additional mixin)
        let bird = ostrich.clone().into_superclass::<CRc<Bird>>();
        let bird_move = bird.move_action();

        assert_eq!(original_move, bird_move);
        assert_eq!(bird.get_name().as_ref().unwrap(), "Ozzy");

        // Bird -> Animal (with Feathered)
        let animal = bird.into_superclass::<CRc<Animal>>();
        let animal_sound = animal.make_sound();
        let animal_move = animal.move_action();
        let animal_desc = animal.describe();

        assert_eq!(original_sound, animal_sound);
        assert_eq!(original_move, animal_move);
        assert_eq!(original_desc, animal_desc);
        assert_eq!(animal.get_name().as_ref().unwrap(), "Ozzy");
        assert_eq!(animal.get_age(), 6);

        println!("✓ Ostrich multilevel upcast: behavior preserved");
    }

    #[test]
    fn test_eagle_to_flyable_mixin() {
        use crate::mixins;
        use classes::prelude::*;

        println!("\n=== Testing Eagle -> Flyable mixin conversion ===");

        // 创建 Eagle 实例
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        // 记录原始字段值
        let original_name = eagle.get_name().as_ref().unwrap().clone();
        let original_max_altitude = eagle.get_max_altitude();

        println!(
            "Original Eagle - Name: {}, Max Altitude: {}",
            original_name, original_max_altitude
        );

        // 转换为 Flyable mixin 引用
        let flyable: CRc<mixins::Flyable> = eagle.to_mixin();

        // 通过 mixin 引用调用方法
        let fly_result = flyable.fly();
        println!("Flyable.fly() result: {}", fly_result);

        // 验证方法返回包含实例信息
        assert!(
            fly_result.contains(&original_name),
            "fly() should contain animal name"
        );
        assert!(
            fly_result.contains(&original_max_altitude.to_string()),
            "fly() should contain max altitude"
        );

        // 验证可以访问 mixin 字段
        assert_eq!(
            flyable.get_max_altitude(),
            original_max_altitude,
            "Max altitude should be accessible through mixin reference"
        );

        println!("✓ Eagle -> Flyable mixin conversion successful");
    }

    #[test]
    fn test_penguin_to_swimmable_mixin() {
        use crate::mixins;
        use classes::prelude::*;

        println!("\n=== Testing Penguin -> Swimmable mixin conversion ===");

        // 创建 Penguin 实例
        let penguin = Penguin::new(
            "Pingu".to_string(),
            4,
            0.8,
            "Black and White".to_string(),
            5.0,
            1000,
        );

        // 记录原始字段值
        let original_name = penguin.get_name().as_ref().unwrap().clone();
        let original_swim_speed = penguin.get_swim_speed();

        println!(
            "Original Penguin - Name: {}, Swim Speed: {}",
            original_name, original_swim_speed
        );

        // 转换为 Swimmable mixin 引用
        let swimmable: CRc<mixins::Swimmable> = penguin.to_mixin();

        // 通过 mixin 引用调用方法
        let swim_result = swimmable.swim();
        println!("Swimmable.swim() result: {}", swim_result);

        // 验证方法返回包含实例信息
        assert!(
            swim_result.contains(&original_name),
            "swim() should contain animal name"
        );
        assert!(
            swim_result.contains(&original_swim_speed.to_string()),
            "swim() should contain swim speed"
        );

        // 验证可以访问 mixin 字段
        assert_eq!(
            swimmable.get_swim_speed(),
            original_swim_speed,
            "Swim speed should be accessible through mixin reference"
        );

        println!("✓ Penguin -> Swimmable mixin conversion successful");
    }

    #[test]
    fn test_duck_to_multiple_mixins() {
        use crate::mixins;
        use classes::prelude::*;

        println!("\n=== Testing Duck -> Flyable and Swimmable mixin conversions ===");

        // 创建 Duck 实例（同时使用 Flyable 和 Swimmable）
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );

        // 记录原始字段值
        let original_name = duck.get_name().as_ref().unwrap().clone();
        let original_max_altitude = duck.get_max_altitude();
        let original_swim_speed = duck.get_swim_speed();

        println!(
            "Original Duck - Name: {}, Max Altitude: {}, Swim Speed: {}",
            original_name, original_max_altitude, original_swim_speed
        );

        // 转换为 Flyable mixin 引用
        let flyable: CRc<mixins::Flyable> = duck.clone().cast_mixin();
        let fly_result = flyable.fly();
        println!("Flyable.fly() result: {}", fly_result);

        // 验证 Flyable 方法和字段
        assert!(
            fly_result.contains(&original_name),
            "fly() should contain duck name"
        );
        assert_eq!(
            flyable.get_max_altitude(),
            original_max_altitude,
            "Max altitude should be accessible through Flyable mixin"
        );

        // 转换为 Swimmable mixin 引用
        let swimmable: CRc<mixins::Swimmable> = duck.clone().cast_mixin();
        let swim_result = swimmable.swim();
        println!("Swimmable.swim() result: {}", swim_result);

        // 验证 Swimmable 方法和字段
        assert!(
            swim_result.contains(&original_name),
            "swim() should contain duck name"
        );
        assert_eq!(
            swimmable.get_swim_speed(),
            original_swim_speed,
            "Swim speed should be accessible through Swimmable mixin"
        );

        // 验证可以独立转换为每个 mixin
        println!("✓ Duck can be independently converted to both Flyable and Swimmable mixins");

        // 验证通过不同 mixin 引用访问的 mixin 字段一致
        // 通过两个不同的 mixin 引用访问相同的 mixin 字段
        let flyable2: CRc<mixins::Flyable> = duck.clone().cast_mixin();
        assert_eq!(
            flyable.get_max_altitude(),
            flyable2.get_max_altitude(),
            "Max altitude should be consistent across multiple Flyable references"
        );

        let swimmable2: CRc<mixins::Swimmable> = duck.clone().cast_mixin();
        assert_eq!(
            swimmable.get_swim_speed(),
            swimmable2.get_swim_speed(),
            "Swim speed should be consistent across multiple Swimmable references"
        );

        println!(
            "✓ Duck -> multiple mixins: fields are consistent across different mixin references"
        );
    }

    #[test]
    fn test_eagle_full_conversion_chain() {
        use crate::Animal;
        use classes::prelude::*;

        println!(
            "\n=== Testing Eagle full conversion chain: Eagle -> Bird -> Animal -> Bird -> Eagle ==="
        );

        // 创建 Eagle 实例
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        // 记录原始字段值
        let original_name = eagle.get_name().as_ref().unwrap().clone();
        let original_age = eagle.get_age();
        let original_wingspan = eagle.get_wingspan();
        let original_feather_color = eagle.get_feather_color().as_ref().unwrap().clone();
        let original_max_altitude = eagle.get_max_altitude();
        let original_hunting_territory = eagle.get_hunting_territory_size();
        let original_sound = eagle.make_sound();
        let original_move = eagle.move_action();
        let original_desc = eagle.describe();

        println!("Original Eagle:");
        println!("  Name: {}, Age: {}", original_name, original_age);
        println!(
            "  Wingspan: {}, Feather Color: {}",
            original_wingspan, original_feather_color
        );
        println!(
            "  Max Altitude: {}, Hunting Territory: {}",
            original_max_altitude, original_hunting_territory
        );

        // 第一步：Eagle -> Bird
        println!("\n--- Step 1: Eagle -> Bird ---");
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();

        // 验证转换成功
        assert_eq!(
            bird.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after Eagle->Bird"
        );
        assert_eq!(
            bird.get_age(),
            original_age,
            "Age should be preserved after Eagle->Bird"
        );
        assert_eq!(
            bird.get_wingspan(),
            original_wingspan,
            "Wingspan should be preserved after Eagle->Bird"
        );
        assert_eq!(
            bird.make_sound(),
            original_sound,
            "Sound should be preserved after Eagle->Bird"
        );
        println!("✓ Eagle -> Bird conversion successful, fields preserved");

        // 第二步：Bird -> Animal
        println!("\n--- Step 2: Bird -> Animal ---");
        let animal = bird.into_superclass::<CRc<Animal>>();

        // 验证转换成功
        assert_eq!(
            animal.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after Bird->Animal"
        );
        assert_eq!(
            animal.get_age(),
            original_age,
            "Age should be preserved after Bird->Animal"
        );
        assert_eq!(
            animal.make_sound(),
            original_sound,
            "Sound should be preserved after Bird->Animal"
        );
        assert_eq!(
            animal.describe(),
            original_desc,
            "Description should be preserved after Bird->Animal"
        );
        println!("✓ Bird -> Animal conversion successful, fields preserved");

        // 第三步：Animal -> Bird (向下转换)
        println!("\n--- Step 3: Animal -> Bird (downcast) ---");
        let bird_again = animal.try_into_subtype::<CRc<Bird>>();
        assert!(
            bird_again.is_some(),
            "Downcast from Animal to Bird should succeed"
        );

        let bird_ref = bird_again.unwrap();
        assert_eq!(
            bird_ref.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after Animal->Bird downcast"
        );
        assert_eq!(
            bird_ref.get_age(),
            original_age,
            "Age should be preserved after Animal->Bird downcast"
        );
        assert_eq!(
            bird_ref.get_wingspan(),
            original_wingspan,
            "Wingspan should be preserved after Animal->Bird downcast"
        );
        assert_eq!(
            bird_ref.make_sound(),
            original_sound,
            "Sound should be preserved after Animal->Bird downcast"
        );
        println!("✓ Animal -> Bird downcast successful, fields preserved");

        // 第四步：Bird -> Eagle (向下转换)
        println!("\n--- Step 4: Bird -> Eagle (downcast) ---");
        let eagle_again = bird_ref.try_into_subtype::<CRc<Eagle>>();
        assert!(
            eagle_again.is_some(),
            "Downcast from Bird to Eagle should succeed"
        );

        let eagle_final = eagle_again.unwrap();

        // 验证最终恢复到原始类型，所有字段值保持不变
        assert_eq!(
            eagle_final.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.get_age(),
            original_age,
            "Age should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.get_wingspan(),
            original_wingspan,
            "Wingspan should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.get_feather_color().as_ref().unwrap(),
            &original_feather_color,
            "Feather color should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.get_max_altitude(),
            original_max_altitude,
            "Max altitude should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.get_hunting_territory_size(),
            original_hunting_territory,
            "Hunting territory should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.make_sound(),
            original_sound,
            "Sound should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.move_action(),
            original_move,
            "Move action should be preserved in final Eagle"
        );
        assert_eq!(
            eagle_final.describe(),
            original_desc,
            "Description should be preserved in final Eagle"
        );

        println!("✓ Bird -> Eagle downcast successful, all fields preserved");
        println!(
            "\n✓ Complete conversion chain successful: Eagle -> Bird -> Animal -> Bird -> Eagle"
        );
        println!("  All fields and behaviors preserved through the entire chain");
    }
}
