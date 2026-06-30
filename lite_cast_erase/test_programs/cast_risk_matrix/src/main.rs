use oop_rs::prelude::*;
use std::collections::{BTreeMap, HashMap};

#[class(extends(Object))]
type Animal = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(extends(Animal))]
type Dog = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(extends(Animal))]
type Cat = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(extends(Animal))]
type Bird = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(extends(Bird))]
type Eagle = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(implements(Downcast))]
type Runnable = interface<{
    pub fn run(&self);
}>;

#[class(implements(Runnable))]
type RunnerDog = class<{
    pub fn new() -> Self {
        Self {}
    }

    #[method(override(Runnable))]
    pub fn run(&self) {}
}>;

#[class(implements(Runnable))]
type RunnerCat = class<{
    pub fn new() -> Self {
        Self {}
    }

    #[method(override(Runnable))]
    pub fn run(&self) {}
}>;

#[class(on(Animal))]
type Tagged = mixin<{
    pub fn tag(&self) -> usize {
        1
    }
}>;

#[class(extends(Animal), with(Tagged))]
type TaggedDog = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class(extends(Animal), with(Tagged))]
type TaggedCat = class<{
    pub fn new() -> Self {
        Self { ..Super::new() }
    }
}>;

#[class]
type AnimalHolder = class<{
    #[late]
    let animal: CRc<Animal>;

    pub fn new() -> Self {
        Self {}
    }

    pub fn stored_animal(&self) -> CRc<Animal> {
        self.get().animal()
    }
}>;

fn dog_as_animal() -> CRc<Animal> {
    Dog::new() as CRc<Animal>
}

fn cat_as_animal() -> CRc<Animal> {
    Cat::new() as CRc<Animal>
}

fn choose_animal(flag: bool) -> CRc<Animal> {
    if flag {
        dog_as_animal()
    } else {
        cat_as_animal()
    }
}

fn unwrap_dog() -> CRc<Animal> {
    let animal = dog_as_animal();
    animal.downcast_rc::<Dog>().unwrap() as CRc<Animal>
}

fn passthrough_animal(animal: CRc<Animal>) -> CRc<Animal> {
    animal
}

fn store_dog_in_holder(holder: CRc<AnimalHolder>) {
    holder.set().animal(dog_as_animal());
}

fn store_chosen_animal_in_holder(holder: CRc<AnimalHolder>, flag: bool) {
    holder.set().animal(choose_animal(flag));
}

fn load_animal_from_holder(holder: CRc<AnimalHolder>) -> CRc<Animal> {
    holder.get().animal()
}

fn make_dog_holder_vec() -> Vec<CRc<AnimalHolder>> {
    let holder = AnimalHolder::new();
    holder.set().animal(dog_as_animal());
    vec![holder]
}

fn make_chosen_holder_vec(flag: bool) -> Vec<CRc<AnimalHolder>> {
    let dog_holder = AnimalHolder::new();
    let cat_holder = AnimalHolder::new();
    dog_holder.set().animal(dog_as_animal());
    cat_holder.set().animal(cat_as_animal());
    if flag {
        vec![dog_holder]
    } else {
        vec![cat_holder]
    }
}

fn make_dog_holder_option() -> Option<CRc<AnimalHolder>> {
    let holder = AnimalHolder::new();
    holder.set().animal(dog_as_animal());
    Some(holder)
}

fn make_chosen_holder_option(flag: bool) -> Option<CRc<AnimalHolder>> {
    let holder = if flag {
        let holder = AnimalHolder::new();
        holder.set().animal(dog_as_animal());
        holder
    } else {
        let holder = AnimalHolder::new();
        holder.set().animal(cat_as_animal());
        holder
    };
    Some(holder)
}

fn runner_dog_view() -> CRc<Runnable> {
    RunnerDog::new()
}

fn choose_runner_view(flag: bool) -> CRc<Runnable> {
    if flag {
        RunnerDog::new()
    } else {
        RunnerCat::new()
    }
}

fn tagged_dog_as_animal() -> CRc<Animal> {
    TaggedDog::new() as CRc<Animal>
}

fn choose_tagged_or_plain_animal(flag: bool) -> CRc<Animal> {
    if flag {
        TaggedDog::new() as CRc<Animal>
    } else {
        Cat::new() as CRc<Animal>
    }
}

fn dog_result() -> Result<CRc<Animal>, &'static str> {
    Ok(dog_as_animal())
}

fn choose_animal_result(flag: bool) -> Result<CRc<Animal>, &'static str> {
    Ok(choose_animal(flag))
}

fn dog_option_result() -> Option<Result<CRc<Animal>, &'static str>> {
    Some(Ok(dog_as_animal()))
}

fn choose_animal_option_result(flag: bool) -> Option<Result<CRc<Animal>, &'static str>> {
    Some(Ok(choose_animal(flag)))
}

fn dog_result_option() -> Result<Option<CRc<Animal>>, &'static str> {
    Ok(Some(dog_as_animal()))
}

fn choose_animal_result_option(flag: bool) -> Result<Option<CRc<Animal>>, &'static str> {
    Ok(Some(choose_animal(flag)))
}

fn dog_option() -> Option<CRc<Animal>> {
    Some(dog_as_animal())
}

fn choose_animal_option(flag: bool) -> Option<CRc<Animal>> {
    Some(choose_animal(flag))
}

pub fn proven_safe_local_downcast() {
    let animal: CRc<Animal> = Dog::new() as CRc<Animal>;
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn proven_safe_helper_return_downcast() {
    let animal = dog_as_animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn proven_safe_chained_result_unwrap_downcast() {
    let animal: CRc<Animal> = Eagle::new() as CRc<Bird> as CRc<Animal>;
    let bird = animal.downcast_rc::<Bird>().unwrap();
    let eagle = bird.downcast_rc::<Eagle>().unwrap();
    let _ = eagle;
}

pub fn proven_safe_interface_downcast_ref() {
    let runner: CRc<Runnable> = RunnerDog::new();
    runner.run();
    let dog: &RunnerDog = runner.downcast_ref().unwrap();
    let _ = dog;
}

pub fn proven_safe_mixin_downcast_ref() {
    let animal: CRc<Animal> = TaggedDog::new() as CRc<Animal>;
    let tagged: &Tagged = animal.downcast_ref().unwrap();
    let _ = tagged.tag();
}

pub fn must_unsafe_sibling_downcast() {
    let animal: CRc<Animal> = Dog::new() as CRc<Animal>;
    let result = animal.downcast_rc::<Cat>();
    assert!(result.is_err());
}

pub fn must_unsafe_interface_wrong_concrete_downcast_ref() {
    let runner: CRc<Runnable> = RunnerDog::new();
    let result: Result<&Cat, _> = runner.downcast_ref();
    assert!(result.is_err());
}

pub fn may_unsafe_branch_join_downcast(flag: bool) {
    let animal = choose_animal(flag);
    let _ = animal.downcast_rc::<Dog>();
}

pub fn may_unsafe_local_branch_join_downcast(flag: bool) {
    let animal: CRc<Animal> = if flag {
        Dog::new() as CRc<Animal>
    } else {
        Cat::new() as CRc<Animal>
    };
    let _ = animal.downcast_rc::<Dog>();
}

pub fn unknown_should_fix_vec_element_downcast() {
    let animals: Vec<CRc<Animal>> = vec![Dog::new() as CRc<Animal>];
    let animal = animals[0].clone();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn unknown_should_fix_option_unwrap_source_downcast() {
    let animal = Some(dog_as_animal()).unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn proven_safe_field_store_load_downcast() {
    let holder = AnimalHolder::new();
    holder.set().animal(dog_as_animal());
    let animal = holder.get().animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_field_branch_store_load_downcast(flag: bool) {
    let holder = AnimalHolder::new();
    if flag {
        holder.set().animal(dog_as_animal());
    } else {
        holder.set().animal(cat_as_animal());
    }
    let animal = holder.get().animal();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn must_unsafe_field_store_load_downcast() {
    let holder = AnimalHolder::new();
    holder.set().animal(cat_as_animal());
    let animal = holder.get().animal();
    let result = animal.downcast_rc::<Dog>();
    assert!(result.is_err());
}

pub fn proven_safe_two_holder_field_precision_downcast() {
    let dog_holder = AnimalHolder::new();
    let cat_holder = AnimalHolder::new();
    dog_holder.set().animal(dog_as_animal());
    cat_holder.set().animal(cat_as_animal());
    let animal = dog_holder.get().animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_field_overwrite_flow_insensitive_downcast() {
    let holder = AnimalHolder::new();
    holder.set().animal(cat_as_animal());
    holder.set().animal(dog_as_animal());
    let animal = holder.get().animal();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_field_helper_store_load_downcast() {
    let holder = AnimalHolder::new();
    store_dog_in_holder(holder.clone());
    let animal = load_animal_from_holder(holder);
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_field_helper_branch_store_load_downcast(flag: bool) {
    let holder = AnimalHolder::new();
    store_chosen_animal_in_holder(holder.clone(), flag);
    let animal = load_animal_from_holder(holder);
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_method_return_field_downcast() {
    let holder = AnimalHolder::new();
    holder.set().animal(dog_as_animal());
    let animal = holder.stored_animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_method_return_field_downcast(flag: bool) {
    let holder = AnimalHolder::new();
    holder.set().animal(choose_animal(flag));
    let animal = holder.stored_animal();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_function_arg_passthrough_downcast() {
    let animal = passthrough_animal(dog_as_animal());
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_function_arg_passthrough_downcast(flag: bool) {
    let animal = passthrough_animal(choose_animal(flag));
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_closure_arg_passthrough_downcast() {
    let passthrough = |animal: CRc<Animal>| animal;
    let animal = passthrough(dog_as_animal());
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_closure_arg_passthrough_downcast(flag: bool) {
    let passthrough = |animal: CRc<Animal>| animal;
    let animal = passthrough(choose_animal(flag));
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_vec_holder_field_downcast() {
    let holder = AnimalHolder::new();
    holder.set().animal(dog_as_animal());
    let holders: Vec<CRc<AnimalHolder>> = vec![holder];
    let animal = holders[0].get().animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_vec_holder_field_downcast(flag: bool) {
    let dog_holder = AnimalHolder::new();
    let cat_holder = AnimalHolder::new();
    dog_holder.set().animal(dog_as_animal());
    cat_holder.set().animal(cat_as_animal());
    let holders: Vec<CRc<AnimalHolder>> = if flag {
        vec![dog_holder]
    } else {
        vec![cat_holder]
    };
    let animal = holders[0].get().animal();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_option_holder_field_downcast() {
    let holder = AnimalHolder::new();
    holder.set().animal(dog_as_animal());
    let holder = Some(holder).unwrap();
    let animal = holder.get().animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_option_holder_field_downcast(flag: bool) {
    let holder = if flag {
        let holder = AnimalHolder::new();
        holder.set().animal(dog_as_animal());
        holder
    } else {
        let holder = AnimalHolder::new();
        holder.set().animal(cat_as_animal());
        holder
    };
    let holder = Some(holder).unwrap();
    let animal = holder.get().animal();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_helper_return_vec_holder_field_downcast() {
    let holders = make_dog_holder_vec();
    let animal = holders[0].get().animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_helper_return_vec_holder_field_downcast(flag: bool) {
    let holders = make_chosen_holder_vec(flag);
    let animal = holders[0].get().animal();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_helper_return_option_holder_field_downcast() {
    let holder = make_dog_holder_option().unwrap();
    let animal = holder.get().animal();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_helper_return_option_holder_field_downcast(flag: bool) {
    let holder = make_chosen_holder_option(flag).unwrap();
    let animal = holder.get().animal();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_helper_interface_view_downcast_ref() {
    let runner = runner_dog_view();
    let dog: &RunnerDog = runner.downcast_ref().unwrap();
    let _ = dog;
}

pub fn may_unsafe_helper_interface_view_downcast_ref(flag: bool) {
    let runner = choose_runner_view(flag);
    let _ = runner.downcast_ref::<RunnerDog>();
}

pub fn proven_safe_vec_interface_view_downcast_ref() {
    let runners: Vec<CRc<Runnable>> = vec![runner_dog_view()];
    let runner = runners[0].clone();
    let dog: &RunnerDog = runner.downcast_ref().unwrap();
    let _ = dog;
}

pub fn may_unsafe_vec_interface_view_downcast_ref(flag: bool) {
    let runners: Vec<CRc<Runnable>> = if flag {
        vec![RunnerDog::new()]
    } else {
        vec![RunnerCat::new()]
    };
    let runner = runners[0].clone();
    let _ = runner.downcast_ref::<RunnerDog>();
}

pub fn proven_safe_helper_mixin_view_downcast_ref() {
    let animal = tagged_dog_as_animal();
    let tagged: &Tagged = animal.downcast_ref().unwrap();
    let _ = tagged.tag();
}

pub fn may_unsafe_helper_mixin_view_downcast_ref(flag: bool) {
    let animal = choose_tagged_or_plain_animal(flag);
    let _ = animal.downcast_ref::<Tagged>();
}

pub fn proven_safe_vec_mixin_view_downcast_ref() {
    let animals: Vec<CRc<Animal>> = vec![tagged_dog_as_animal()];
    let animal = animals[0].clone();
    let tagged: &Tagged = animal.downcast_ref().unwrap();
    let _ = tagged.tag();
}

pub fn may_unsafe_vec_mixin_view_downcast_ref(flag: bool) {
    let animals: Vec<CRc<Animal>> = if flag {
        vec![TaggedDog::new() as CRc<Animal>]
    } else {
        vec![Cat::new() as CRc<Animal>]
    };
    let animal = animals[0].clone();
    let _ = animal.downcast_ref::<Tagged>();
}

pub fn proven_safe_helper_result_unwrap_source_downcast() {
    let animal = dog_result().unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_helper_result_unwrap_source_downcast(flag: bool) {
    let animal = choose_animal_result(flag).unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_option_result_double_unwrap_downcast() {
    let animal = dog_option_result().unwrap().unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_option_result_double_unwrap_downcast(flag: bool) {
    let animal = choose_animal_option_result(flag).unwrap().unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_result_option_double_unwrap_downcast() {
    let animal = dog_result_option().unwrap().unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_result_option_double_unwrap_downcast(flag: bool) {
    let animal = choose_animal_result_option(flag).unwrap().unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_option_ok_or_unwrap_downcast() {
    let animal = dog_option().ok_or("missing animal").unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_option_ok_or_unwrap_downcast(flag: bool) {
    let animal = choose_animal_option(flag).ok_or("missing animal").unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_option_map_passthrough_downcast() {
    let animal = dog_option().map(|animal| animal).unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_option_map_passthrough_downcast(flag: bool) {
    let animal = choose_animal_option(flag).map(|animal| animal).unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_result_map_passthrough_downcast() {
    let animal = dog_result().map(|animal| animal).unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_result_map_passthrough_downcast(flag: bool) {
    let animal = choose_animal_result(flag).map(|animal| animal).unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_option_and_then_passthrough_downcast() {
    let animal = dog_option().and_then(|animal| Some(animal)).unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_option_and_then_passthrough_downcast(flag: bool) {
    let animal = choose_animal_option(flag)
        .and_then(|animal| Some(animal))
        .unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_result_and_then_passthrough_downcast() {
    let animal = dog_result().and_then(|animal| Ok(animal)).unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_result_and_then_passthrough_downcast(flag: bool) {
    let animal = choose_animal_result(flag)
        .and_then(|animal| Ok(animal))
        .unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_option_fallback_combinators_downcast() {
    let missing: Option<CRc<Animal>> = None;
    let animal = missing
        .or(Some(dog_as_animal()))
        .or_else(|| dog_option())
        .unwrap_or_else(|| dog_as_animal());
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_option_fallback_combinators_downcast(flag: bool) {
    let missing: Option<CRc<Animal>> = None;
    let animal = missing
        .or(Some(choose_animal(flag)))
        .or_else(|| choose_animal_option(flag))
        .unwrap_or_else(|| cat_as_animal());
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_result_fallback_combinators_downcast() {
    let missing: Result<CRc<Animal>, &'static str> = Err("missing animal");
    let animal = missing
        .or(Ok::<CRc<Animal>, &'static str>(dog_as_animal()))
        .or_else(|_| dog_result())
        .unwrap_or_else(|_| dog_as_animal());
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_result_fallback_combinators_downcast(flag: bool) {
    let missing: Result<CRc<Animal>, &'static str> = Err("missing animal");
    let animal = missing
        .or(Ok::<CRc<Animal>, &'static str>(choose_animal(flag)))
        .or_else(|_| choose_animal_result(flag))
        .unwrap_or_else(|_| cat_as_animal());
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_vec_iter_next_downcast() {
    let animals: Vec<CRc<Animal>> = vec![dog_as_animal()];
    let animal = animals.iter().next().unwrap().clone();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_vec_iter_next_downcast(flag: bool) {
    let animals: Vec<CRc<Animal>> = if flag {
        vec![dog_as_animal()]
    } else {
        vec![cat_as_animal()]
    };
    let animal = animals.iter().next().unwrap().clone();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_vec_into_iter_next_downcast() {
    let animals: Vec<CRc<Animal>> = vec![dog_as_animal()];
    let animal = animals.into_iter().next().unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_vec_into_iter_next_downcast(flag: bool) {
    let animals: Vec<CRc<Animal>> = if flag {
        vec![dog_as_animal()]
    } else {
        vec![cat_as_animal()]
    };
    let animal = animals.into_iter().next().unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_vec_iter_map_next_downcast() {
    let animals: Vec<CRc<Animal>> = vec![dog_as_animal()];
    let animal = animals.iter().map(|animal| animal.clone()).next().unwrap();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_vec_iter_map_next_downcast(flag: bool) {
    let animals: Vec<CRc<Animal>> = if flag {
        vec![dog_as_animal()]
    } else {
        vec![cat_as_animal()]
    };
    let animal = animals.iter().map(|animal| animal.clone()).next().unwrap();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_vec_iter_find_downcast() {
    let animals: Vec<CRc<Animal>> = vec![dog_as_animal()];
    let animal = animals.iter().find(|_| true).unwrap().clone();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_vec_iter_find_downcast(flag: bool) {
    let animals: Vec<CRc<Animal>> = if flag {
        vec![dog_as_animal()]
    } else {
        vec![cat_as_animal()]
    };
    let animal = animals.iter().find(|_| true).unwrap().clone();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_vec_into_iter_collect_downcast() {
    let animals: Vec<CRc<Animal>> = vec![dog_as_animal()];
    let collected: Vec<CRc<Animal>> = animals.into_iter().collect();
    let animal = collected[0].clone();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_vec_into_iter_collect_downcast(flag: bool) {
    let animals: Vec<CRc<Animal>> = if flag {
        vec![dog_as_animal()]
    } else {
        vec![cat_as_animal()]
    };
    let collected: Vec<CRc<Animal>> = animals.into_iter().collect();
    let animal = collected[0].clone();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_hashmap_get_downcast() {
    let mut animals: HashMap<&'static str, CRc<Animal>> = HashMap::new();
    animals.insert("pet", dog_as_animal());
    let animal = animals.get("pet").unwrap().clone();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_hashmap_get_downcast(flag: bool) {
    let mut animals: HashMap<&'static str, CRc<Animal>> = HashMap::new();
    animals.insert("pet", choose_animal(flag));
    let animal = animals.get("pet").unwrap().clone();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn must_unsafe_hashmap_get_downcast() {
    let mut animals: HashMap<&'static str, CRc<Animal>> = HashMap::new();
    animals.insert("pet", cat_as_animal());
    let animal = animals.get("pet").unwrap().clone();
    let result = animal.downcast_rc::<Dog>();
    assert!(result.is_err());
}

pub fn proven_safe_hashmap_values_next_downcast() {
    let mut animals: HashMap<&'static str, CRc<Animal>> = HashMap::new();
    animals.insert("pet", dog_as_animal());
    let animal = animals.values().next().unwrap().clone();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_hashmap_values_next_downcast(flag: bool) {
    let mut animals: HashMap<&'static str, CRc<Animal>> = HashMap::new();
    animals.insert("pet", choose_animal(flag));
    let animal = animals.values().next().unwrap().clone();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn proven_safe_btreemap_get_downcast() {
    let mut animals: BTreeMap<&'static str, CRc<Animal>> = BTreeMap::new();
    animals.insert("pet", dog_as_animal());
    let animal = animals.get("pet").unwrap().clone();
    let dog = animal.downcast_rc::<Dog>().unwrap();
    let _ = dog;
}

pub fn may_unsafe_btreemap_get_downcast(flag: bool) {
    let mut animals: BTreeMap<&'static str, CRc<Animal>> = BTreeMap::new();
    animals.insert("pet", choose_animal(flag));
    let animal = animals.get("pet").unwrap().clone();
    let _ = animal.downcast_rc::<Dog>();
}

pub fn must_unsafe_btreemap_get_downcast() {
    let mut animals: BTreeMap<&'static str, CRc<Animal>> = BTreeMap::new();
    animals.insert("pet", cat_as_animal());
    let animal = animals.get("pet").unwrap().clone();
    let result = animal.downcast_rc::<Dog>();
    assert!(result.is_err());
}

fn main() {
    proven_safe_local_downcast();
    proven_safe_helper_return_downcast();
    proven_safe_chained_result_unwrap_downcast();
    proven_safe_interface_downcast_ref();
    proven_safe_mixin_downcast_ref();
    must_unsafe_sibling_downcast();
    must_unsafe_interface_wrong_concrete_downcast_ref();
    may_unsafe_branch_join_downcast(true);
    may_unsafe_local_branch_join_downcast(true);
    unknown_should_fix_vec_element_downcast();
    unknown_should_fix_option_unwrap_source_downcast();
    proven_safe_field_store_load_downcast();
    may_unsafe_field_branch_store_load_downcast(true);
    must_unsafe_field_store_load_downcast();
    proven_safe_two_holder_field_precision_downcast();
    may_unsafe_field_overwrite_flow_insensitive_downcast();
    proven_safe_field_helper_store_load_downcast();
    may_unsafe_field_helper_branch_store_load_downcast(true);
    proven_safe_method_return_field_downcast();
    may_unsafe_method_return_field_downcast(true);
    proven_safe_function_arg_passthrough_downcast();
    may_unsafe_function_arg_passthrough_downcast(true);
    proven_safe_closure_arg_passthrough_downcast();
    may_unsafe_closure_arg_passthrough_downcast(true);
    proven_safe_vec_holder_field_downcast();
    may_unsafe_vec_holder_field_downcast(true);
    proven_safe_option_holder_field_downcast();
    may_unsafe_option_holder_field_downcast(true);
    proven_safe_helper_return_vec_holder_field_downcast();
    may_unsafe_helper_return_vec_holder_field_downcast(true);
    proven_safe_helper_return_option_holder_field_downcast();
    may_unsafe_helper_return_option_holder_field_downcast(true);
    proven_safe_helper_interface_view_downcast_ref();
    may_unsafe_helper_interface_view_downcast_ref(true);
    proven_safe_vec_interface_view_downcast_ref();
    may_unsafe_vec_interface_view_downcast_ref(true);
    proven_safe_helper_mixin_view_downcast_ref();
    may_unsafe_helper_mixin_view_downcast_ref(true);
    proven_safe_vec_mixin_view_downcast_ref();
    may_unsafe_vec_mixin_view_downcast_ref(true);
    proven_safe_helper_result_unwrap_source_downcast();
    may_unsafe_helper_result_unwrap_source_downcast(true);
    proven_safe_option_result_double_unwrap_downcast();
    may_unsafe_option_result_double_unwrap_downcast(true);
    proven_safe_result_option_double_unwrap_downcast();
    may_unsafe_result_option_double_unwrap_downcast(true);
    proven_safe_option_ok_or_unwrap_downcast();
    may_unsafe_option_ok_or_unwrap_downcast(true);
    proven_safe_option_map_passthrough_downcast();
    may_unsafe_option_map_passthrough_downcast(true);
    proven_safe_result_map_passthrough_downcast();
    may_unsafe_result_map_passthrough_downcast(true);
    proven_safe_option_and_then_passthrough_downcast();
    may_unsafe_option_and_then_passthrough_downcast(true);
    proven_safe_result_and_then_passthrough_downcast();
    may_unsafe_result_and_then_passthrough_downcast(true);
    proven_safe_option_fallback_combinators_downcast();
    may_unsafe_option_fallback_combinators_downcast(true);
    proven_safe_result_fallback_combinators_downcast();
    may_unsafe_result_fallback_combinators_downcast(true);
    proven_safe_vec_iter_next_downcast();
    may_unsafe_vec_iter_next_downcast(true);
    proven_safe_vec_into_iter_next_downcast();
    may_unsafe_vec_into_iter_next_downcast(true);
    proven_safe_vec_iter_map_next_downcast();
    may_unsafe_vec_iter_map_next_downcast(true);
    proven_safe_vec_iter_find_downcast();
    may_unsafe_vec_iter_find_downcast(true);
    proven_safe_vec_into_iter_collect_downcast();
    may_unsafe_vec_into_iter_collect_downcast(true);
    proven_safe_hashmap_get_downcast();
    may_unsafe_hashmap_get_downcast(true);
    must_unsafe_hashmap_get_downcast();
    proven_safe_hashmap_values_next_downcast();
    may_unsafe_hashmap_values_next_downcast(true);
    proven_safe_btreemap_get_downcast();
    may_unsafe_btreemap_get_downcast(true);
    must_unsafe_btreemap_get_downcast();
}
