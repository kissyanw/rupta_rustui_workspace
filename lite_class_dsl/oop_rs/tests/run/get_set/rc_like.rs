use crate::BUF;
use oop_rs::prelude::*;

struct RcWrapper<T: ?Sized>(CRc<T>);

impl<T: ?Sized> Clone for RcWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

unsafe impl<T: ?Sized> IsRcLike for RcWrapper<T> {}

#[class]
type Class = class<
    {
        pub fn new() -> Self {
            Self {}
        }
    },
>;

#[class]
type RcLikeClass = class<
    {
        let rc_like: RcLike<RcWrapper<Class>> = RcWrapper(Class::new());

        pub fn new() -> Self {
            println!("RcLikeClass::new");
            Self {}
        }
    },
>;

#[test]
fn test_rc_like() {
    BUF.take();
    let _rc_like = RcLikeClass::new();
    assert_eq!(BUF.take(), ["RcLikeClass::new"]);
}
