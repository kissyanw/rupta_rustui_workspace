// Fish module
//
// Contains Fish base class and all fish derived classes

use classes::*;

use super::animal::Animal;
use super::mixins::Scaled;

classes! {
    /// Fish class
    ///
    /// Fish base implementation, inherits from Animal and mixes in Scaled
    /// All concrete fish classes should inherit from this class
    pub class Fish extends Animal with Scaled {
        struct {
            // Water type (freshwater/saltwater) - uses pub final modifier
            pub final water_type: Option<String> = None,
        }

        /// Constructor
        ///
        /// # Parameters
        /// * `name` - Fish's name
        /// * `age` - Fish's age
        /// * `water_type` - Water type (freshwater/saltwater)
        /// * `scale_pattern` - Scale pattern
        pub fn new(name: String, age: i32, water_type: String, scale_pattern: String) -> Self {
            let fish: CRc<Self> = Self {
                super: Super::new(name, age),
                water_type: Some(water_type),
                ..
            };
            fish.set_scale_pattern(Some(scale_pattern));
            fish
        }

        /// Override make_sound method
        pub override fn Animal::make_sound(&self) -> String {
            format!("{} makes bubbles", self.get_name().as_ref().unwrap())
        }

        /// Override move_action method
        pub override fn Animal::move_action(&self) -> String {
            format!("{} swims in {} water",
                    self.get_name().as_ref().unwrap(),
                    self.get_water_type().as_ref().unwrap())
        }

        /// Override describe method
        pub override fn Animal::describe(&self) -> String {
            format!(
                "Fish: {}, age {}, {} water, {} scales",
                self.get_name().as_ref().unwrap(),
                self.get_age(),
                self.get_water_type().as_ref().unwrap(),
                self.get_scale_pattern().as_ref().unwrap()
            )
        }
    }
}

// Declare submodules
mod flying_fish;
mod salmon;
mod shark;

// Re-export types
pub use flying_fish::FlyingFish;
pub use salmon::Salmon;
pub use shark::Shark;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fish() {
        // 测试创建 Fish 实例
        let fish = Fish::new(
            "Nemo".to_string(),
            2,
            "saltwater".to_string(),
            "orange and white stripes".to_string(),
        );

        // 验证字段初始化
        assert_eq!(fish.get_name().as_ref().unwrap(), "Nemo");
        assert_eq!(fish.get_age(), 2);
        assert_eq!(fish.get_water_type().as_ref().unwrap(), "saltwater");
        assert_eq!(
            fish.get_scale_pattern().as_ref().unwrap(),
            "orange and white stripes"
        );

        println!("Fish created successfully: {}", fish.describe());
    }

    #[test]
    fn test_fish_methods() {
        // 测试 Fish 的方法
        let fish = Fish::new(
            "Goldie".to_string(),
            1,
            "freshwater".to_string(),
            "golden scales".to_string(),
        );

        let sound = fish.make_sound();
        let movement = fish.move_action();
        let description = fish.describe();

        // 验证方法返回非空字符串
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // 验证返回内容包含预期信息
        assert!(sound.contains("Goldie"));
        assert!(sound.contains("bubbles"));
        assert!(movement.contains("Goldie"));
        assert!(movement.contains("freshwater"));
        assert!(description.contains("Goldie"));
        assert!(description.contains("freshwater"));
        assert!(description.contains("golden scales"));

        println!("Fish sound: {}", sound);
        println!("Fish movement: {}", movement);
        println!("Fish description: {}", description);
    }

    #[test]
    fn test_fish_scaled_mixin() {
        // 测试 Fish 的 Scaled mixin 方法
        let fish = Fish::new(
            "Bubbles".to_string(),
            3,
            "saltwater".to_string(),
            "silver scales".to_string(),
        );

        let shed = fish.shed_scales();

        // 验证 mixin 方法返回非空字符串
        assert!(!shed.is_empty());

        // 验证返回内容包含预期信息
        assert!(shed.contains("Bubbles"));
        assert!(shed.contains("shedding"));
        assert!(shed.contains("silver scales"));

        println!("Fish shed scales: {}", shed);

        // 测试改变鳞片图案
        fish.change_scale_pattern("rainbow scales".to_string());
        assert_eq!(fish.get_scale_pattern().as_ref().unwrap(), "rainbow scales");

        let shed_after = fish.shed_scales();
        assert!(shed_after.contains("rainbow scales"));
        println!("After pattern change: {}", shed_after);
    }

    #[test]
    fn test_create_shark() {
        // 测试创建 Shark 实例
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "saltwater".to_string(),
            "gray scales".to_string(),
            300,
            15.0,
        );

        // 验证字段初始化
        assert_eq!(shark.get_name().as_ref().unwrap(), "Jaws");
        assert_eq!(shark.get_age(), 10);
        assert_eq!(shark.get_water_type().as_ref().unwrap(), "saltwater");
        assert_eq!(shark.get_scale_pattern().as_ref().unwrap(), "gray scales");
        assert_eq!(shark.get_swim_speed(), 15.0);
        assert_eq!(shark.get_teeth_count(), 300);

        println!("Shark created successfully: {}", shark.describe());
    }

    #[test]
    fn test_shark_methods() {
        // 测试 Shark 的方法
        let shark = Shark::new(
            "Bruce".to_string(),
            12,
            "saltwater".to_string(),
            "dark gray scales".to_string(),
            350,
            18.0,
        );

        let sound = shark.make_sound();
        let movement = shark.move_action();
        let description = shark.describe();

        // 验证方法返回非空字符串
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // 验证返回内容包含预期信息
        assert!(sound.contains("Bruce"));
        assert!(movement.contains("Bruce"));
        assert!(description.contains("Bruce"));
        assert!(description.contains("Shark"));
        assert!(description.contains("350"));

        println!("Shark sound: {}", sound);
        println!("Shark movement: {}", movement);
        println!("Shark description: {}", description);
    }

    #[test]
    fn test_shark_swimmable_mixin() {
        // 测试 Shark 的 Swimmable mixin 方法
        let shark = Shark::new(
            "Hunter".to_string(),
            11,
            "saltwater".to_string(),
            "gray scales".to_string(),
            400,
            20.0,
        );

        let swim_result = shark.swim();

        // 验证 mixin 方法返回非空字符串
        assert!(!swim_result.is_empty());

        // 验证返回内容包含预期信息
        assert!(swim_result.contains("Hunter"));
        assert!(swim_result.contains("swimming"));
        assert!(swim_result.contains("20"));

        println!("Shark swim: {}", swim_result);
    }

    #[test]
    fn test_create_salmon() {
        // 测试创建 Salmon 实例
        let salmon = Salmon::new(
            "Sammy".to_string(),
            4,
            "freshwater".to_string(),
            "silver scales".to_string(),
            "Alaska River".to_string(),
            8.0,
        );

        // 验证字段初始化
        assert_eq!(salmon.get_name().as_ref().unwrap(), "Sammy");
        assert_eq!(salmon.get_age(), 4);
        assert_eq!(salmon.get_water_type().as_ref().unwrap(), "freshwater");
        assert_eq!(
            salmon.get_scale_pattern().as_ref().unwrap(),
            "silver scales"
        );
        assert_eq!(salmon.get_swim_speed(), 8.0);
        assert_eq!(
            salmon.get_spawning_ground().as_ref().unwrap(),
            "Alaska River"
        );

        println!("Salmon created successfully: {}", salmon.describe());
    }

    #[test]
    fn test_salmon_methods() {
        // 测试 Salmon 的方法
        let salmon = Salmon::new(
            "Splash".to_string(),
            3,
            "freshwater".to_string(),
            "pink scales".to_string(),
            "Pacific River".to_string(),
            9.0,
        );

        let sound = salmon.make_sound();
        let movement = salmon.move_action();
        let description = salmon.describe();

        // 验证方法返回非空字符串
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // 验证返回内容包含预期信息
        assert!(sound.contains("Splash"));
        assert!(movement.contains("Splash"));
        assert!(description.contains("Splash"));
        assert!(description.contains("Salmon"));
        assert!(description.contains("Pacific River"));

        println!("Salmon sound: {}", sound);
        println!("Salmon movement: {}", movement);
        println!("Salmon description: {}", description);
    }

    #[test]
    fn test_salmon_swimmable_mixin() {
        // 测试 Salmon 的 Swimmable mixin 方法
        let salmon = Salmon::new(
            "Jumper".to_string(),
            5,
            "freshwater".to_string(),
            "silver scales".to_string(),
            "Columbia River".to_string(),
            10.0,
        );

        let swim_result = salmon.swim();

        // 验证 mixin 方法返回非空字符串
        assert!(!swim_result.is_empty());

        // 验证返回内容包含预期信息
        assert!(swim_result.contains("Jumper"));
        assert!(swim_result.contains("swimming"));
        assert!(swim_result.contains("10"));

        println!("Salmon swim: {}", swim_result);
    }

    #[test]
    fn test_create_flying_fish() {
        // 测试创建 FlyingFish 实例
        let flying_fish = FlyingFish::new(
            "Flipper".to_string(),
            2,
            "saltwater".to_string(),
            "blue scales".to_string(),
            200.0,
            50.0,
            10.0,
        );

        // 验证字段初始化
        assert_eq!(flying_fish.get_name().as_ref().unwrap(), "Flipper");
        assert_eq!(flying_fish.get_age(), 2);
        assert_eq!(flying_fish.get_water_type().as_ref().unwrap(), "saltwater");
        assert_eq!(
            flying_fish.get_scale_pattern().as_ref().unwrap(),
            "blue scales"
        );
        assert_eq!(flying_fish.get_max_altitude(), 50.0);
        assert_eq!(flying_fish.get_swim_speed(), 10.0);
        assert_eq!(flying_fish.get_glide_distance(), 200.0);

        println!(
            "FlyingFish created successfully: {}",
            flying_fish.describe()
        );
    }

    #[test]
    fn test_flying_fish_methods() {
        // 测试 FlyingFish 的方法
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            1,
            "saltwater".to_string(),
            "silver scales".to_string(),
            150.0,
            40.0,
            12.0,
        );

        let sound = flying_fish.make_sound();
        let movement = flying_fish.move_action();
        let description = flying_fish.describe();

        // 验证方法返回非空字符串
        assert!(!sound.is_empty());
        assert!(!movement.is_empty());
        assert!(!description.is_empty());

        // 验证返回内容包含预期信息
        assert!(sound.contains("Glider"));
        assert!(movement.contains("Glider"));
        assert!(description.contains("Glider"));
        assert!(description.contains("FlyingFish"));
        assert!(description.contains("150"));

        println!("FlyingFish sound: {}", sound);
        println!("FlyingFish movement: {}", movement);
        println!("FlyingFish description: {}", description);
    }

    #[test]
    fn test_flying_fish_flyable_mixin() {
        // 测试 FlyingFish 的 Flyable mixin 方法
        let flying_fish = FlyingFish::new(
            "Glide".to_string(),
            2,
            "saltwater".to_string(),
            "blue scales".to_string(),
            250.0,
            60.0,
            11.0,
        );

        let fly_result = flying_fish.fly();

        // 验证 mixin 方法返回非空字符串
        assert!(!fly_result.is_empty());

        // 验证返回内容包含预期信息
        assert!(fly_result.contains("Glide"));
        assert!(fly_result.contains("flying"));
        assert!(fly_result.contains("60"));

        println!("FlyingFish fly: {}", fly_result);
    }

    #[test]
    fn test_flying_fish_swimmable_mixin() {
        // 测试 FlyingFish 的 Swimmable mixin 方法
        let flying_fish = FlyingFish::new(
            "Splash".to_string(),
            1,
            "saltwater".to_string(),
            "silver scales".to_string(),
            180.0,
            45.0,
            13.0,
        );

        let swim_result = flying_fish.swim();

        // 验证 mixin 方法返回非空字符串
        assert!(!swim_result.is_empty());

        // 验证返回内容包含预期信息
        assert!(swim_result.contains("Splash"));
        assert!(swim_result.contains("swimming"));
        assert!(swim_result.contains("13"));

        println!("FlyingFish swim: {}", swim_result);
    }

    #[test]
    fn test_flying_fish_both_mixins() {
        // 测试 FlyingFish 同时使用 Flyable 和 Swimmable mixin
        let flying_fish = FlyingFish::new(
            "Hybrid".to_string(),
            3,
            "saltwater".to_string(),
            "rainbow scales".to_string(),
            220.0,
            55.0,
            12.0,
        );

        let fly_result = flying_fish.fly();
        let swim_result = flying_fish.swim();

        // 验证两个 mixin 方法都可以调用
        assert!(!fly_result.is_empty());
        assert!(!swim_result.is_empty());

        assert!(fly_result.contains("Hybrid"));
        assert!(swim_result.contains("Hybrid"));

        println!(
            "FlyingFish can both fly: {} and swim: {}",
            fly_result, swim_result
        );
    }

    // ========== 从 main.rs 移动过来的测试 ==========

    #[test]
    fn test_shark_multilevel_upcast() {
        use crate::Animal;
        use classes::prelude::*;

        println!("\n=== Testing Shark multilevel upcast: Shark -> Fish -> Animal ===");

        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );

        let original_sound = shark.make_sound();
        let original_desc = shark.describe();

        println!("Original Shark - Sound: {}", original_sound);

        // Shark -> Fish (with Swimmable)
        let fish = shark.clone().into_superclass::<CRc<Fish>>();
        let fish_sound = fish.make_sound();

        assert_eq!(original_sound, fish_sound);
        assert_eq!(fish.get_name().as_ref().unwrap(), "Jaws");
        assert_eq!(fish.get_age(), 10);

        // Fish -> Animal (with Scaled)
        let animal = fish.into_superclass::<CRc<Animal>>();
        let animal_sound = animal.make_sound();
        let animal_desc = animal.describe();

        assert_eq!(original_sound, animal_sound);
        assert_eq!(original_desc, animal_desc);
        assert_eq!(animal.get_name().as_ref().unwrap(), "Jaws");
        assert_eq!(animal.get_age(), 10);

        println!("✓ Shark multilevel upcast: behavior preserved");
    }

    #[test]
    fn test_salmon_multilevel_upcast() {
        use crate::Animal;
        use classes::prelude::*;

        println!("\n=== Testing Salmon multilevel upcast: Salmon -> Fish -> Animal ===");

        let salmon = Salmon::new(
            "Sockeye".to_string(),
            4,
            "Freshwater".to_string(),
            "Silver".to_string(),
            "Alaska River".to_string(),
            8.0,
        );

        let original_sound = salmon.make_sound();
        let original_desc = salmon.describe();

        println!("Original Salmon - Sound: {}", original_sound);

        // Salmon -> Fish (with Swimmable)
        let fish = salmon.clone().into_superclass::<CRc<Fish>>();
        let fish_sound = fish.make_sound();

        assert_eq!(original_sound, fish_sound);
        assert_eq!(fish.get_name().as_ref().unwrap(), "Sockeye");

        // Fish -> Animal (with Scaled)
        let animal = fish.into_superclass::<CRc<Animal>>();
        let animal_sound = animal.make_sound();
        let animal_desc = animal.describe();

        assert_eq!(original_sound, animal_sound);
        assert_eq!(original_desc, animal_desc);
        assert_eq!(animal.get_name().as_ref().unwrap(), "Sockeye");
        assert_eq!(animal.get_age(), 4);

        println!("✓ Salmon multilevel upcast: behavior preserved");
    }

    #[test]
    fn test_flying_fish_multilevel_upcast() {
        use crate::Animal;
        use classes::prelude::*;

        println!("\n=== Testing FlyingFish multilevel upcast: FlyingFish -> Fish -> Animal ===");

        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        let original_sound = flying_fish.make_sound();
        let original_desc = flying_fish.describe();

        println!("Original FlyingFish - Sound: {}", original_sound);

        // FlyingFish -> Fish (with Flyable + Swimmable)
        let fish = flying_fish.clone().into_superclass::<CRc<Fish>>();
        let fish_sound = fish.make_sound();

        assert_eq!(original_sound, fish_sound);
        assert_eq!(fish.get_name().as_ref().unwrap(), "Glider");

        // Fish -> Animal (with Scaled)
        let animal = fish.into_superclass::<CRc<Animal>>();
        let animal_sound = animal.make_sound();
        let animal_desc = animal.describe();

        assert_eq!(original_sound, animal_sound);
        assert_eq!(original_desc, animal_desc);
        assert_eq!(animal.get_name().as_ref().unwrap(), "Glider");
        assert_eq!(animal.get_age(), 2);

        println!("✓ FlyingFish multilevel upcast: behavior preserved");
    }

    #[test]
    fn test_shark_to_swimmable_mixin() {
        use crate::mixins;
        use classes::prelude::*;

        println!("\n=== Testing Shark -> Swimmable mixin conversion ===");

        // 创建 Shark 实例
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );

        // 记录原始字段值
        let original_name = shark.get_name().as_ref().unwrap().clone();
        let original_swim_speed = shark.get_swim_speed();

        println!(
            "Original Shark - Name: {}, Swim Speed: {}",
            original_name, original_swim_speed
        );

        // 转换为 Swimmable mixin 引用
        let swimmable: CRc<mixins::Swimmable> = shark.to_mixin();

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

        println!("✓ Shark -> Swimmable mixin conversion successful");
    }

    #[test]
    fn test_flying_fish_to_multiple_mixins() {
        use crate::mixins;
        use classes::prelude::*;

        println!("\n=== Testing FlyingFish -> Flyable and Swimmable mixin conversions ===");

        // 创建 FlyingFish 实例（同时使用 Flyable 和 Swimmable）
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        // 记录原始字段值
        let original_name = flying_fish.get_name().as_ref().unwrap().clone();
        let original_max_altitude = flying_fish.get_max_altitude();
        let original_swim_speed = flying_fish.get_swim_speed();

        println!(
            "Original FlyingFish - Name: {}, Max Altitude: {}, Swim Speed: {}",
            original_name, original_max_altitude, original_swim_speed
        );

        // 转换为 Flyable mixin 引用
        let flyable: CRc<mixins::Flyable> = flying_fish.clone().cast_mixin();
        let fly_result = flyable.fly();
        println!("Flyable.fly() result: {}", fly_result);

        // 验证 Flyable 方法和字段
        assert!(
            fly_result.contains(&original_name),
            "fly() should contain flying fish name"
        );
        assert_eq!(
            flyable.get_max_altitude(),
            original_max_altitude,
            "Max altitude should be accessible through Flyable mixin"
        );

        // 转换为 Swimmable mixin 引用
        let swimmable: CRc<mixins::Swimmable> = flying_fish.clone().cast_mixin();
        let swim_result = swimmable.swim();
        println!("Swimmable.swim() result: {}", swim_result);

        // 验证 Swimmable 方法和字段
        assert!(
            swim_result.contains(&original_name),
            "swim() should contain flying fish name"
        );
        assert_eq!(
            swimmable.get_swim_speed(),
            original_swim_speed,
            "Swim speed should be accessible through Swimmable mixin"
        );

        // 验证可以独立转换为每个 mixin
        println!(
            "✓ FlyingFish can be independently converted to both Flyable and Swimmable mixins"
        );

        // 验证通过不同 mixin 引用访问的 mixin 字段一致
        // 通过两个不同的 mixin 引用访问相同的 mixin 字段
        let flyable2: CRc<mixins::Flyable> = flying_fish.clone().cast_mixin();
        assert_eq!(
            flyable.get_max_altitude(),
            flyable2.get_max_altitude(),
            "Max altitude should be consistent across multiple Flyable references"
        );

        let swimmable2: CRc<mixins::Swimmable> = flying_fish.clone().cast_mixin();
        assert_eq!(
            swimmable.get_swim_speed(),
            swimmable2.get_swim_speed(),
            "Swim speed should be consistent across multiple Swimmable references"
        );

        println!(
            "✓ FlyingFish -> multiple mixins: fields are consistent across different mixin references"
        );
    }

    #[test]
    fn test_shark_full_conversion_chain() {
        use crate::Animal;
        use classes::prelude::*;

        println!(
            "\n=== Testing Shark full conversion chain: Shark -> Fish -> Animal -> Fish -> Shark ==="
        );

        // 创建 Shark 实例
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );

        // 记录原始字段值
        let original_name = shark.get_name().as_ref().unwrap().clone();
        let original_age = shark.get_age();
        let original_water_type = shark.get_water_type().as_ref().unwrap().clone();
        let original_scale_pattern = shark.get_scale_pattern().as_ref().unwrap().clone();
        let original_teeth_count = shark.get_teeth_count();
        let original_swim_speed = shark.get_swim_speed();
        let original_sound = shark.make_sound();
        let original_move = shark.move_action();
        let original_desc = shark.describe();

        println!("Original Shark:");
        println!("  Name: {}, Age: {}", original_name, original_age);
        println!(
            "  Water Type: {}, Scale Pattern: {}",
            original_water_type, original_scale_pattern
        );
        println!(
            "  Teeth Count: {}, Swim Speed: {}",
            original_teeth_count, original_swim_speed
        );

        // 第一步：Shark -> Fish
        println!("\n--- Step 1: Shark -> Fish ---");
        let fish = shark.clone().into_superclass::<CRc<Fish>>();

        // 验证转换成功
        assert_eq!(
            fish.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after Shark->Fish"
        );
        assert_eq!(
            fish.get_age(),
            original_age,
            "Age should be preserved after Shark->Fish"
        );
        assert_eq!(
            fish.get_water_type().as_ref().unwrap(),
            &original_water_type,
            "Water type should be preserved after Shark->Fish"
        );
        assert_eq!(
            fish.make_sound(),
            original_sound,
            "Sound should be preserved after Shark->Fish"
        );
        println!("✓ Shark -> Fish conversion successful, fields preserved");

        // 第二步：Fish -> Animal
        println!("\n--- Step 2: Fish -> Animal ---");
        let animal = fish.into_superclass::<CRc<Animal>>();

        // 验证转换成功
        assert_eq!(
            animal.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after Fish->Animal"
        );
        assert_eq!(
            animal.get_age(),
            original_age,
            "Age should be preserved after Fish->Animal"
        );
        assert_eq!(
            animal.make_sound(),
            original_sound,
            "Sound should be preserved after Fish->Animal"
        );
        assert_eq!(
            animal.describe(),
            original_desc,
            "Description should be preserved after Fish->Animal"
        );
        println!("✓ Fish -> Animal conversion successful, fields preserved");

        // 第三步：Animal -> Fish (向下转换)
        println!("\n--- Step 3: Animal -> Fish (downcast) ---");
        let fish_again = animal.try_into_subtype::<CRc<Fish>>();
        assert!(
            fish_again.is_some(),
            "Downcast from Animal to Fish should succeed"
        );

        let fish_ref = fish_again.unwrap();
        assert_eq!(
            fish_ref.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after Animal->Fish downcast"
        );
        assert_eq!(
            fish_ref.get_age(),
            original_age,
            "Age should be preserved after Animal->Fish downcast"
        );
        assert_eq!(
            fish_ref.get_water_type().as_ref().unwrap(),
            &original_water_type,
            "Water type should be preserved after Animal->Fish downcast"
        );
        assert_eq!(
            fish_ref.make_sound(),
            original_sound,
            "Sound should be preserved after Animal->Fish downcast"
        );
        println!("✓ Animal -> Fish downcast successful, fields preserved");

        // 第四步：Fish -> Shark (向下转换)
        println!("\n--- Step 4: Fish -> Shark (downcast) ---");
        let shark_again = fish_ref.try_into_subtype::<CRc<Shark>>();
        assert!(
            shark_again.is_some(),
            "Downcast from Fish to Shark should succeed"
        );

        let shark_final = shark_again.unwrap();

        // 验证最终恢复到原始类型，所有字段值保持不变
        assert_eq!(
            shark_final.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.get_age(),
            original_age,
            "Age should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.get_water_type().as_ref().unwrap(),
            &original_water_type,
            "Water type should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.get_scale_pattern().as_ref().unwrap(),
            &original_scale_pattern,
            "Scale pattern should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.get_teeth_count(),
            original_teeth_count,
            "Teeth count should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.get_swim_speed(),
            original_swim_speed,
            "Swim speed should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.make_sound(),
            original_sound,
            "Sound should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.move_action(),
            original_move,
            "Move action should be preserved in final Shark"
        );
        assert_eq!(
            shark_final.describe(),
            original_desc,
            "Description should be preserved in final Shark"
        );

        println!("✓ Fish -> Shark downcast successful, all fields preserved");
        println!(
            "\n✓ Complete conversion chain successful: Shark -> Fish -> Animal -> Fish -> Shark"
        );
        println!("  All fields and behaviors preserved through the entire chain");
    }
}
