use oop_rs::prelude::*;

#[class]
type Mixin = mixin<
    {
        #[late]
        let ref late_field: String = format!("{}", self.f());

        fn f(&self) -> usize {
            0_usize
        }
    },
>;

#[class(with(Mixin))]
type Class = class<
    {
        fn new() -> Self {
            Self {}
        }
    },
>;

#[test]
fn test_late_field() {
    let mixin = Class::new();
    assert_eq!(mixin.get().late_field(), "0");
}
