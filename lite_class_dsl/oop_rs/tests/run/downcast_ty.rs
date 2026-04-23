// Tests for IDowncast::ty() method
//
// Verifies that ty() returns TypeId::of::<ClassName>() (the dyn trait TypeId),
// NOT Any::type_id() (the concrete struct TypeId), and that it dispatches
// correctly through the vtable even when called via a base class reference.

use std::any::Any;
use std::collections::HashMap;

use oop_rs::prelude::*;

#[class(extends(Object))]
pub type Base = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
    },
>;

#[class(extends(Base))]
pub type Child = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
    },
>;

#[test]
fn ty_returns_class_type_id_not_any_type_id() {
    // IDowncast::ty() should return TypeId::of::<ClassName>() (i.e. dyn IMyClass),
    // which differs from Any::type_id() that returns the concrete struct's TypeId.
    let base = Base::new();
    let ty_via_method = base.ty();
    let ty_of_alias = Type::of::<Base>();
    let ty_via_any = (&*base as &dyn Any).type_id();

    assert_eq!(
        ty_via_method, ty_of_alias,
        "ty() should return Type::of::<Base>()"
    );
    assert_ne!(
        ty_via_method.type_id(), ty_via_any,
        "ty() should differ from Any::type_id()"
    );
}

#[test]
fn ty_dispatches_to_subclass_via_base_ref() {
    // When calling ty() through a base class reference, vtable dispatch
    // should return the actual (sub)class TypeId, not the base class TypeId.
    let child = Child::new();
    let base_ref: CRc<Base> = child as CRc<Base>;

    let ty_via_base = base_ref.ty();
    let expected = Type::of::<Child>();
    let not_expected = Type::of::<Base>();

    assert_eq!(
        ty_via_base, expected,
        "ty() via base ref should return Child's TypeId"
    );
    assert_ne!(
        ty_via_base, not_expected,
        "ty() via base ref should NOT return Base's TypeId"
    );
}

#[test]
fn ty_usable_as_hashmap_key() {
    // The motivation for ty(): register and look up by the same TypeId
    // in a HashMap, which fails with Any::type_id().
    let mut map: HashMap<Type, &str> = HashMap::new();

    // Register using TypeId::of::<ClassName>()
    map.insert(Type::of::<Base>(), "base_value");
    map.insert(Type::of::<Child>(), "child_value");

    // Look up using ty() — should find the entry
    let base = Base::new();
    let child = Child::new();

    assert_eq!(
        map.get(&base.ty()),
        Some(&"base_value"),
        "ty() should match TypeId::of::<Base>() in HashMap"
    );
    assert_eq!(
        map.get(&child.ty()),
        Some(&"child_value"),
        "ty() should match TypeId::of::<Child>() in HashMap"
    );

    // Also works when accessed through a base class reference
    let child_as_base: CRc<Base> = Child::new() as CRc<Base>;
    assert_eq!(
        map.get(&child_as_base.ty()),
        Some(&"child_value"),
        "ty() via base ref should still match Child's entry"
    );
}
