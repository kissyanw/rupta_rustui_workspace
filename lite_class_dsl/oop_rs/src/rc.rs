use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::__private::dynamic::ClassConcreteOrDyn;
use crate::alloc::rc::{Rc, Weak};

/// A trait that guarantees any reference to this type is from an Rc<T> instance.
pub unsafe trait RcRef: 'static {}

pub trait RcRefImpl: RcRef {
    fn to_rc(&self) -> CRc<Self>;
    fn to_weak(&self) -> CWeak<Self>;
}

impl<__Self: ClassConcreteOrDyn + RcRef + ?Sized> RcRefImpl for __Self {
    fn to_rc(&self) -> CRc<Self> {
        unsafe {
            // FIXME: This is an undefined behavior in stacked borrows/tree borrows,
            // because `self` doesn't have the right to write to the reference count.
            // Should switch to a custom reference type since the features
            // `#[feature(arbitrary_self_types)]` and `#[feature(field_projection)]` are stable.
            Rc::increment_strong_count(self);
            CRc::from_raw(self)
        }
    }
    fn to_weak(&self) -> CWeak<Self> {
        let weak = core::mem::ManuallyDrop::new(unsafe { CWeak::from_raw(self) });
        CWeak::clone(&weak)
    }
}

pub trait RcDefault: RcRef {
    fn default() -> CRc<Self>;
}

pub type CRc<T> = Rc<T>;
pub type CWeak<T> = Weak<T>;

pub unsafe trait IsRcLike: Clone {}

unsafe impl<T: IsRcLike> IsRcLike for Option<T> {}

/// A type wrapper that tells the `classes!` macro to treat the type as an `Rc`-like type.
#[derive(Default, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RcLike<T>(T);

impl<T> RcLike<T> {
    pub const fn new(value: T) -> Self
    where
        T: IsRcLike,
    {
        Self(value)
    }
    pub fn into_inner(this: Self) -> T {
        this.0
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for RcLike<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: core::fmt::Display> core::fmt::Display for RcLike<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "rpds")]
unsafe impl<K: Eq + core::hash::Hash, V, H: core::hash::BuildHasher + Clone> IsRcLike
    for rpds::HashTrieMap<K, V, archery::RcK, H>
{
}

/// An `Rc<MaybeUninit<T>>` which guarantees that the `Rc` is unique and initalized
/// except the first `U` payload of this `Rc<MaybeUninit<T>>`.
///
/// It is used in the initialization of class objects.
#[repr(transparent)]
pub struct CRcUninit<T, U = T> {
    initialized: Rc<MaybeUninit<T>>,
    execpt: PhantomData<U>,
}

impl<T> CRcUninit<T> {
    pub fn new() -> Self {
        Self {
            initialized: Rc::new_uninit(),
            execpt: PhantomData,
        }
    }
}

impl<T, U> CRcUninit<T, U> {
    /// Get the mutable pointer of uninitialized part of this `Rc<MaybeUninit<T>>`.
    pub fn as_uninit_mut_ptr(&mut self) -> *mut U {
        // FIXME: replace this since `Rc::as_mut_unchecked` is stablized
        // SAFETY: `as_ptr` returns a `*const T` casted from `*mut T`, so the `*mut T`
        // by `cast_mut` is safe to write to.
        Rc::as_ptr(&mut self.initialized).cast_mut().cast()
    }

    pub fn cast_uninit<V>(self) -> CRcUninit<T, V> {
        const {
            assert!(
                core::mem::size_of::<V>() >= core::mem::size_of::<U>(),
                "cannot cast safely to a type assuming a smaller part of the uninitialized part, \
                use the unsafe method `assume_init_except` instead"
            );
        }
        CRcUninit {
            initialized: self.initialized,
            execpt: PhantomData,
        }
    }

    /// Assume it is partially initalized execpt the first `V` payload.
    pub unsafe fn assume_init_except<V>(self) -> CRcUninit<T, V> {
        CRcUninit {
            initialized: self.initialized,
            execpt: PhantomData,
        }
    }
}

impl<T> CRcUninit<T, ()> {
    /// Assume it is fully initialized.
    ///
    /// This is safe because `()` marks there are no uninitialized parts in this `Rc<MaybeUninit<T>>`.
    pub fn assume_init(self) -> CRc<T> {
        unsafe { self.initialized.assume_init() }
    }
}
