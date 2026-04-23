use crate::alloc::rc::{Rc, Weak};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ptr::NonNull;

#[derive(Clone, Copy)]
pub struct Method<T, F: Copy> {
    object: T,
    fn_ptr: NonNull<()>,
    _marker: PhantomData<F>,
}

pub type MethodRc<T, F> = Method<Rc<T>, F>;
pub type MethodWeak<T, F> = Method<Weak<T>, F>;

impl<T: ?Sized, F: Copy> PartialEq for MethodRc<T, F> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.object, &other.object) && self.fn_ptr == other.fn_ptr
    }
}

impl<T: ?Sized, F: Copy> Eq for MethodRc<T, F> {}

impl<T: ?Sized, F: Copy> Hash for MethodRc<T, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.object).hash(state);
        self.fn_ptr.hash(state);
    }
}

impl<T: ?Sized, F: Copy> PartialEq for MethodWeak<T, F> {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.object, &other.object) && self.fn_ptr == other.fn_ptr
    }
}

impl<T: ?Sized, F: Copy> Eq for MethodWeak<T, F> {}

impl<T: ?Sized, F: Copy> Hash for MethodWeak<T, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Weak::as_ptr(&self.object).hash(state);
        self.fn_ptr.hash(state);
    }
}

const fn check_fn_ptr_size<F: Copy>() {
    assert!(
        core::mem::size_of::<F>() == core::mem::size_of::<NonNull<()>>(),
        "Function pointer must be the same size as a pointer"
    );
}

impl<T, F: Copy> Method<T, F> {
    pub const fn new(object: T, fn_ptr: F) -> Self {
        const { check_fn_ptr_size::<F>() }
        Self {
            object,
            fn_ptr: unsafe { core::mem::transmute_copy(&fn_ptr) },
            _marker: PhantomData,
        }
    }
    #[inline(always)]
    const fn fn_ptr(&self) -> F {
        unsafe { core::mem::transmute_copy(&self.fn_ptr) }
    }
}

macro_rules! impl_method_rc {
    ($($U:ident),* $(,)?) => {
        impl<T: ?Sized, Ret, $($U),*> MethodRc<T, fn(&T, $($U),*) -> Ret> {
            #[allow(non_snake_case)]
            #[inline(always)]
            pub fn invoke(&self, $($U: $U),*) -> Ret {
                self.fn_ptr()(&self.object, $($U),*)
            }
        }
    }
}

macro_rules! impl_method_weak {
    ($($U:ident),* $(,)?) => {
        impl<T: ?Sized, Ret, $($U),*> MethodWeak<T, fn(&T, $($U),*) -> Ret> {
            #[allow(non_snake_case)]
            #[inline(always)]
            pub fn invoke(&self, $($U: $U),*) -> Option<Ret>
            {
                Some(self.fn_ptr()(&*self.object.upgrade()?, $($U),*))
            }

            #[allow(non_snake_case)]
            #[inline(always)]
            #[track_caller]
            pub fn force_invoke(&self, $($U: $U),*) -> Ret {
                let object = self.object.upgrade().expect("object was already released");
                self.fn_ptr()(&object, $($U),*)
            }
        }
    };
}

impl_method_rc!();
impl_method_rc!(U1);
impl_method_rc!(U1, U2);
impl_method_rc!(U1, U2, U3);
impl_method_rc!(U1, U2, U3, U4);
impl_method_rc!(U1, U2, U3, U4, U5);
impl_method_rc!(U1, U2, U3, U4, U5, U6);
impl_method_rc!(U1, U2, U3, U4, U5, U6, U7);
impl_method_rc!(U1, U2, U3, U4, U5, U6, U7, U8);
impl_method_rc!(U1, U2, U3, U4, U5, U6, U7, U8, U9);
impl_method_rc!(U1, U2, U3, U4, U5, U6, U7, U8, U9, U10);
impl_method_rc!(U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11);
impl_method_rc!(U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12);

impl_method_weak!();
impl_method_weak!(U1);
impl_method_weak!(U1, U2);
impl_method_weak!(U1, U2, U3);
impl_method_weak!(U1, U2, U3, U4);
impl_method_weak!(U1, U2, U3, U4, U5);
impl_method_weak!(U1, U2, U3, U4, U5, U6);
impl_method_weak!(U1, U2, U3, U4, U5, U6, U7);
impl_method_weak!(U1, U2, U3, U4, U5, U6, U7, U8);
impl_method_weak!(U1, U2, U3, U4, U5, U6, U7, U8, U9);
impl_method_weak!(U1, U2, U3, U4, U5, U6, U7, U8, U9, U10);
impl_method_weak!(U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11);
impl_method_weak!(U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12);

#[macro_export]
macro_rules! method {
    (Rc::new($self:ident, Self::$method:ident $(,)?)) => {
        $crate::method::Method::new($self.to_rc(), $crate::read_vtable!($method))
    };
    (Weak::new($self:ident, Self::$method:ident $(,)?)) => {
        $crate::method::Method::new($self.to_weak(), $crate::read_vtable!($method))
    };
    (Rc::new($self:ident, $Class:ident::$method:ident $(,)?)) => {
        $crate::method::Method::new($self.to_rc(), $crate::read_vtable!(<$Class>::$method))
    };
    (Weak::new($self:ident, $Class:ident::$method:ident $(,)?)) => {
        $crate::method::Method::new($self.to_weak(), $crate::read_vtable!(<$Class>::$method))
    };
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[class]
    type Base = class<
        {
            fn new() -> Self {
                Self {}
            }
            fn name(&self) -> &'static str {
                "Base"
            }
            fn test_method(&self) -> &'static str {
                method!(Rc::new(self, Self::name)).invoke()
            }
        },
    >;

    #[class(extends(Base))]
    type Derived = class<
        {
            fn new() -> Self {
                Self { ..Super::new() }
            }
            #[method(override(Base))]
            fn name(&self) -> &'static str {
                "Derived"
            }
        },
    >;

    #[test]
    fn test_method() {
        let base = Base::new();
        let derived = Derived::new();
        assert_eq!(base.test_method(), "Base");
        assert_eq!(derived.test_method(), "Derived");
        assert_eq!((&*derived as &Base).test_method(), "Derived");
    }
}
