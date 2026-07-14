# min_class_downcast analysis

## 1. Test program

```rust
#[class(extends(Object))]
type Animal = class<{
    pub fn new() -> Self { Self { ..Super::new() } }
}>;

#[class(extends(Animal))]
type Dog = class<{
    pub fn new() -> Self { Self { ..Super::new() } }
}>;

#[class(extends(Animal))]
type Cat = class<{
    pub fn new() -> Self { Self { ..Super::new() } }
}>;

fn must_succeed_downcast(animal: CRc<Animal>) -> bool {
    animal.downcast_rc::<Dog>().is_ok()
}

fn must_fail_downcast(animal: CRc<Animal>) -> bool {
    animal.downcast_rc::<Cat>().is_ok()
}

fn main() {
    let animal: CRc<Animal> = Dog::new();
    assert!(must_succeed_downcast(animal.clone()));
    assert!(!must_fail_downcast(animal));
}
```

## 2. Expanded shape

Observed from `/tmp/min_class_downcast.expanded.rs`.

- `Animal`, `Dog`, `Cat` each expand into `__S*`, `__D*`, `I*`, `__V*`, `__C*`.
- Each class vtable writes its own `ty` and `__downcast` slots.
- `Dog` and `Cat` inherit through `Animal`.

Key callsites:

- `animal.downcast_rc::<Dog>()`
- `animal.downcast_rc::<Cat>()`

MIR callsite shape, observed from `/tmp/min_class_downcast-*.mir`:

```text
<(dyn oop_rs::prelude::IDowncast + 'static)>::downcast_rc::<dyn IDog>(...)
<(dyn oop_rs::prelude::IDowncast + 'static)>::downcast_rc::<dyn ICat>(...)
```

This suggests that a MIR recognizer can key on `IDowncast::downcast_rc::<TargetDyn>`.

## 3. Class hierarchy

```text
Object
└── Animal
    ├── Dog
    └── Cat
```

Concrete type graph:

```text
__CDog  -> dyn Dog  -> dyn Animal
__CCat  -> dyn Cat  -> dyn Animal
```

## 4. Downcast callsites

1. `must_succeed_downcast`
   - target: `Dog`
   - expanded target dyn trait: `dyn IDog`
   - expected: `must-success` if input allocation is `Dog::new()`

2. `must_fail_downcast`
   - target: `Cat`
   - expanded target dyn trait: `dyn ICat`
   - expected: `must-fail` for the current program, because the only allocation at the call site is `Dog::new()`
