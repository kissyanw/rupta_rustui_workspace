# Lite Class DSL Surface Syntax Map

This note summarizes the current surface syntax of `lite_class_dsl` for study
purposes. The main syntax sources are:

- `lite_class_dsl/oop_rs_macros/src/syntax/class.rs`
- `lite_class_dsl/oop_rs_macros/src/syntax/field.rs`
- `lite_class_dsl/oop_rs_macros/src/syntax/method.rs`
- `lite_class_dsl/oop_rs_macros/src/syntax/tests.rs`

## Core Syntax

| Category | Syntax | Meaning | Suggested Example |
| --- | --- | --- | --- |
| Type kind | `type A = class<{ ... }>;` | Define a class | `syntax/tests.rs::test_simple_class` |
| Type kind | `type I = interface<{ ... }>;` | Define an interface | `oop_rs/tests/run/interface/multiple_interfaces.rs` |
| Type kind | `type M = mixin<{ ... }>;` | Define a mixin | `syntax/tests.rs::test_mixin`, `oop_rs/tests/run/mixin/multi_withs.rs` |
| Class attr | `#[class(abstract)]` | Define an abstract class | `syntax/tests.rs::test_abstract_class` |
| Class attr | `#[class(final)]` | Define a final class | `syntax/tests.rs::test_final_class` |
| Class attr | `#[class(extends(Base))]` | Single inheritance | `oop_rs/tests/run/ctor.rs` |
| Class attr | `#[class(implements(I, J))]` | Implement interfaces | `oop_rs/tests/run/interface/multiple_interfaces.rs` |
| Class attr | `#[class(on(A, B))]` | Restrict mixin host classes | `oop_rs/tests/run/mixin/multi_ons.rs` |
| Class attr | `#[class(with(M1, M2))]` | Apply mixins to a class | `oop_rs/tests/run/mixin/multi_withs.rs` |
| Field | `let x: i32;` | Immutable field | `syntax/tests.rs::test_simple_field` |
| Field | `let mut x: String;` | Mutable field | `syntax/tests.rs::test_field_with_mut` |
| Field | `let x: i32 = 42;` | Field with default initializer | `syntax/tests.rs::test_field_with_init` |
| Field attr | `#[late] let x: T;` | Late-initialized field | `syntax/tests.rs::test_field_with_late`, `oop_rs/tests/run/mixin/late_field.rs` |
| Field attr | `#[vis(pub)] let x: T;` | Generate accessor with explicit visibility | `oop_rs/tests/animal_hierarchy/animal.rs` |
| Ctor | `fn new(...) -> Self { Self { ... } }` | Constructor | `syntax/tests.rs::test_simple_constructor` |
| Ctor | `Self { ..., ..Super::new(...) }` | Constructor with super call | `syntax/tests.rs::test_constructor_with_super_call` |
| Ctor | `let self = Self { ... }; ...; self` | Constructor with post-construction initialization | `syntax/tests.rs::test_constructor_with_self_stmt` |
| Method | `fn f(&self) { ... }` | Instance method | `syntax/tests.rs::test_simple_method` |
| Method | `fn f(&mut self) { ... }` | Mutable instance method | `syntax/tests.rs::test_mutable_method` |
| Method | `fn f(self) { ... }` | By-value receiver | `syntax/tests.rs::test_method_with_self_by_value` |
| Method | `fn f(self: Pin<&mut Self>)` | Explicit receiver form | `syntax/tests.rs::test_method_with_pinned_self` |
| Method attr | `#[method(final)]` | Final method | `syntax/tests.rs::test_final_method` |
| Method attr | `#[method(override(Base))]` | Override a method from a class, interface, or mixin | `syntax/tests.rs::test_method_with_override` |
| Static item | `fn helper() -> T` | Associated function, not a virtual method | `syntax/tests.rs::test_class_with_functions` |
| Const item | `const X: T = ...;` | Associated constant | `syntax/tests.rs::test_class_with_consts` |
| Generic form | `type C<T> = class<{ ... }>;` | Generic class declaration | `syntax/tests.rs::test_class_with_generics` |
| Visibility | `pub type C = ...`, `pub fn ...` | Reuse Rust visibility syntax | `syntax/tests.rs::test_class_with_visibility` |

## Important Constraints

| Constraint | Meaning |
| --- | --- |
| `override` and `final` cannot be combined | `#[method(override(Base), final)]` is rejected |
| Constructors must return `Self` | Otherwise they are not recognized as constructors |
| Constructor bodies must end in `Self { ... }` or `let self = Self { ... }; ...; self` | This is part of the DSL contract |
| `interface` cannot use `extends` | Rejected by class attribute checking |
| `mixin` cannot use `extends` or `with` | Rejected by class attribute checking |
| `Downcast` must be first in `implements(...)` | Enforced by syntax checking |

## Recommended Reading Order

1. `lite_class_dsl/oop_rs_macros/src/syntax/tests.rs`
2. `lite_class_dsl/oop_rs_macros/src/syntax/class.rs`
3. `lite_class_dsl/oop_rs_macros/src/syntax/field.rs`
4. `lite_class_dsl/oop_rs_macros/src/syntax/method.rs`
5. `lite_class_dsl/oop_rs/src/object.rs`
6. `lite_class_dsl/oop_rs/tests/run/interface/multiple_interfaces.rs`
7. `lite_class_dsl/oop_rs/tests/run/mixin/multi_withs.rs`
8. `lite_class_dsl/oop_rs/tests/run/gallery_page.rs`

## First-Pass Study Focus

For a first pass, focus on:

1. `class`, `interface`, `mixin`
2. `extends`, `implements`, `with`, `on`
3. Fields
4. Constructors
5. `#[method(override(...))]`

Leave the less central cases, such as pinned receivers and more elaborate
constructor shapes, for the second pass.
