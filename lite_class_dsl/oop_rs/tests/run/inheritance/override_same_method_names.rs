use oop_rs::prelude::*;

use crate::BUF;

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
type Mixin = mixin<
    {
        fn foo(&self) {
            println!("Mixin::foo");
        }
    },
>;

#[class(extends(Base), with(Mixin))]
type Derived = class<
    {
        fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(Base))]
        fn foo(&self) {
            println!("Derived::foo");
            (super as Mixin).foo();
        }
    },
>;

static EXPECTED: &[&str] = &["Derived::foo", "Mixin::foo"];

#[test]
fn test_override_same_method_name() {
    BUF.take();
    let derived = Derived::new();
    Base::foo(&*derived);
    assert_eq!(BUF.take(), EXPECTED);
}
