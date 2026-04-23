use oop_rs::prelude::*;

#[class]
type Class = class<
    {
        const CONSTANT: i32 = 0;

        fn function() -> i32 {
            1
        }
    },
>;

#[class]
type Interface = interface<
    {
        const CONSTANT: i32 = 0;
        fn function() -> i32 {
            1
        }
        #[allow(dead_code)]
        fn method(&self);
    },
>;

#[class]
type Mixin = mixin<
    {
        const CONSTANT: i32 = 0;
        fn function() -> i32 {
            1
        }
    },
>;

#[test]
fn test_static_items() {
    assert_eq!(Class::CONSTANT, 0);
    assert_eq!(Class::function(), 1);

    assert_eq!(Interface::CONSTANT, 0);
    assert_eq!(Interface::function(), 1);

    assert_eq!(Mixin::CONSTANT, 0);
    assert_eq!(Mixin::function(), 1);
}
