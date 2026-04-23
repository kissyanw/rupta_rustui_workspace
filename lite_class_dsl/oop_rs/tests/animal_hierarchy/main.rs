// Animal Hierarchy Test Module
//
// 本模块演示使用 classes 宏实现的动物层次结构系统
// 包括抽象类、继承、多态、mixin 以及类型转换

// 声明所有子模块
mod animal;
mod bird;
mod cat;
mod dog;
mod fish;
mod mixins;

// 重新导出主要类型供测试使用
pub use animal::Animal;
pub use bird::{Bird, Duck, Eagle, Ostrich, Penguin};
pub use cat::Cat;
pub use dog::Dog;
pub use fish::{Fish, FlyingFish, Salmon, Shark};

#[cfg(test)]
mod tests {
    use super::*;
    use classes::prelude::*;

    #[test]
    fn prop_multilevel_upcast_preserves_identity() {
        println!("\n=== Property Test: Multilevel upcast preserves object identity ===");

        // 测试 Dog 向上转换
        let dog = Dog::new(
            "TestDog".to_string(),
            5,
            "Labrador".to_string(),
            "Black".to_string(),
        );
        let dog_sound = dog.make_sound();
        let dog_move = dog.move_action();
        let dog_desc = dog.describe();
        let dog_name = dog.get_name().as_ref().unwrap().clone();
        let dog_age = dog.get_age();

        let animal: CRc<Animal> = dog.into_superclass();
        assert_eq!(
            animal.make_sound(),
            dog_sound,
            "Dog sound should be preserved after upcast"
        );
        assert_eq!(
            animal.move_action(),
            dog_move,
            "Dog move should be preserved after upcast"
        );
        assert_eq!(
            animal.describe(),
            dog_desc,
            "Dog description should be preserved after upcast"
        );
        assert_eq!(
            animal.get_name().as_ref().unwrap(),
            &dog_name,
            "Dog name should be preserved"
        );
        assert_eq!(animal.get_age(), dog_age, "Dog age should be preserved");

        // 测试 Eagle 多级向上转换
        let eagle = Eagle::new(
            "TestEagle".to_string(),
            7,
            2.5,
            "Brown".to_string(),
            3000.0,
            50.0,
        );
        let eagle_sound = eagle.make_sound();
        let eagle_desc = eagle.describe();
        let eagle_name = eagle.get_name().as_ref().unwrap().clone();
        let eagle_age = eagle.get_age();

        // Eagle -> Bird -> Animal
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        assert_eq!(
            bird.make_sound(),
            eagle_sound,
            "Eagle sound should be preserved after Eagle->Bird"
        );
        assert_eq!(bird.get_name().as_ref().unwrap(), &eagle_name);

        let animal = bird.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            eagle_sound,
            "Eagle sound should be preserved after Bird->Animal"
        );
        assert_eq!(
            animal.describe(),
            eagle_desc,
            "Eagle description should be preserved"
        );
        assert_eq!(animal.get_name().as_ref().unwrap(), &eagle_name);
        assert_eq!(animal.get_age(), eagle_age);

        // 测试 Shark 多级向上转换
        let shark = Shark::new(
            "TestShark".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let shark_sound = shark.make_sound();
        let shark_desc = shark.describe();
        let shark_name = shark.get_name().as_ref().unwrap().clone();
        let shark_age = shark.get_age();

        // Shark -> Fish -> Animal
        let fish = shark.clone().into_superclass::<CRc<Fish>>();
        assert_eq!(
            fish.make_sound(),
            shark_sound,
            "Shark sound should be preserved after Shark->Fish"
        );
        assert_eq!(fish.get_name().as_ref().unwrap(), &shark_name);

        let animal = fish.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            shark_sound,
            "Shark sound should be preserved after Fish->Animal"
        );
        assert_eq!(
            animal.describe(),
            shark_desc,
            "Shark description should be preserved"
        );
        assert_eq!(animal.get_name().as_ref().unwrap(), &shark_name);
        assert_eq!(animal.get_age(), shark_age);

        println!(
            "✓ Property verified: Multilevel upcast preserves object identity for all tested types"
        );
    }

    #[test]
    fn test_downcast_animal_to_dog_success() {
        println!("\n=== Testing successful downcast: Animal -> Dog ===");

        // 创建 Dog 实例并向上转换到 Animal
        let dog = Dog::new(
            "Buddy".to_string(),
            5,
            "Golden Retriever".to_string(),
            "Golden".to_string(),
        );

        // 记录原始字段值
        let original_breed = dog.get_breed().as_ref().unwrap().clone();
        let original_fur_color = dog.get_fur_color().as_ref().unwrap().clone();
        let original_name = dog.get_name().as_ref().unwrap().clone();
        let original_age = dog.get_age();

        println!(
            "Original Dog - Breed: {}, Fur: {}",
            original_breed, original_fur_color
        );

        // 向上转换到 Animal
        let animal: CRc<Animal> = dog.into_superclass();

        // 尝试向下转换回 Dog
        let downcast_result = animal.try_into_subtype::<CRc<Dog>>();

        // 验证转换成功
        assert!(
            downcast_result.is_some(),
            "Downcast from Animal to Dog should succeed"
        );

        let dog_again = downcast_result.unwrap();

        // 验证可以访问派生类特定字段
        assert_eq!(
            dog_again.get_breed().as_ref().unwrap(),
            &original_breed,
            "Breed should be preserved after downcast"
        );
        assert_eq!(
            dog_again.get_fur_color().as_ref().unwrap(),
            &original_fur_color,
            "Fur color should be preserved after downcast"
        );
        assert_eq!(
            dog_again.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after downcast"
        );
        assert_eq!(
            dog_again.get_age(),
            original_age,
            "Age should be preserved after downcast"
        );

        println!("✓ Successful downcast: Animal -> Dog, all fields preserved");
    }

    #[test]
    fn test_downcast_animal_to_eagle_through_bird() {
        println!("\n=== Testing successful downcast: Animal -> Bird -> Eagle ===");

        // 创建 Eagle 实例并向上转换到 Animal
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        // 记录原始字段值
        let original_hunting_territory = eagle.get_hunting_territory_size();
        let original_max_altitude = eagle.get_max_altitude();
        let original_wingspan = eagle.get_wingspan();
        let original_feather_color = eagle.get_feather_color().as_ref().unwrap().clone();
        let original_name = eagle.get_name().as_ref().unwrap().clone();
        let original_age = eagle.get_age();

        println!(
            "Original Eagle - Territory: {}, Altitude: {}, Wingspan: {}",
            original_hunting_territory, original_max_altitude, original_wingspan
        );

        // 多级向上转换：Eagle -> Bird -> Animal
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        let animal = bird.into_superclass::<CRc<Animal>>();

        // 第一步：Animal -> Bird
        let downcast_to_bird = animal.try_into_subtype::<CRc<Bird>>();
        assert!(
            downcast_to_bird.is_some(),
            "Downcast from Animal to Bird should succeed"
        );

        let bird_again = downcast_to_bird.unwrap();

        // 验证 Bird 字段
        assert_eq!(
            bird_again.get_wingspan(),
            original_wingspan,
            "Wingspan should be preserved"
        );
        assert_eq!(
            bird_again.get_feather_color().as_ref().unwrap(),
            &original_feather_color,
            "Feather color should be preserved"
        );

        // 第二步：Bird -> Eagle
        let downcast_to_eagle = bird_again.try_into_subtype::<CRc<Eagle>>();
        assert!(
            downcast_to_eagle.is_some(),
            "Downcast from Bird to Eagle should succeed"
        );

        let eagle_again = downcast_to_eagle.unwrap();

        // 验证所有 Eagle 特定字段
        assert_eq!(
            eagle_again.get_hunting_territory_size(),
            original_hunting_territory,
            "Hunting territory should be preserved"
        );
        assert_eq!(
            eagle_again.get_max_altitude(),
            original_max_altitude,
            "Max altitude should be preserved"
        );
        assert_eq!(
            eagle_again.get_wingspan(),
            original_wingspan,
            "Wingspan should be preserved"
        );
        assert_eq!(
            eagle_again.get_feather_color().as_ref().unwrap(),
            &original_feather_color,
            "Feather color should be preserved"
        );
        assert_eq!(
            eagle_again.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved"
        );
        assert_eq!(
            eagle_again.get_age(),
            original_age,
            "Age should be preserved"
        );

        println!("✓ Successful multilevel downcast: Animal -> Bird -> Eagle, all fields preserved");
    }

    #[test]
    fn test_downcast_animal_to_shark_through_fish() {
        println!("\n=== Testing successful downcast: Animal -> Fish -> Shark ===");

        // 创建 Shark 实例并向上转换到 Animal
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );

        // 记录原始字段值
        let original_teeth_count = shark.get_teeth_count();
        let original_swim_speed = shark.get_swim_speed();
        let original_water_type = shark.get_water_type().as_ref().unwrap().clone();
        let original_scale_pattern = shark.get_scale_pattern().as_ref().unwrap().clone();
        let original_name = shark.get_name().as_ref().unwrap().clone();
        let original_age = shark.get_age();

        println!(
            "Original Shark - Teeth: {}, Speed: {}, Water: {}",
            original_teeth_count, original_swim_speed, original_water_type
        );

        // 多级向上转换：Shark -> Fish -> Animal
        let fish = shark.clone().into_superclass::<CRc<Fish>>();
        let animal = fish.into_superclass::<CRc<Animal>>();

        // 第一步：Animal -> Fish
        let downcast_to_fish = animal.try_into_subtype::<CRc<Fish>>();
        assert!(
            downcast_to_fish.is_some(),
            "Downcast from Animal to Fish should succeed"
        );

        let fish_again = downcast_to_fish.unwrap();

        // 验证 Fish 字段
        assert_eq!(
            fish_again.get_water_type().as_ref().unwrap(),
            &original_water_type,
            "Water type should be preserved"
        );
        assert_eq!(
            fish_again.get_scale_pattern().as_ref().unwrap(),
            &original_scale_pattern,
            "Scale pattern should be preserved"
        );

        // 第二步：Fish -> Shark
        let downcast_to_shark = fish_again.try_into_subtype::<CRc<Shark>>();
        assert!(
            downcast_to_shark.is_some(),
            "Downcast from Fish to Shark should succeed"
        );

        let shark_again = downcast_to_shark.unwrap();

        // 验证所有 Shark 特定字段
        assert_eq!(
            shark_again.get_teeth_count(),
            original_teeth_count,
            "Teeth count should be preserved"
        );
        assert_eq!(
            shark_again.get_swim_speed(),
            original_swim_speed,
            "Swim speed should be preserved"
        );
        assert_eq!(
            shark_again.get_water_type().as_ref().unwrap(),
            &original_water_type,
            "Water type should be preserved"
        );
        assert_eq!(
            shark_again.get_scale_pattern().as_ref().unwrap(),
            &original_scale_pattern,
            "Scale pattern should be preserved"
        );
        assert_eq!(
            shark_again.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved"
        );
        assert_eq!(
            shark_again.get_age(),
            original_age,
            "Age should be preserved"
        );

        println!("✓ Successful multilevel downcast: Animal -> Fish -> Shark, all fields preserved");
    }

    #[test]
    fn test_downcast_animal_dog_to_cat_failure() {
        println!("\n=== Testing failed downcast: Animal(Dog) -> Cat ===");

        // 创建 Dog 实例并向上转换到 Animal
        let dog = Dog::new(
            "Buddy".to_string(),
            5,
            "Golden Retriever".to_string(),
            "Golden".to_string(),
        );

        println!("Created Dog instance: {}", dog.get_name().as_ref().unwrap());

        // 向上转换到 Animal
        let animal: CRc<Animal> = dog.into_superclass();

        // 尝试向下转换到 Cat（应该失败）
        let downcast_result = animal.try_into_subtype::<CRc<Cat>>();

        // 验证转换失败（返回 None）
        assert!(
            downcast_result.is_none(),
            "Downcast from Animal(Dog) to Cat should fail and return None"
        );

        println!("✓ Failed downcast correctly returned None: Animal(Dog) -> Cat");
    }

    #[test]
    fn test_downcast_animal_eagle_to_penguin_failure() {
        println!("\n=== Testing failed downcast: Animal(Eagle) -> Penguin ===");

        // 创建 Eagle 实例并向上转换到 Animal
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        println!(
            "Created Eagle instance: {}",
            eagle.get_name().as_ref().unwrap()
        );

        // 多级向上转换到 Animal
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        let animal = bird.into_superclass::<CRc<Animal>>();

        // 尝试向下转换到 Penguin（应该失败）
        let downcast_result = animal.try_into_subtype::<CRc<Penguin>>();

        // 验证转换失败（返回 None）
        assert!(
            downcast_result.is_none(),
            "Downcast from Animal(Eagle) to Penguin should fail and return None"
        );

        println!("✓ Failed downcast correctly returned None: Animal(Eagle) -> Penguin");
    }

    #[test]
    fn test_downcast_bird_eagle_to_duck_failure() {
        println!("\n=== Testing failed downcast: Bird(Eagle) -> Duck ===");

        // 创建 Eagle 实例并向上转换到 Bird
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        println!(
            "Created Eagle instance: {}",
            eagle.get_name().as_ref().unwrap()
        );

        // 向上转换到 Bird
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();

        // 尝试向下转换到 Duck（应该失败）
        let downcast_result = bird.try_into_subtype::<CRc<Duck>>();

        // 验证转换失败（返回 None）
        assert!(
            downcast_result.is_none(),
            "Downcast from Bird(Eagle) to Duck should fail and return None"
        );

        println!("✓ Failed downcast correctly returned None: Bird(Eagle) -> Duck");
    }

    #[test]
    fn test_downcast_fish_shark_to_salmon_failure() {
        println!("\n=== Testing failed downcast: Fish(Shark) -> Salmon ===");

        // 创建 Shark 实例并向上转换到 Fish
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );

        println!(
            "Created Shark instance: {}",
            shark.get_name().as_ref().unwrap()
        );

        // 向上转换到 Fish
        let fish = shark.clone().into_superclass::<CRc<Fish>>();

        // 尝试向下转换到 Salmon（应该失败）
        let downcast_result = fish.try_into_subtype::<CRc<Salmon>>();

        // 验证转换失败（返回 None）
        assert!(
            downcast_result.is_none(),
            "Downcast from Fish(Shark) to Salmon should fail and return None"
        );

        println!("✓ Failed downcast correctly returned None: Fish(Shark) -> Salmon");
    }

    #[test]
    fn test_downcast_does_not_panic() {
        println!("\n=== Testing that failed downcasts do not panic ===");

        // 测试多种失败的向下转换，确保都不会 panic

        // Dog -> Cat
        let dog = Dog::new(
            "Test".to_string(),
            1,
            "Breed".to_string(),
            "Color".to_string(),
        );
        let animal: CRc<Animal> = dog.into_superclass();
        let result = animal.try_into_subtype::<CRc<Cat>>();
        assert!(result.is_none(), "Dog -> Cat should return None");

        // Cat -> Dog
        let cat = Cat::new("Test".to_string(), 1, true, "Color".to_string());
        let animal: CRc<Animal> = cat.into_superclass();
        let result = animal.try_into_subtype::<CRc<Dog>>();
        assert!(result.is_none(), "Cat -> Dog should return None");

        // Eagle -> Penguin
        let eagle = Eagle::new(
            "Test".to_string(),
            1,
            1.0,
            "Color".to_string(),
            1000.0,
            10.0,
        );
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        let result = bird.try_into_subtype::<CRc<Penguin>>();
        assert!(result.is_none(), "Eagle -> Penguin should return None");

        // Shark -> Salmon
        let shark = Shark::new(
            "Test".to_string(),
            1,
            "Water".to_string(),
            "Pattern".to_string(),
            100,
            10.0,
        );
        let fish = shark.clone().into_superclass::<CRc<Fish>>();
        let result = fish.try_into_subtype::<CRc<Salmon>>();
        assert!(result.is_none(), "Shark -> Salmon should return None");

        // Duck -> Ostrich
        let duck = Duck::new(
            "Test".to_string(),
            1,
            1.0,
            "Color".to_string(),
            1000.0,
            10.0,
            1000.0,
        );
        let bird = duck.clone().into_superclass::<CRc<Bird>>();
        let result = bird.try_into_subtype::<CRc<Ostrich>>();
        assert!(result.is_none(), "Duck -> Ostrich should return None");

        println!("✓ All failed downcasts returned None without panicking");
    }

    #[test]
    fn prop_downcast_type_safety() {
        println!("\n=== Property Test: Downcast type safety ===");

        // 测试 1: Dog 向上转换后，正确向下转换应该成功
        println!("\n--- Testing Dog downcast type safety ---");
        let dog = Dog::new(
            "TestDog".to_string(),
            5,
            "Labrador".to_string(),
            "Black".to_string(),
        );
        let dog_breed = dog.get_breed().as_ref().unwrap().clone();

        let animal: CRc<Animal> = dog.into_superclass();

        // 正确的向下转换应该返回 Some
        let downcast_to_dog = animal.clone().try_into_subtype::<CRc<Dog>>();
        assert!(
            downcast_to_dog.is_some(),
            "Downcast to correct type (Dog) should return Some"
        );
        let dog_again = downcast_to_dog.unwrap();
        assert_eq!(
            dog_again.get_breed().as_ref().unwrap(),
            &dog_breed,
            "Fields should be preserved after correct downcast"
        );

        // 错误的向下转换应该返回 None
        let downcast_to_cat = animal.try_into_subtype::<CRc<Cat>>();
        assert!(
            downcast_to_cat.is_none(),
            "Downcast to incorrect type (Cat) should return None"
        );

        // 测试 2: Eagle 向上转换后的类型安全性
        println!("\n--- Testing Eagle downcast type safety ---");
        let eagle = Eagle::new(
            "TestEagle".to_string(),
            7,
            2.5,
            "Brown".to_string(),
            3000.0,
            50.0,
        );
        let eagle_territory = eagle.get_hunting_territory_size();

        // Eagle -> Bird -> Animal
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        let animal = bird.into_superclass::<CRc<Animal>>();

        // 从 Animal 向下转换到 Bird 应该成功
        let downcast_to_bird = animal.clone().try_into_subtype::<CRc<Bird>>();
        assert!(
            downcast_to_bird.is_some(),
            "Downcast Animal to Bird should succeed"
        );

        // 从 Bird 向下转换到 Eagle 应该成功
        let bird_ref = downcast_to_bird.unwrap();
        let downcast_to_eagle = bird_ref.try_into_subtype::<CRc<Eagle>>();
        assert!(
            downcast_to_eagle.is_some(),
            "Downcast Bird to Eagle should succeed"
        );
        let eagle_again = downcast_to_eagle.unwrap();
        assert_eq!(
            eagle_again.get_hunting_territory_size(),
            eagle_territory,
            "Eagle fields should be preserved"
        );

        // 从 Animal 向下转换到错误的类型应该失败
        let downcast_to_penguin = animal.clone().try_into_subtype::<CRc<Penguin>>();
        assert!(
            downcast_to_penguin.is_none(),
            "Downcast Animal(Eagle) to Penguin should return None"
        );

        let downcast_to_dog = animal.try_into_subtype::<CRc<Dog>>();
        assert!(
            downcast_to_dog.is_none(),
            "Downcast Animal(Eagle) to Dog should return None"
        );

        // 测试 3: Shark 向上转换后的类型安全性
        println!("\n--- Testing Shark downcast type safety ---");
        let shark = Shark::new(
            "TestShark".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let shark_teeth = shark.get_teeth_count();

        // Shark -> Fish -> Animal
        let fish = shark.clone().into_superclass::<CRc<Fish>>();
        let animal = fish.into_superclass::<CRc<Animal>>();

        // 从 Animal 向下转换到 Fish 应该成功
        let downcast_to_fish = animal.clone().try_into_subtype::<CRc<Fish>>();
        assert!(
            downcast_to_fish.is_some(),
            "Downcast Animal to Fish should succeed"
        );

        // 从 Fish 向下转换到 Shark 应该成功
        let fish_ref = downcast_to_fish.unwrap();
        let downcast_to_shark = fish_ref.try_into_subtype::<CRc<Shark>>();
        assert!(
            downcast_to_shark.is_some(),
            "Downcast Fish to Shark should succeed"
        );
        let shark_again = downcast_to_shark.unwrap();
        assert_eq!(
            shark_again.get_teeth_count(),
            shark_teeth,
            "Shark fields should be preserved"
        );

        // 从 Animal 向下转换到错误的类型应该失败
        let downcast_to_salmon = animal.clone().try_into_subtype::<CRc<Salmon>>();
        assert!(
            downcast_to_salmon.is_none(),
            "Downcast Animal(Shark) to Salmon should return None"
        );

        let downcast_to_cat = animal.try_into_subtype::<CRc<Cat>>();
        assert!(
            downcast_to_cat.is_none(),
            "Downcast Animal(Shark) to Cat should return None"
        );

        // 测试 4: 多种动物类型的类型安全性
        println!("\n--- Testing multiple animal types ---");

        // Cat
        let cat = Cat::new("TestCat".to_string(), 3, true, "Gray".to_string());
        let animal: CRc<Animal> = cat.into_superclass();
        assert!(
            animal.clone().try_into_subtype::<CRc<Cat>>().is_some(),
            "Cat should downcast to Cat"
        );
        assert!(
            animal.try_into_subtype::<CRc<Dog>>().is_none(),
            "Cat should not downcast to Dog"
        );

        // Penguin
        let penguin = Penguin::new(
            "TestPenguin".to_string(),
            4,
            0.8,
            "Black".to_string(),
            5.0,
            1000,
        );
        let bird = penguin.clone().into_superclass::<CRc<Bird>>();
        assert!(
            bird.clone().try_into_subtype::<CRc<Penguin>>().is_some(),
            "Penguin should downcast to Penguin"
        );
        assert!(
            bird.try_into_subtype::<CRc<Eagle>>().is_none(),
            "Penguin should not downcast to Eagle"
        );

        // Salmon
        let salmon = Salmon::new(
            "TestSalmon".to_string(),
            4,
            "Freshwater".to_string(),
            "Silver".to_string(),
            "River".to_string(),
            8.0,
        );
        let fish = salmon.clone().into_superclass::<CRc<Fish>>();
        assert!(
            fish.clone().try_into_subtype::<CRc<Salmon>>().is_some(),
            "Salmon should downcast to Salmon"
        );
        assert!(
            fish.try_into_subtype::<CRc<Shark>>().is_none(),
            "Salmon should not downcast to Shark"
        );

        println!(
            "✓ Property verified: Downcast type safety - correct types return Some, incorrect types return None"
        );
    }

    #[test]
    fn test_mixin_reference_back_conversion() {
        println!("\n=== Testing Mixin reference back conversion ===");

        // 测试 Eagle: Eagle -> Flyable -> Eagle
        println!("\n--- Testing Eagle -> Flyable -> Eagle ---");
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        // 记录原始字段值
        let original_name = eagle.get_name().as_ref().unwrap().clone();
        let original_age = eagle.get_age();
        let original_wingspan = eagle.get_wingspan();
        let original_feather_color = eagle.get_feather_color().as_ref().unwrap().clone();
        let original_max_altitude = eagle.get_max_altitude();
        let original_hunting_territory = eagle.get_hunting_territory_size();

        println!(
            "Original Eagle - Name: {}, Age: {}, Wingspan: {}, Territory: {}",
            original_name, original_age, original_wingspan, original_hunting_territory
        );

        // 转换为 Flyable mixin 引用
        let flyable: CRc<mixins::Flyable> = eagle.clone().cast_mixin();
        println!("Converted to Flyable mixin");

        // 从 mixin 引用转换回 Animal，然后向下转换到 Eagle
        let animal: CRc<Animal> = flyable.mixin_to_impl();
        let eagle_again = animal
            .try_into_subtype::<CRc<Eagle>>()
            .expect("Should be able to downcast back to Eagle");
        println!("Converted back to Eagle");

        // 验证所有字段值保持不变
        assert_eq!(
            eagle_again.get_name().as_ref().unwrap(),
            &original_name,
            "Name should be preserved after round-trip conversion"
        );
        assert_eq!(
            eagle_again.get_age(),
            original_age,
            "Age should be preserved"
        );
        assert_eq!(
            eagle_again.get_wingspan(),
            original_wingspan,
            "Wingspan should be preserved"
        );
        assert_eq!(
            eagle_again.get_feather_color().as_ref().unwrap(),
            &original_feather_color,
            "Feather color should be preserved"
        );
        assert_eq!(
            eagle_again.get_max_altitude(),
            original_max_altitude,
            "Max altitude should be preserved"
        );
        assert_eq!(
            eagle_again.get_hunting_territory_size(),
            original_hunting_territory,
            "Hunting territory should be preserved"
        );

        println!("✓ Eagle -> Flyable -> Eagle: all fields preserved");

        // 测试 Duck: Duck -> Swimmable -> Duck
        println!("\n--- Testing Duck -> Swimmable -> Duck ---");
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );

        let duck_name = duck.get_name().as_ref().unwrap().clone();
        let duck_age = duck.get_age();
        let duck_wingspan = duck.get_wingspan();
        let duck_swim_speed = duck.get_swim_speed();
        let duck_max_altitude = duck.get_max_altitude();
        let duck_migration_distance = duck.get_migration_distance();

        println!(
            "Original Duck - Name: {}, Swim Speed: {}, Max Altitude: {}, Migration: {}",
            duck_name, duck_swim_speed, duck_max_altitude, duck_migration_distance
        );

        // 转换为 Swimmable mixin 引用
        let swimmable: CRc<mixins::Swimmable> = duck.clone().cast_mixin();
        println!("Converted to Swimmable mixin");

        // 从 mixin 引用转换回 Animal，然后向下转换到 Duck
        let animal: CRc<Animal> = swimmable.mixin_to_impl();
        let duck_again = animal
            .try_into_subtype::<CRc<Duck>>()
            .expect("Should be able to downcast back to Duck");
        println!("Converted back to Duck");

        // 验证所有字段值保持不变
        assert_eq!(
            duck_again.get_name().as_ref().unwrap(),
            &duck_name,
            "Duck name should be preserved"
        );
        assert_eq!(
            duck_again.get_age(),
            duck_age,
            "Duck age should be preserved"
        );
        assert_eq!(
            duck_again.get_wingspan(),
            duck_wingspan,
            "Duck wingspan should be preserved"
        );
        assert_eq!(
            duck_again.get_swim_speed(),
            duck_swim_speed,
            "Duck swim speed should be preserved"
        );
        assert_eq!(
            duck_again.get_max_altitude(),
            duck_max_altitude,
            "Duck max altitude should be preserved"
        );
        assert_eq!(
            duck_again.get_migration_distance(),
            duck_migration_distance,
            "Duck migration distance should be preserved"
        );

        println!("✓ Duck -> Swimmable -> Duck: all fields preserved");

        // 测试 FlyingFish: FlyingFish -> Flyable -> FlyingFish
        println!("\n--- Testing FlyingFish -> Flyable -> FlyingFish ---");
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        let fish_name = flying_fish.get_name().as_ref().unwrap().clone();
        let fish_age = flying_fish.get_age();
        let fish_water_type = flying_fish.get_water_type().as_ref().unwrap().clone();
        let fish_scale_pattern = flying_fish.get_scale_pattern().as_ref().unwrap().clone();
        let fish_max_altitude = flying_fish.get_max_altitude();
        let fish_swim_speed = flying_fish.get_swim_speed();
        let fish_glide_distance = flying_fish.get_glide_distance();

        println!(
            "Original FlyingFish - Name: {}, Max Altitude: {}, Swim Speed: {}, Glide: {}",
            fish_name, fish_max_altitude, fish_swim_speed, fish_glide_distance
        );

        // 转换为 Flyable mixin 引用
        let flyable: CRc<mixins::Flyable> = flying_fish.clone().cast_mixin();
        println!("Converted to Flyable mixin");

        // 从 mixin 引用转换回 Animal，然后向下转换到 FlyingFish
        let animal: CRc<Animal> = flyable.mixin_to_impl();
        let fish_again = animal
            .try_into_subtype::<CRc<FlyingFish>>()
            .expect("Should be able to downcast back to FlyingFish");
        println!("Converted back to FlyingFish");

        // 验证所有字段值保持不变
        assert_eq!(
            fish_again.get_name().as_ref().unwrap(),
            &fish_name,
            "FlyingFish name should be preserved"
        );
        assert_eq!(
            fish_again.get_age(),
            fish_age,
            "FlyingFish age should be preserved"
        );
        assert_eq!(
            fish_again.get_water_type().as_ref().unwrap(),
            &fish_water_type,
            "Water type should be preserved"
        );
        assert_eq!(
            fish_again.get_scale_pattern().as_ref().unwrap(),
            &fish_scale_pattern,
            "Scale pattern should be preserved"
        );
        assert_eq!(
            fish_again.get_max_altitude(),
            fish_max_altitude,
            "Max altitude should be preserved"
        );
        assert_eq!(
            fish_again.get_swim_speed(),
            fish_swim_speed,
            "Swim speed should be preserved"
        );
        assert_eq!(
            fish_again.get_glide_distance(),
            fish_glide_distance,
            "Glide distance should be preserved"
        );

        println!("✓ FlyingFish -> Flyable -> FlyingFish: all fields preserved");
        println!("\n✓ All mixin reference back conversions successful with fields preserved");
    }

    #[test]
    fn prop_mixin_reference_access_integrity() {
        println!("\n=== Property Test: Mixin reference access integrity ===");

        // 测试 Eagle 的 Flyable mixin 访问
        println!("\n--- Testing Eagle Flyable mixin access ---");
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);
        let original_max_altitude = eagle.get_max_altitude();

        let flyable: CRc<mixins::Flyable> = eagle.clone().cast_mixin();
        assert_eq!(
            flyable.get_max_altitude(),
            original_max_altitude,
            "Max altitude accessed through Flyable mixin should match original"
        );

        // 测试 Penguin 的 Swimmable mixin 访问
        println!("--- Testing Penguin Swimmable mixin access ---");
        let penguin = Penguin::new("Pingu".to_string(), 4, 0.8, "Black".to_string(), 5.0, 1000);
        let original_swim_speed = penguin.get_swim_speed();

        let swimmable: CRc<mixins::Swimmable> = penguin.clone().cast_mixin();
        assert_eq!(
            swimmable.get_swim_speed(),
            original_swim_speed,
            "Swim speed accessed through Swimmable mixin should match original"
        );

        // 测试 Duck 的多个 mixin 访问
        println!("--- Testing Duck multiple mixin access ---");
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );
        let duck_max_altitude = duck.get_max_altitude();
        let duck_swim_speed = duck.get_swim_speed();

        let flyable: CRc<mixins::Flyable> = duck.clone().cast_mixin();
        assert_eq!(
            flyable.get_max_altitude(),
            duck_max_altitude,
            "Duck max altitude through Flyable should match original"
        );

        let swimmable: CRc<mixins::Swimmable> = duck.clone().cast_mixin();
        assert_eq!(
            swimmable.get_swim_speed(),
            duck_swim_speed,
            "Duck swim speed through Swimmable should match original"
        );

        // 测试 Shark 的 Swimmable mixin 访问
        println!("--- Testing Shark Swimmable mixin access ---");
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let shark_swim_speed = shark.get_swim_speed();

        let swimmable: CRc<mixins::Swimmable> = shark.clone().cast_mixin();
        assert_eq!(
            swimmable.get_swim_speed(),
            shark_swim_speed,
            "Shark swim speed through Swimmable should match original"
        );

        // 测试 FlyingFish 的多个 mixin 访问
        println!("--- Testing FlyingFish multiple mixin access ---");
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );
        let fish_max_altitude = flying_fish.get_max_altitude();
        let fish_swim_speed = flying_fish.get_swim_speed();

        let flyable: CRc<mixins::Flyable> = flying_fish.clone().cast_mixin();
        assert_eq!(
            flyable.get_max_altitude(),
            fish_max_altitude,
            "FlyingFish max altitude through Flyable should match original"
        );

        let swimmable: CRc<mixins::Swimmable> = flying_fish.clone().cast_mixin();
        assert_eq!(
            swimmable.get_swim_speed(),
            fish_swim_speed,
            "FlyingFish swim speed through Swimmable should match original"
        );

        println!(
            "\n✓ Property verified: Mixin reference access integrity - all mixin field accesses match original values"
        );
    }

    #[test]
    fn prop_mixin_bidirectional_conversion() {
        println!("\n=== Property Test: Mixin bidirectional conversion ===");

        // 测试 Eagle: Eagle -> Flyable -> Animal -> Eagle
        println!("\n--- Testing Eagle bidirectional conversion ---");
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);
        let original_name = eagle.get_name().as_ref().unwrap().clone();
        let original_age = eagle.get_age();
        let original_wingspan = eagle.get_wingspan();
        let original_max_altitude = eagle.get_max_altitude();
        let original_hunting_territory = eagle.get_hunting_territory_size();

        // Eagle -> Flyable
        let flyable: CRc<mixins::Flyable> = eagle.clone().cast_mixin();
        // Flyable -> Animal
        let animal: CRc<Animal> = flyable.mixin_to_impl();
        // Animal -> Eagle
        let eagle_again = animal
            .try_into_subtype::<CRc<Eagle>>()
            .expect("Should convert back to Eagle");

        assert_eq!(
            eagle_again.get_name().as_ref().unwrap(),
            &original_name,
            "Eagle name should be preserved through bidirectional conversion"
        );
        assert_eq!(
            eagle_again.get_age(),
            original_age,
            "Eagle age should be preserved"
        );
        assert_eq!(
            eagle_again.get_wingspan(),
            original_wingspan,
            "Eagle wingspan should be preserved"
        );
        assert_eq!(
            eagle_again.get_max_altitude(),
            original_max_altitude,
            "Eagle max altitude should be preserved"
        );
        assert_eq!(
            eagle_again.get_hunting_territory_size(),
            original_hunting_territory,
            "Eagle hunting territory should be preserved"
        );

        // 测试 Duck: Duck -> Swimmable -> Animal -> Duck
        println!("--- Testing Duck bidirectional conversion ---");
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );
        let duck_name = duck.get_name().as_ref().unwrap().clone();
        let duck_age = duck.get_age();
        let duck_wingspan = duck.get_wingspan();
        let duck_swim_speed = duck.get_swim_speed();
        let duck_max_altitude = duck.get_max_altitude();
        let duck_migration = duck.get_migration_distance();

        // Duck -> Swimmable
        let swimmable: CRc<mixins::Swimmable> = duck.clone().cast_mixin();
        // Swimmable -> Animal
        let animal: CRc<Animal> = swimmable.mixin_to_impl();
        // Animal -> Duck
        let duck_again = animal
            .try_into_subtype::<CRc<Duck>>()
            .expect("Should convert back to Duck");

        assert_eq!(
            duck_again.get_name().as_ref().unwrap(),
            &duck_name,
            "Duck name should be preserved"
        );
        assert_eq!(
            duck_again.get_age(),
            duck_age,
            "Duck age should be preserved"
        );
        assert_eq!(
            duck_again.get_wingspan(),
            duck_wingspan,
            "Duck wingspan should be preserved"
        );
        assert_eq!(
            duck_again.get_swim_speed(),
            duck_swim_speed,
            "Duck swim speed should be preserved"
        );
        assert_eq!(
            duck_again.get_max_altitude(),
            duck_max_altitude,
            "Duck max altitude should be preserved"
        );
        assert_eq!(
            duck_again.get_migration_distance(),
            duck_migration,
            "Duck migration distance should be preserved"
        );

        // 测试 FlyingFish: FlyingFish -> Flyable -> Animal -> FlyingFish
        println!("--- Testing FlyingFish bidirectional conversion ---");
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );
        let fish_name = flying_fish.get_name().as_ref().unwrap().clone();
        let fish_age = flying_fish.get_age();
        let fish_water_type = flying_fish.get_water_type().as_ref().unwrap().clone();
        let fish_scale_pattern = flying_fish.get_scale_pattern().as_ref().unwrap().clone();
        let fish_max_altitude = flying_fish.get_max_altitude();
        let fish_swim_speed = flying_fish.get_swim_speed();
        let fish_glide_distance = flying_fish.get_glide_distance();

        // FlyingFish -> Flyable
        let flyable: CRc<mixins::Flyable> = flying_fish.clone().cast_mixin();
        // Flyable -> Animal
        let animal: CRc<Animal> = flyable.mixin_to_impl();
        // Animal -> FlyingFish
        let fish_again = animal
            .try_into_subtype::<CRc<FlyingFish>>()
            .expect("Should convert back to FlyingFish");

        assert_eq!(
            fish_again.get_name().as_ref().unwrap(),
            &fish_name,
            "FlyingFish name should be preserved"
        );
        assert_eq!(
            fish_again.get_age(),
            fish_age,
            "FlyingFish age should be preserved"
        );
        assert_eq!(
            fish_again.get_water_type().as_ref().unwrap(),
            &fish_water_type,
            "FlyingFish water type should be preserved"
        );
        assert_eq!(
            fish_again.get_scale_pattern().as_ref().unwrap(),
            &fish_scale_pattern,
            "FlyingFish scale pattern should be preserved"
        );
        assert_eq!(
            fish_again.get_max_altitude(),
            fish_max_altitude,
            "FlyingFish max altitude should be preserved"
        );
        assert_eq!(
            fish_again.get_swim_speed(),
            fish_swim_speed,
            "FlyingFish swim speed should be preserved"
        );
        assert_eq!(
            fish_again.get_glide_distance(),
            fish_glide_distance,
            "FlyingFish glide distance should be preserved"
        );

        println!(
            "\n✓ Property verified: Mixin bidirectional conversion - all fields preserved through round-trip"
        );
    }

    #[test]
    fn prop_multiple_mixin_independent_conversion() {
        println!("\n=== Property Test: Multiple mixin independent conversion ===");

        // 测试 Duck（使用 Flyable 和 Swimmable）
        println!("\n--- Testing Duck with Flyable and Swimmable ---");
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );

        let original_max_altitude = duck.get_max_altitude();
        let original_swim_speed = duck.get_swim_speed();

        // 独立转换为 Flyable
        let flyable1: CRc<mixins::Flyable> = duck.clone().cast_mixin();
        let altitude1 = flyable1.get_max_altitude();

        // 独立转换为 Swimmable
        let swimmable1: CRc<mixins::Swimmable> = duck.clone().cast_mixin();
        let speed1 = swimmable1.get_swim_speed();

        // 再次独立转换为 Flyable
        let flyable2: CRc<mixins::Flyable> = duck.clone().cast_mixin();
        let altitude2 = flyable2.get_max_altitude();

        // 再次独立转换为 Swimmable
        let swimmable2: CRc<mixins::Swimmable> = duck.clone().cast_mixin();
        let speed2 = swimmable2.get_swim_speed();

        // 验证通过不同 mixin 引用访问的值一致
        assert_eq!(
            altitude1, original_max_altitude,
            "First Flyable conversion should match original"
        );
        assert_eq!(
            altitude2, original_max_altitude,
            "Second Flyable conversion should match original"
        );
        assert_eq!(
            altitude1, altitude2,
            "Multiple Flyable conversions should be consistent"
        );

        assert_eq!(
            speed1, original_swim_speed,
            "First Swimmable conversion should match original"
        );
        assert_eq!(
            speed2, original_swim_speed,
            "Second Swimmable conversion should match original"
        );
        assert_eq!(
            speed1, speed2,
            "Multiple Swimmable conversions should be consistent"
        );

        println!(
            "✓ Duck: Multiple independent conversions to Flyable and Swimmable are consistent"
        );

        // 测试 FlyingFish（使用 Flyable 和 Swimmable）
        println!("\n--- Testing FlyingFish with Flyable and Swimmable ---");
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        let fish_max_altitude = flying_fish.get_max_altitude();
        let fish_swim_speed = flying_fish.get_swim_speed();

        // 独立转换为 Flyable
        let flyable1: CRc<mixins::Flyable> = flying_fish.clone().cast_mixin();
        let fish_altitude1 = flyable1.get_max_altitude();

        // 独立转换为 Swimmable
        let swimmable1: CRc<mixins::Swimmable> = flying_fish.clone().cast_mixin();
        let fish_speed1 = swimmable1.get_swim_speed();

        // 再次独立转换为 Flyable
        let flyable2: CRc<mixins::Flyable> = flying_fish.clone().cast_mixin();
        let fish_altitude2 = flyable2.get_max_altitude();

        // 再次独立转换为 Swimmable
        let swimmable2: CRc<mixins::Swimmable> = flying_fish.clone().cast_mixin();
        let fish_speed2 = swimmable2.get_swim_speed();

        // 验证通过不同 mixin 引用访问的值一致
        assert_eq!(
            fish_altitude1, fish_max_altitude,
            "First Flyable conversion should match original"
        );
        assert_eq!(
            fish_altitude2, fish_max_altitude,
            "Second Flyable conversion should match original"
        );
        assert_eq!(
            fish_altitude1, fish_altitude2,
            "Multiple Flyable conversions should be consistent"
        );

        assert_eq!(
            fish_speed1, fish_swim_speed,
            "First Swimmable conversion should match original"
        );
        assert_eq!(
            fish_speed2, fish_swim_speed,
            "Second Swimmable conversion should match original"
        );
        assert_eq!(
            fish_speed1, fish_speed2,
            "Multiple Swimmable conversions should be consistent"
        );

        println!(
            "✓ FlyingFish: Multiple independent conversions to Flyable and Swimmable are consistent"
        );

        println!(
            "\n✓ Property verified: Multiple mixin independent conversion - all conversions are consistent"
        );
    }

    #[test]
    fn test_polymorphic_collection() {
        println!("\n=== Testing polymorphic collection with all animal types ===");

        // 创建所有动物类型的实例
        let dog = Dog::new(
            "Buddy".to_string(),
            5,
            "Golden Retriever".to_string(),
            "Golden".to_string(),
        );
        let cat = Cat::new("Whiskers".to_string(), 3, true, "Gray".to_string());
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);
        let penguin = Penguin::new(
            "Pingu".to_string(),
            4,
            0.8,
            "Black and White".to_string(),
            5.0,
            1000,
        );
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );
        let ostrich = Ostrich::new("Ozzy".to_string(), 6, 2.0, "Black".to_string(), 70.0);
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let salmon = Salmon::new(
            "Sockeye".to_string(),
            4,
            "Freshwater".to_string(),
            "Silver".to_string(),
            "Alaska River".to_string(),
            8.0,
        );
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        println!("Created 9 different animal instances");

        // 创建多态集合：Vec<CRc<Animal>>
        let animals: Vec<CRc<Animal>> = vec![
            dog.into_superclass(),
            cat.into_superclass(),
            eagle
                .clone()
                .into_superclass::<CRc<Bird>>()
                .into_superclass(),
            penguin
                .clone()
                .into_superclass::<CRc<Bird>>()
                .into_superclass(),
            duck.clone()
                .into_superclass::<CRc<Bird>>()
                .into_superclass(),
            ostrich
                .clone()
                .into_superclass::<CRc<Bird>>()
                .into_superclass(),
            shark
                .clone()
                .into_superclass::<CRc<Fish>>()
                .into_superclass(),
            salmon
                .clone()
                .into_superclass::<CRc<Fish>>()
                .into_superclass(),
            flying_fish
                .clone()
                .into_superclass::<CRc<Fish>>()
                .into_superclass(),
        ];

        println!(
            "Created polymorphic collection with {} animals",
            animals.len()
        );
        assert_eq!(animals.len(), 9, "Collection should contain 9 animals");

        // 遍历集合并调用方法
        println!("\n--- Iterating through polymorphic collection ---");
        for (i, animal) in animals.iter().enumerate() {
            let sound = animal.make_sound();
            let movement = animal.move_action();
            let description = animal.describe();

            println!(
                "\nAnimal {}: {}",
                i + 1,
                animal.get_name().as_ref().unwrap()
            );
            println!("  Sound: {}", sound);
            println!("  Movement: {}", movement);
            println!("  Description: {}", description);

            // 验证每个动物都能正确响应
            assert!(
                !sound.is_empty(),
                "Animal {} should have non-empty sound",
                i + 1
            );
            assert!(
                !movement.is_empty(),
                "Animal {} should have non-empty movement",
                i + 1
            );
            assert!(
                !description.is_empty(),
                "Animal {} should have non-empty description",
                i + 1
            );

            // 验证描述包含动物名称
            assert!(
                description.contains(animal.get_name().as_ref().unwrap()),
                "Description should contain animal name"
            );
        }

        println!("\n✓ Polymorphic collection test passed: all 9 animals responded correctly");
    }

    #[test]
    fn test_polymorphic_collection_by_category() {
        println!("\n=== Testing polymorphic collections by category ===");

        // 创建鸟类多态集合
        println!("\n--- Testing Bird polymorphic collection ---");
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);
        let penguin = Penguin::new(
            "Pingu".to_string(),
            4,
            0.8,
            "Black and White".to_string(),
            5.0,
            1000,
        );
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );
        let ostrich = Ostrich::new("Ozzy".to_string(), 6, 2.0, "Black".to_string(), 70.0);

        let birds: Vec<CRc<Bird>> = vec![
            eagle.into_superclass(),
            penguin.into_superclass(),
            duck.into_superclass(),
            ostrich.into_superclass(),
        ];

        println!("Created Bird collection with {} birds", birds.len());
        for (i, bird) in birds.iter().enumerate() {
            let sound = bird.make_sound();
            let movement = bird.move_action();
            println!(
                "Bird {}: {} - Sound: {}, Movement: {}",
                i + 1,
                bird.get_name().as_ref().unwrap(),
                sound,
                movement
            );
            assert!(!sound.is_empty(), "Bird should have sound");
            assert!(!movement.is_empty(), "Bird should have movement");
        }

        // 创建鱼类多态集合
        println!("\n--- Testing Fish polymorphic collection ---");
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let salmon = Salmon::new(
            "Sockeye".to_string(),
            4,
            "Freshwater".to_string(),
            "Silver".to_string(),
            "Alaska River".to_string(),
            8.0,
        );
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        let fishes: Vec<CRc<Fish>> = vec![
            shark.into_superclass(),
            salmon.into_superclass(),
            flying_fish.into_superclass(),
        ];

        println!("Created Fish collection with {} fishes", fishes.len());
        for (i, fish) in fishes.iter().enumerate() {
            let sound = fish.make_sound();
            let movement = fish.move_action();
            println!(
                "Fish {}: {} - Sound: {}, Movement: {}",
                i + 1,
                fish.get_name().as_ref().unwrap(),
                sound,
                movement
            );
            assert!(!sound.is_empty(), "Fish should have sound");
            assert!(!movement.is_empty(), "Fish should have movement");
        }

        println!("\n✓ Polymorphic collections by category test passed");
    }

    #[test]
    fn test_polymorphic_method_calls() {
        println!("\n=== Testing polymorphic method calls ===");

        // 测试 Dog 的多态方法调用
        println!("\n--- Testing Dog polymorphic method calls ---");
        let dog = Dog::new(
            "Buddy".to_string(),
            5,
            "Golden Retriever".to_string(),
            "Golden".to_string(),
        );

        // 直接调用
        let direct_sound = dog.make_sound();
        let direct_move = dog.move_action();
        let direct_desc = dog.describe();

        println!("Direct call - Sound: {}", direct_sound);
        println!("Direct call - Move: {}", direct_move);

        // 通过 Animal 引用调用
        let animal: CRc<Animal> = dog.into_superclass();
        let polymorphic_sound = animal.make_sound();
        let polymorphic_move = animal.move_action();
        let polymorphic_desc = animal.describe();

        println!("Polymorphic call - Sound: {}", polymorphic_sound);
        println!("Polymorphic call - Move: {}", polymorphic_move);

        // 验证执行的是具体类的实现
        assert_eq!(
            direct_sound, polymorphic_sound,
            "Polymorphic call should execute Dog's make_sound implementation"
        );
        assert_eq!(
            direct_move, polymorphic_move,
            "Polymorphic call should execute Dog's move_action implementation"
        );
        assert_eq!(
            direct_desc, polymorphic_desc,
            "Polymorphic call should execute Dog's describe implementation"
        );

        println!("✓ Dog: Polymorphic calls match direct calls");

        // 测试 Eagle 的多态方法调用
        println!("\n--- Testing Eagle polymorphic method calls ---");
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        let direct_sound = eagle.make_sound();
        let direct_move = eagle.move_action();
        let direct_desc = eagle.describe();

        println!("Direct call - Sound: {}", direct_sound);
        println!("Direct call - Move: {}", direct_move);

        // 通过 Bird 引用调用
        let bird: CRc<Bird> = eagle.clone().into_superclass();
        let bird_sound = bird.make_sound();
        let bird_move = bird.move_action();
        let bird_desc = bird.describe();

        assert_eq!(
            direct_sound, bird_sound,
            "Bird reference should execute Eagle's implementation"
        );
        assert_eq!(
            direct_move, bird_move,
            "Bird reference should execute Eagle's implementation"
        );
        assert_eq!(
            direct_desc, bird_desc,
            "Bird reference should execute Eagle's implementation"
        );

        // 通过 Animal 引用调用
        let animal: CRc<Animal> = bird.into_superclass();
        let animal_sound = animal.make_sound();
        let animal_move = animal.move_action();
        let animal_desc = animal.describe();

        println!("Polymorphic call (Animal) - Sound: {}", animal_sound);
        println!("Polymorphic call (Animal) - Move: {}", animal_move);

        assert_eq!(
            direct_sound, animal_sound,
            "Animal reference should execute Eagle's implementation"
        );
        assert_eq!(
            direct_move, animal_move,
            "Animal reference should execute Eagle's implementation"
        );
        assert_eq!(
            direct_desc, animal_desc,
            "Animal reference should execute Eagle's implementation"
        );

        println!("✓ Eagle: Polymorphic calls through Bird and Animal match direct calls");

        // 测试 Shark 的多态方法调用
        println!("\n--- Testing Shark polymorphic method calls ---");
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );

        let direct_sound = shark.make_sound();
        let direct_move = shark.move_action();
        let direct_desc = shark.describe();

        println!("Direct call - Sound: {}", direct_sound);
        println!("Direct call - Move: {}", direct_move);

        // 通过 Fish 引用调用
        let fish: CRc<Fish> = shark.clone().into_superclass();
        let fish_sound = fish.make_sound();
        let fish_move = fish.move_action();
        let fish_desc = fish.describe();

        assert_eq!(
            direct_sound, fish_sound,
            "Fish reference should execute Shark's implementation"
        );
        assert_eq!(
            direct_move, fish_move,
            "Fish reference should execute Shark's implementation"
        );
        assert_eq!(
            direct_desc, fish_desc,
            "Fish reference should execute Shark's implementation"
        );

        // 通过 Animal 引用调用
        let animal: CRc<Animal> = fish.into_superclass();
        let animal_sound = animal.make_sound();
        let animal_move = animal.move_action();
        let animal_desc = animal.describe();

        println!("Polymorphic call (Animal) - Sound: {}", animal_sound);
        println!("Polymorphic call (Animal) - Move: {}", animal_move);

        assert_eq!(
            direct_sound, animal_sound,
            "Animal reference should execute Shark's implementation"
        );
        assert_eq!(
            direct_move, animal_move,
            "Animal reference should execute Shark's implementation"
        );
        assert_eq!(
            direct_desc, animal_desc,
            "Animal reference should execute Shark's implementation"
        );

        println!("✓ Shark: Polymorphic calls through Fish and Animal match direct calls");

        println!(
            "\n✓ All polymorphic method calls correctly execute concrete class implementations"
        );
    }

    #[test]
    fn prop_polymorphic_method_call_correctness() {
        println!("\n=== Property Test: Polymorphic method call correctness ===");

        // 测试所有动物类型的多态方法调用
        println!("\n--- Testing Dog ---");
        let dog = Dog::new(
            "TestDog".to_string(),
            5,
            "Labrador".to_string(),
            "Black".to_string(),
        );
        let dog_sound_direct = dog.make_sound();
        let dog_move_direct = dog.move_action();
        let dog_desc_direct = dog.describe();

        let animal: CRc<Animal> = dog.into_superclass();
        assert_eq!(
            animal.make_sound(),
            dog_sound_direct,
            "Dog: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            dog_move_direct,
            "Dog: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            dog_desc_direct,
            "Dog: polymorphic describe should match direct call"
        );

        println!("--- Testing Cat ---");
        let cat = Cat::new("TestCat".to_string(), 3, true, "Gray".to_string());
        let cat_sound_direct = cat.make_sound();
        let cat_move_direct = cat.move_action();
        let cat_desc_direct = cat.describe();

        let animal: CRc<Animal> = cat.into_superclass();
        assert_eq!(
            animal.make_sound(),
            cat_sound_direct,
            "Cat: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            cat_move_direct,
            "Cat: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            cat_desc_direct,
            "Cat: polymorphic describe should match direct call"
        );

        println!("--- Testing Eagle ---");
        let eagle = Eagle::new(
            "TestEagle".to_string(),
            7,
            2.5,
            "Brown".to_string(),
            3000.0,
            50.0,
        );
        let eagle_sound_direct = eagle.make_sound();
        let eagle_move_direct = eagle.move_action();
        let eagle_desc_direct = eagle.describe();

        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        let animal = bird.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            eagle_sound_direct,
            "Eagle: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            eagle_move_direct,
            "Eagle: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            eagle_desc_direct,
            "Eagle: polymorphic describe should match direct call"
        );

        println!("--- Testing Penguin ---");
        let penguin = Penguin::new(
            "TestPenguin".to_string(),
            4,
            0.8,
            "Black".to_string(),
            5.0,
            1000,
        );
        let penguin_sound_direct = penguin.make_sound();
        let penguin_move_direct = penguin.move_action();
        let penguin_desc_direct = penguin.describe();

        let bird = penguin.clone().into_superclass::<CRc<Bird>>();
        let animal = bird.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            penguin_sound_direct,
            "Penguin: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            penguin_move_direct,
            "Penguin: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            penguin_desc_direct,
            "Penguin: polymorphic describe should match direct call"
        );

        println!("--- Testing Duck ---");
        let duck = Duck::new(
            "TestDuck".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );
        let duck_sound_direct = duck.make_sound();
        let duck_move_direct = duck.move_action();
        let duck_desc_direct = duck.describe();

        let bird = duck.clone().into_superclass::<CRc<Bird>>();
        let animal = bird.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            duck_sound_direct,
            "Duck: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            duck_move_direct,
            "Duck: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            duck_desc_direct,
            "Duck: polymorphic describe should match direct call"
        );

        println!("--- Testing Ostrich ---");
        let ostrich = Ostrich::new("TestOstrich".to_string(), 6, 2.0, "Black".to_string(), 70.0);
        let ostrich_sound_direct = ostrich.make_sound();
        let ostrich_move_direct = ostrich.move_action();
        let ostrich_desc_direct = ostrich.describe();

        let bird = ostrich.clone().into_superclass::<CRc<Bird>>();
        let animal = bird.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            ostrich_sound_direct,
            "Ostrich: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            ostrich_move_direct,
            "Ostrich: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            ostrich_desc_direct,
            "Ostrich: polymorphic describe should match direct call"
        );

        println!("--- Testing Shark ---");
        let shark = Shark::new(
            "TestShark".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let shark_sound_direct = shark.make_sound();
        let shark_move_direct = shark.move_action();
        let shark_desc_direct = shark.describe();

        let fish = shark.clone().into_superclass::<CRc<Fish>>();
        let animal = fish.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            shark_sound_direct,
            "Shark: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            shark_move_direct,
            "Shark: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            shark_desc_direct,
            "Shark: polymorphic describe should match direct call"
        );

        println!("--- Testing Salmon ---");
        let salmon = Salmon::new(
            "TestSalmon".to_string(),
            4,
            "Freshwater".to_string(),
            "Silver".to_string(),
            "River".to_string(),
            8.0,
        );
        let salmon_sound_direct = salmon.make_sound();
        let salmon_move_direct = salmon.move_action();
        let salmon_desc_direct = salmon.describe();

        let fish = salmon.clone().into_superclass::<CRc<Fish>>();
        let animal = fish.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            salmon_sound_direct,
            "Salmon: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            salmon_move_direct,
            "Salmon: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            salmon_desc_direct,
            "Salmon: polymorphic describe should match direct call"
        );

        println!("--- Testing FlyingFish ---");
        let flying_fish = FlyingFish::new(
            "TestFlyingFish".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );
        let fish_sound_direct = flying_fish.make_sound();
        let fish_move_direct = flying_fish.move_action();
        let fish_desc_direct = flying_fish.describe();

        let fish = flying_fish.clone().into_superclass::<CRc<Fish>>();
        let animal = fish.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.make_sound(),
            fish_sound_direct,
            "FlyingFish: polymorphic make_sound should match direct call"
        );
        assert_eq!(
            animal.move_action(),
            fish_move_direct,
            "FlyingFish: polymorphic move_action should match direct call"
        );
        assert_eq!(
            animal.describe(),
            fish_desc_direct,
            "FlyingFish: polymorphic describe should match direct call"
        );

        println!(
            "\n✓ Property verified: Polymorphic method calls correctly execute concrete implementations for all 9 animal types"
        );
    }

    #[test]
    fn prop_type_distinctiveness() {
        println!("\n=== Property Test: Type distinctiveness ===");

        // 创建所有动物类型的实例
        let dog = Dog::new(
            "TestDog".to_string(),
            5,
            "Labrador".to_string(),
            "Black".to_string(),
        );
        let cat = Cat::new("TestCat".to_string(), 3, true, "Gray".to_string());
        let eagle = Eagle::new(
            "TestEagle".to_string(),
            7,
            2.5,
            "Brown".to_string(),
            3000.0,
            50.0,
        );
        let penguin = Penguin::new(
            "TestPenguin".to_string(),
            4,
            0.8,
            "Black".to_string(),
            5.0,
            1000,
        );
        let duck = Duck::new(
            "TestDuck".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );
        let ostrich = Ostrich::new("TestOstrich".to_string(), 6, 2.0, "Black".to_string(), 70.0);
        let shark = Shark::new(
            "TestShark".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let salmon = Salmon::new(
            "TestSalmon".to_string(),
            4,
            "Freshwater".to_string(),
            "Silver".to_string(),
            "River".to_string(),
            8.0,
        );
        let flying_fish = FlyingFish::new(
            "TestFlyingFish".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        // 收集所有动物的 make_sound 和 move_action 输出
        let animals_with_outputs = vec![
            ("Dog", dog.make_sound(), dog.move_action()),
            ("Cat", cat.make_sound(), cat.move_action()),
            ("Eagle", eagle.make_sound(), eagle.move_action()),
            ("Penguin", penguin.make_sound(), penguin.move_action()),
            ("Duck", duck.make_sound(), duck.move_action()),
            ("Ostrich", ostrich.make_sound(), ostrich.move_action()),
            ("Shark", shark.make_sound(), shark.move_action()),
            ("Salmon", salmon.make_sound(), salmon.move_action()),
            (
                "FlyingFish",
                flying_fish.make_sound(),
                flying_fish.move_action(),
            ),
        ];

        println!("\n--- Collected outputs from all animal types ---");
        for (name, sound, movement) in &animals_with_outputs {
            println!("{}: Sound='{}', Move='{}'", name, sound, movement);
        }

        // 验证不同类型的动物有不同的输出
        println!("\n--- Verifying type distinctiveness ---");
        let mut distinct_count = 0;
        let mut comparison_count = 0;

        for i in 0..animals_with_outputs.len() {
            for j in (i + 1)..animals_with_outputs.len() {
                let (name1, sound1, move1) = &animals_with_outputs[i];
                let (name2, sound2, move2) = &animals_with_outputs[j];

                comparison_count += 1;

                // 验证至少 make_sound 或 move_action 有一个不同
                let sound_different = sound1 != sound2;
                let move_different = move1 != move2;

                if sound_different || move_different {
                    distinct_count += 1;
                    println!(
                        "✓ {} vs {}: Sound different={}, Move different={}",
                        name1, name2, sound_different, move_different
                    );
                } else {
                    println!(
                        "✗ {} vs {}: Both sound and move are the same!",
                        name1, name2
                    );
                }

                // 断言：不同类型的动物应该有不同的行为
                assert!(
                    sound_different || move_different,
                    "{} and {} should have different make_sound or move_action outputs",
                    name1,
                    name2
                );
            }
        }

        println!(
            "\n✓ Property verified: Type distinctiveness - {}/{} comparisons showed distinct behavior",
            distinct_count, comparison_count
        );
        println!(
            "  All {} animal types have distinguishable behaviors",
            animals_with_outputs.len()
        );
    }

    #[test]
    fn test_conversion_chain_object_identity() {
        println!("\n=== Testing conversion chain object identity ===");

        // 测试 Eagle 转换链中每一步的字段值
        println!("\n--- Testing Eagle conversion chain identity ---");
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);

        let original_name = eagle.get_name().as_ref().unwrap().clone();
        let original_age = eagle.get_age();
        let original_wingspan = eagle.get_wingspan();
        let original_feather_color = eagle.get_feather_color().as_ref().unwrap().clone();
        let original_max_altitude = eagle.get_max_altitude();
        let original_hunting_territory = eagle.get_hunting_territory_size();

        println!(
            "Original Eagle - Name: {}, Age: {}, Wingspan: {}",
            original_name, original_age, original_wingspan
        );

        // Eagle -> Bird：验证字段
        let bird = eagle.clone().into_superclass::<CRc<Bird>>();
        assert_eq!(
            bird.get_name().as_ref().unwrap(),
            &original_name,
            "Step 1: Name preserved"
        );
        assert_eq!(bird.get_age(), original_age, "Step 1: Age preserved");
        assert_eq!(
            bird.get_wingspan(),
            original_wingspan,
            "Step 1: Wingspan preserved"
        );
        println!("Step 1 (Eagle->Bird): All accessible fields verified");

        // Bird -> Animal：验证字段
        let animal = bird.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.get_name().as_ref().unwrap(),
            &original_name,
            "Step 2: Name preserved"
        );
        assert_eq!(animal.get_age(), original_age, "Step 2: Age preserved");
        println!("Step 2 (Bird->Animal): All accessible fields verified");

        // Animal -> Bird：验证字段
        let bird_again = animal.try_into_subtype::<CRc<Bird>>().unwrap();
        assert_eq!(
            bird_again.get_name().as_ref().unwrap(),
            &original_name,
            "Step 3: Name preserved"
        );
        assert_eq!(bird_again.get_age(), original_age, "Step 3: Age preserved");
        assert_eq!(
            bird_again.get_wingspan(),
            original_wingspan,
            "Step 3: Wingspan preserved"
        );
        println!("Step 3 (Animal->Bird): All accessible fields verified");

        // Bird -> Eagle：验证所有字段
        let eagle_final = bird_again.try_into_subtype::<CRc<Eagle>>().unwrap();
        assert_eq!(
            eagle_final.get_name().as_ref().unwrap(),
            &original_name,
            "Step 4: Name preserved"
        );
        assert_eq!(eagle_final.get_age(), original_age, "Step 4: Age preserved");
        assert_eq!(
            eagle_final.get_wingspan(),
            original_wingspan,
            "Step 4: Wingspan preserved"
        );
        assert_eq!(
            eagle_final.get_feather_color().as_ref().unwrap(),
            &original_feather_color,
            "Step 4: Feather color preserved"
        );
        assert_eq!(
            eagle_final.get_max_altitude(),
            original_max_altitude,
            "Step 4: Max altitude preserved"
        );
        assert_eq!(
            eagle_final.get_hunting_territory_size(),
            original_hunting_territory,
            "Step 4: Hunting territory preserved"
        );
        println!("Step 4 (Bird->Eagle): All fields verified");

        println!("✓ Eagle conversion chain: Object identity maintained through all steps");

        // 测试 Shark 转换链中每一步的字段值
        println!("\n--- Testing Shark conversion chain identity ---");
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );

        let shark_name = shark.get_name().as_ref().unwrap().clone();
        let shark_age = shark.get_age();
        let shark_water_type = shark.get_water_type().as_ref().unwrap().clone();
        let shark_scale_pattern = shark.get_scale_pattern().as_ref().unwrap().clone();
        let shark_teeth_count = shark.get_teeth_count();
        let shark_swim_speed = shark.get_swim_speed();

        println!(
            "Original Shark - Name: {}, Age: {}, Teeth: {}",
            shark_name, shark_age, shark_teeth_count
        );

        // Shark -> Fish：验证字段
        let fish = shark.clone().into_superclass::<CRc<Fish>>();
        assert_eq!(
            fish.get_name().as_ref().unwrap(),
            &shark_name,
            "Step 1: Name preserved"
        );
        assert_eq!(fish.get_age(), shark_age, "Step 1: Age preserved");
        assert_eq!(
            fish.get_water_type().as_ref().unwrap(),
            &shark_water_type,
            "Step 1: Water type preserved"
        );
        assert_eq!(
            fish.get_scale_pattern().as_ref().unwrap(),
            &shark_scale_pattern,
            "Step 1: Scale pattern preserved"
        );
        println!("Step 1 (Shark->Fish): All accessible fields verified");

        // Fish -> Animal：验证字段
        let animal = fish.into_superclass::<CRc<Animal>>();
        assert_eq!(
            animal.get_name().as_ref().unwrap(),
            &shark_name,
            "Step 2: Name preserved"
        );
        assert_eq!(animal.get_age(), shark_age, "Step 2: Age preserved");
        println!("Step 2 (Fish->Animal): All accessible fields verified");

        // Animal -> Fish：验证字段
        let fish_again = animal.try_into_subtype::<CRc<Fish>>().unwrap();
        assert_eq!(
            fish_again.get_name().as_ref().unwrap(),
            &shark_name,
            "Step 3: Name preserved"
        );
        assert_eq!(fish_again.get_age(), shark_age, "Step 3: Age preserved");
        assert_eq!(
            fish_again.get_water_type().as_ref().unwrap(),
            &shark_water_type,
            "Step 3: Water type preserved"
        );
        assert_eq!(
            fish_again.get_scale_pattern().as_ref().unwrap(),
            &shark_scale_pattern,
            "Step 3: Scale pattern preserved"
        );
        println!("Step 3 (Animal->Fish): All accessible fields verified");

        // Fish -> Shark：验证所有字段
        let shark_final = fish_again.try_into_subtype::<CRc<Shark>>().unwrap();
        assert_eq!(
            shark_final.get_name().as_ref().unwrap(),
            &shark_name,
            "Step 4: Name preserved"
        );
        assert_eq!(shark_final.get_age(), shark_age, "Step 4: Age preserved");
        assert_eq!(
            shark_final.get_water_type().as_ref().unwrap(),
            &shark_water_type,
            "Step 4: Water type preserved"
        );
        assert_eq!(
            shark_final.get_scale_pattern().as_ref().unwrap(),
            &shark_scale_pattern,
            "Step 4: Scale pattern preserved"
        );
        assert_eq!(
            shark_final.get_teeth_count(),
            shark_teeth_count,
            "Step 4: Teeth count preserved"
        );
        assert_eq!(
            shark_final.get_swim_speed(),
            shark_swim_speed,
            "Step 4: Swim speed preserved"
        );
        println!("Step 4 (Fish->Shark): All fields verified");

        println!("✓ Shark conversion chain: Object identity maintained through all steps");

        // 测试更多动物类型的转换链
        println!("\n--- Testing Duck conversion chain identity ---");
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );

        let duck_name = duck.get_name().as_ref().unwrap().clone();
        let duck_age = duck.get_age();
        let duck_wingspan = duck.get_wingspan();
        let duck_migration_distance = duck.get_migration_distance();

        // Duck -> Bird -> Animal -> Bird -> Duck
        let bird = duck.clone().into_superclass::<CRc<Bird>>();
        assert_eq!(bird.get_name().as_ref().unwrap(), &duck_name);
        assert_eq!(bird.get_wingspan(), duck_wingspan);

        let animal = bird.into_superclass::<CRc<Animal>>();
        assert_eq!(animal.get_name().as_ref().unwrap(), &duck_name);
        assert_eq!(animal.get_age(), duck_age);

        let bird_again = animal.try_into_subtype::<CRc<Bird>>().unwrap();
        assert_eq!(bird_again.get_name().as_ref().unwrap(), &duck_name);
        assert_eq!(bird_again.get_wingspan(), duck_wingspan);

        let duck_final = bird_again.try_into_subtype::<CRc<Duck>>().unwrap();
        assert_eq!(duck_final.get_name().as_ref().unwrap(), &duck_name);
        assert_eq!(duck_final.get_age(), duck_age);
        assert_eq!(duck_final.get_wingspan(), duck_wingspan);
        assert_eq!(duck_final.get_migration_distance(), duck_migration_distance);
        println!("✓ Duck conversion chain: Object identity maintained");

        println!("\n--- Testing FlyingFish conversion chain identity ---");
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        let fish_name = flying_fish.get_name().as_ref().unwrap().clone();
        let fish_age = flying_fish.get_age();
        let fish_glide_distance = flying_fish.get_glide_distance();

        // FlyingFish -> Fish -> Animal -> Fish -> FlyingFish
        let fish = flying_fish.clone().into_superclass::<CRc<Fish>>();
        assert_eq!(fish.get_name().as_ref().unwrap(), &fish_name);

        let animal = fish.into_superclass::<CRc<Animal>>();
        assert_eq!(animal.get_name().as_ref().unwrap(), &fish_name);
        assert_eq!(animal.get_age(), fish_age);

        let fish_again = animal.try_into_subtype::<CRc<Fish>>().unwrap();
        assert_eq!(fish_again.get_name().as_ref().unwrap(), &fish_name);

        let fish_final = fish_again.try_into_subtype::<CRc<FlyingFish>>().unwrap();
        assert_eq!(fish_final.get_name().as_ref().unwrap(), &fish_name);
        assert_eq!(fish_final.get_age(), fish_age);
        assert_eq!(fish_final.get_glide_distance(), fish_glide_distance);
        println!("✓ FlyingFish conversion chain: Object identity maintained");

        println!(
            "\n✓ All conversion chains verified: Object identity maintained through all steps"
        );
        println!("  Tested: Eagle, Shark, Duck, FlyingFish");
        println!("  All fields preserved correctly at each conversion step");
    }

    #[test]
    fn prop_mixin_methods_callable() {
        println!("\n=== Property Test: Mixin methods callable ===");

        // 测试 Feathered mixin 方法（通过 Bird 类）
        println!("\n--- Testing Feathered mixin methods ---");
        let eagle = Eagle::new("Sky".to_string(), 7, 2.5, "Brown".to_string(), 3000.0, 50.0);
        let eagle_name = eagle.get_name().as_ref().unwrap().clone();
        let eagle_feather_color = eagle.get_feather_color().as_ref().unwrap().clone();

        // 调用 Feathered mixin 方法
        let preen_result = eagle.preen_feathers();
        println!("Eagle preen_feathers(): {}", preen_result);

        // 验证返回字符串非空且包含实例信息
        assert!(
            !preen_result.is_empty(),
            "preen_feathers() should return non-empty string"
        );
        assert!(
            preen_result.contains(&eagle_name),
            "preen_feathers() should contain animal name"
        );
        assert!(
            preen_result.contains(&eagle_feather_color),
            "preen_feathers() should contain feather color"
        );

        // 测试 Penguin 的 Feathered mixin
        let penguin = Penguin::new(
            "Pingu".to_string(),
            4,
            0.8,
            "Black and White".to_string(),
            5.0,
            1000,
        );
        let penguin_name = penguin.get_name().as_ref().unwrap().clone();
        let penguin_feather_color = penguin.get_feather_color().as_ref().unwrap().clone();

        let preen_result = penguin.preen_feathers();
        println!("Penguin preen_feathers(): {}", preen_result);

        assert!(
            !preen_result.is_empty(),
            "Penguin preen should be non-empty"
        );
        assert!(
            preen_result.contains(&penguin_name),
            "Penguin preen should contain name"
        );
        assert!(
            preen_result.contains(&penguin_feather_color),
            "Penguin preen should contain feather color"
        );

        println!("✓ Feathered mixin methods are callable and return valid results");

        // 测试 Scaled mixin 方法（通过 Fish 类）
        println!("\n--- Testing Scaled mixin methods ---");
        let shark = Shark::new(
            "Jaws".to_string(),
            10,
            "Saltwater".to_string(),
            "Gray".to_string(),
            300,
            15.0,
        );
        let shark_name = shark.get_name().as_ref().unwrap().clone();
        let shark_scale_pattern = shark.get_scale_pattern().as_ref().unwrap().clone();

        // 调用 Scaled mixin 方法
        let shed_result = shark.shed_scales();
        println!("Shark shed_scales(): {}", shed_result);

        // 验证返回字符串非空且包含实例信息
        assert!(
            !shed_result.is_empty(),
            "shed_scales() should return non-empty string"
        );
        assert!(
            shed_result.contains(&shark_name),
            "shed_scales() should contain animal name"
        );
        assert!(
            shed_result.contains(&shark_scale_pattern),
            "shed_scales() should contain scale pattern"
        );

        // 测试 Salmon 的 Scaled mixin
        let salmon = Salmon::new(
            "Sockeye".to_string(),
            4,
            "Freshwater".to_string(),
            "Silver".to_string(),
            "Alaska River".to_string(),
            8.0,
        );
        let salmon_name = salmon.get_name().as_ref().unwrap().clone();
        let salmon_scale_pattern = salmon.get_scale_pattern().as_ref().unwrap().clone();

        let shed_result = salmon.shed_scales();
        println!("Salmon shed_scales(): {}", shed_result);

        assert!(!shed_result.is_empty(), "Salmon shed should be non-empty");
        assert!(
            shed_result.contains(&salmon_name),
            "Salmon shed should contain name"
        );
        assert!(
            shed_result.contains(&salmon_scale_pattern),
            "Salmon shed should contain scale pattern"
        );

        println!("✓ Scaled mixin methods are callable and return valid results");

        // 测试 Flyable mixin 方法
        println!("\n--- Testing Flyable mixin methods ---");
        let eagle2 = Eagle::new(
            "Thunder".to_string(),
            8,
            2.8,
            "Dark Brown".to_string(),
            4000.0,
            60.0,
        );
        let eagle2_name = eagle2.get_name().as_ref().unwrap().clone();
        let eagle2_max_altitude = eagle2.get_max_altitude();

        // 调用 Flyable mixin 方法
        let fly_result = eagle2.fly();
        println!("Eagle fly(): {}", fly_result);

        // 验证返回字符串非空且包含实例信息
        assert!(
            !fly_result.is_empty(),
            "fly() should return non-empty string"
        );
        assert!(
            fly_result.contains(&eagle2_name),
            "fly() should contain animal name"
        );
        assert!(
            fly_result.contains(&eagle2_max_altitude.to_string()),
            "fly() should contain max altitude"
        );

        // 测试 Duck 的 Flyable mixin
        let duck = Duck::new(
            "Daffy".to_string(),
            2,
            1.0,
            "Brown".to_string(),
            800.0,
            8.0,
            3000.0,
        );
        let duck_name = duck.get_name().as_ref().unwrap().clone();
        let duck_max_altitude = duck.get_max_altitude();

        let fly_result = duck.fly();
        println!("Duck fly(): {}", fly_result);

        assert!(!fly_result.is_empty(), "Duck fly should be non-empty");
        assert!(
            fly_result.contains(&duck_name),
            "Duck fly should contain name"
        );
        assert!(
            fly_result.contains(&duck_max_altitude.to_string()),
            "Duck fly should contain max altitude"
        );

        // 测试 FlyingFish 的 Flyable mixin
        let flying_fish = FlyingFish::new(
            "Skipper".to_string(),
            1,
            "Saltwater".to_string(),
            "Silver".to_string(),
            400.0,
            10.0,
            40.0,
        );
        let fish_name = flying_fish.get_name().as_ref().unwrap().clone();
        let fish_max_altitude = flying_fish.get_max_altitude();

        let fly_result = flying_fish.fly();
        println!("FlyingFish fly(): {}", fly_result);

        assert!(!fly_result.is_empty(), "FlyingFish fly should be non-empty");
        assert!(
            fly_result.contains(&fish_name),
            "FlyingFish fly should contain name"
        );
        assert!(
            fly_result.contains(&fish_max_altitude.to_string()),
            "FlyingFish fly should contain max altitude"
        );

        println!("✓ Flyable mixin methods are callable and return valid results");

        // 测试 Swimmable mixin 方法
        println!("\n--- Testing Swimmable mixin methods ---");
        let penguin2 = Penguin::new(
            "Skipper".to_string(),
            5,
            0.9,
            "Black and White".to_string(),
            6.0,
            500,
        );
        let penguin2_name = penguin2.get_name().as_ref().unwrap().clone();
        let penguin2_swim_speed = penguin2.get_swim_speed();

        // 调用 Swimmable mixin 方法
        let swim_result = penguin2.swim();
        println!("Penguin swim(): {}", swim_result);

        // 验证返回字符串非空且包含实例信息
        assert!(
            !swim_result.is_empty(),
            "swim() should return non-empty string"
        );
        assert!(
            swim_result.contains(&penguin2_name),
            "swim() should contain animal name"
        );
        assert!(
            swim_result.contains(&penguin2_swim_speed.to_string()),
            "swim() should contain swim speed"
        );

        // 测试 Duck 的 Swimmable mixin
        let duck2 = Duck::new(
            "Quackers".to_string(),
            3,
            1.1,
            "White".to_string(),
            900.0,
            9.0,
            4000.0,
        );
        let duck2_name = duck2.get_name().as_ref().unwrap().clone();
        let duck2_swim_speed = duck2.get_swim_speed();

        let swim_result = duck2.swim();
        println!("Duck swim(): {}", swim_result);

        assert!(!swim_result.is_empty(), "Duck swim should be non-empty");
        assert!(
            swim_result.contains(&duck2_name),
            "Duck swim should contain name"
        );
        assert!(
            swim_result.contains(&duck2_swim_speed.to_string()),
            "Duck swim should contain swim speed"
        );

        // 测试 Shark 的 Swimmable mixin
        let shark2 = Shark::new(
            "Bruce".to_string(),
            12,
            "Saltwater".to_string(),
            "Blue-Gray".to_string(),
            350,
            18.0,
        );
        let shark2_name = shark2.get_name().as_ref().unwrap().clone();
        let shark2_swim_speed = shark2.get_swim_speed();

        let swim_result = shark2.swim();
        println!("Shark swim(): {}", swim_result);

        assert!(!swim_result.is_empty(), "Shark swim should be non-empty");
        assert!(
            swim_result.contains(&shark2_name),
            "Shark swim should contain name"
        );
        assert!(
            swim_result.contains(&shark2_swim_speed.to_string()),
            "Shark swim should contain swim speed"
        );

        // 测试 Salmon 的 Swimmable mixin
        let salmon2 = Salmon::new(
            "Chinook".to_string(),
            3,
            "Freshwater".to_string(),
            "Red".to_string(),
            "Columbia River".to_string(),
            7.0,
        );
        let salmon2_name = salmon2.get_name().as_ref().unwrap().clone();
        let salmon2_swim_speed = salmon2.get_swim_speed();

        let swim_result = salmon2.swim();
        println!("Salmon swim(): {}", swim_result);

        assert!(!swim_result.is_empty(), "Salmon swim should be non-empty");
        assert!(
            swim_result.contains(&salmon2_name),
            "Salmon swim should contain name"
        );
        assert!(
            swim_result.contains(&salmon2_swim_speed.to_string()),
            "Salmon swim should contain swim speed"
        );

        // 测试 FlyingFish 的 Swimmable mixin
        let flying_fish2 = FlyingFish::new(
            "Jumper".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            450.0,
            11.0,
            45.0,
        );
        let fish2_name = flying_fish2.get_name().as_ref().unwrap().clone();
        let fish2_swim_speed = flying_fish2.get_swim_speed();

        let swim_result = flying_fish2.swim();
        println!("FlyingFish swim(): {}", swim_result);

        assert!(
            !swim_result.is_empty(),
            "FlyingFish swim should be non-empty"
        );
        assert!(
            swim_result.contains(&fish2_name),
            "FlyingFish swim should contain name"
        );
        assert!(
            swim_result.contains(&fish2_swim_speed.to_string()),
            "FlyingFish swim should contain swim speed"
        );

        println!("✓ Swimmable mixin methods are callable and return valid results");

        println!("\n✓ Property verified: All mixin methods are callable and return valid results");
        println!("  Tested mixins: Feathered, Scaled, Flyable, Swimmable");
        println!("  All methods returned non-empty strings containing instance information");
    }

    #[test]
    fn prop_multiple_mixin_method_independence() {
        println!("\n=== Property Test: Multiple mixin method independence ===");

        // 测试 Duck（使用 Flyable 和 Swimmable）
        println!("\n--- Testing Duck with Flyable and Swimmable mixins ---");
        let duck = Duck::new(
            "Donald".to_string(),
            3,
            1.2,
            "White".to_string(),
            1000.0,
            10.0,
            5000.0,
        );

        let duck_name = duck.get_name().as_ref().unwrap().clone();
        let duck_max_altitude = duck.get_max_altitude();
        let duck_swim_speed = duck.get_swim_speed();

        println!(
            "Duck: {}, Max Altitude: {}, Swim Speed: {}",
            duck_name, duck_max_altitude, duck_swim_speed
        );

        // 调用 Flyable mixin 方法
        let fly_result = duck.fly();
        println!("Duck fly(): {}", fly_result);

        // 验证 fly() 方法返回有效结果
        assert!(
            !fly_result.is_empty(),
            "Duck fly() should return non-empty string"
        );
        assert!(
            fly_result.contains(&duck_name),
            "Duck fly() should contain duck name"
        );
        assert!(
            fly_result.contains(&duck_max_altitude.to_string()),
            "Duck fly() should contain max altitude"
        );

        // 调用 Swimmable mixin 方法
        let swim_result = duck.swim();
        println!("Duck swim(): {}", swim_result);

        // 验证 swim() 方法返回有效结果
        assert!(
            !swim_result.is_empty(),
            "Duck swim() should return non-empty string"
        );
        assert!(
            swim_result.contains(&duck_name),
            "Duck swim() should contain duck name"
        );
        assert!(
            swim_result.contains(&duck_swim_speed.to_string()),
            "Duck swim() should contain swim speed"
        );

        // 验证两个方法可以独立调用且都返回有效结果
        println!(
            "✓ Duck: Both Flyable and Swimmable methods are callable and return valid results"
        );

        // 测试 FlyingFish（使用 Flyable 和 Swimmable）
        println!("\n--- Testing FlyingFish with Flyable and Swimmable mixins ---");
        let flying_fish = FlyingFish::new(
            "Glider".to_string(),
            2,
            "Saltwater".to_string(),
            "Blue".to_string(),
            500.0,
            12.0,
            50.0,
        );

        let fish_name = flying_fish.get_name().as_ref().unwrap().clone();
        let fish_max_altitude = flying_fish.get_max_altitude();
        let fish_swim_speed = flying_fish.get_swim_speed();

        println!(
            "FlyingFish: {}, Max Altitude: {}, Swim Speed: {}",
            fish_name, fish_max_altitude, fish_swim_speed
        );

        // 调用 Flyable mixin 方法
        let fly_result = flying_fish.fly();
        println!("FlyingFish fly(): {}", fly_result);

        // 验证 fly() 方法返回有效结果
        assert!(
            !fly_result.is_empty(),
            "FlyingFish fly() should return non-empty string"
        );
        assert!(
            fly_result.contains(&fish_name),
            "FlyingFish fly() should contain fish name"
        );
        assert!(
            fly_result.contains(&fish_max_altitude.to_string()),
            "FlyingFish fly() should contain max altitude"
        );

        // 调用 Swimmable mixin 方法
        let swim_result = flying_fish.swim();
        println!("FlyingFish swim(): {}", swim_result);

        // 验证 swim() 方法返回有效结果
        assert!(
            !swim_result.is_empty(),
            "FlyingFish swim() should return non-empty string"
        );
        assert!(
            swim_result.contains(&fish_name),
            "FlyingFish swim() should contain fish name"
        );
        assert!(
            swim_result.contains(&fish_swim_speed.to_string()),
            "FlyingFish swim() should contain swim speed"
        );

        // 验证两个方法可以独立调用且都返回有效结果
        println!(
            "✓ FlyingFish: Both Flyable and Swimmable methods are callable and return valid results"
        );

        // 测试多次调用的独立性
        println!("\n--- Testing method call independence ---");

        // 多次调用 Duck 的方法，验证每次都返回有效结果
        let duck2 = Duck::new(
            "Daffy".to_string(),
            4,
            1.3,
            "Brown".to_string(),
            1200.0,
            11.0,
            6000.0,
        );

        let fly1 = duck2.fly();
        let swim1 = duck2.swim();
        let fly2 = duck2.fly();
        let swim2 = duck2.swim();

        // 验证多次调用都返回有效结果
        assert!(!fly1.is_empty(), "First fly() call should be valid");
        assert!(!swim1.is_empty(), "First swim() call should be valid");
        assert!(!fly2.is_empty(), "Second fly() call should be valid");
        assert!(!swim2.is_empty(), "Second swim() call should be valid");

        // 验证多次调用返回相同的结果（方法是确定性的）
        assert_eq!(
            fly1, fly2,
            "Multiple fly() calls should return the same result"
        );
        assert_eq!(
            swim1, swim2,
            "Multiple swim() calls should return the same result"
        );

        println!("✓ Multiple calls to the same mixin method return consistent results");

        // 测试交替调用的独立性
        println!("\n--- Testing alternating method calls ---");

        let flying_fish2 = FlyingFish::new(
            "Skipper".to_string(),
            3,
            "Saltwater".to_string(),
            "Silver".to_string(),
            600.0,
            13.0,
            55.0,
        );

        // 交替调用 fly() 和 swim()
        let result1 = flying_fish2.fly();
        let result2 = flying_fish2.swim();
        let result3 = flying_fish2.fly();
        let result4 = flying_fish2.swim();

        // 验证所有调用都返回有效结果
        assert!(
            !result1.is_empty(),
            "Alternating call 1 (fly) should be valid"
        );
        assert!(
            !result2.is_empty(),
            "Alternating call 2 (swim) should be valid"
        );
        assert!(
            !result3.is_empty(),
            "Alternating call 3 (fly) should be valid"
        );
        assert!(
            !result4.is_empty(),
            "Alternating call 4 (swim) should be valid"
        );

        // 验证相同方法的调用返回相同结果
        assert_eq!(
            result1, result3,
            "Alternating fly() calls should return the same result"
        );
        assert_eq!(
            result2, result4,
            "Alternating swim() calls should return the same result"
        );

        println!("✓ Alternating calls to different mixin methods work independently");

        println!("\n✓ Property verified: Multiple mixin methods work independently");
        println!("  Tested classes: Duck, FlyingFish");
        println!("  All mixin methods (Flyable and Swimmable) are callable");
        println!("  Methods return valid results independently");
        println!("  Multiple calls to the same method return consistent results");
    }
}
