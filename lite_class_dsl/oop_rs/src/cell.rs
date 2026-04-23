use crate::alloc::rc::{Rc, Weak};
use core::cell::Cell;

pub unsafe trait CloneFromCopy: Clone {}

unsafe impl<T: ?Sized> CloneFromCopy for Rc<T> {}
unsafe impl<T: ?Sized> CloneFromCopy for Weak<T> {}
// unsafe impl<T: ?Sized> CloneFromCopy for CRc<T> {}
// unsafe impl<T: ?Sized> CloneFromCopy for CWeak<T> {}
unsafe impl<T: CloneFromCopy> CloneFromCopy for Option<T> {}

pub trait CellGetCloned<T: CloneFromCopy> {
    fn get_cloned(&self) -> T;
}

impl<T: CloneFromCopy> CellGetCloned<T> for Cell<T> {
    fn get_cloned(&self) -> T {
        T::clone(unsafe { &*self.as_ptr() })
    }
}
