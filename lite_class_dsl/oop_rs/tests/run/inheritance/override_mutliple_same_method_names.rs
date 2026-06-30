use crate::BUF;
use oop_rs::prelude::*;

#[class(abstract)]
type Base = class<
    {
        fn new() -> Self {
            Self {}
        }
        fn foo(&self);
    },
>;

#[class]
type Interface = interface<
    {
        fn foo(&self);
    },
>;

#[class]
type Mixin = mixin<
    {
        fn foo(&self) {
            println!("Mixin::foo");
        }
    },
>;

#[class(extends(Base), with(Mixin), implements(Interface))]
type Derived = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(Base, Mixin, Interface))]
        fn foo(&self) {
            super.foo();
            println!("Derived::foo");
        }
    },
>;

static EXPECTED: &[&str] = &["Mixin::foo", "Derived::foo"];

#[test]
fn test_override_mutliple_same_method_names() {
    BUF.take();
    let derived = Derived::new();
    (&*derived as &Base).foo();
    assert_eq!(BUF.take(), EXPECTED);
    (&*derived as &Mixin).foo();
    assert_eq!(BUF.take(), EXPECTED);
    (&*derived as &Interface).foo();
    assert_eq!(BUF.take(), EXPECTED);
}
