#[macro_export]
macro_rules! def_unsize {
    () => {
        use __unsize::{Unsize, UnsizeImpl};

        mod __unsize {
            pub trait Unsize<Dyn: ?Sized> {
                fn unsize_ref(&self) -> &Dyn;
            }

            pub(super) trait UnsizeImpl<Dyn: ?Sized>:
                ::core::ops::Deref<Target: Unsize<Dyn>> + Sized
            {
                type Result: $crate::__private::downcast::FromRaw<Dyn>;
                fn unsize(self) -> Self::Result {
                    let ptr = self.unsize_ref().into();
                    ::core::mem::forget(self);
                    unsafe { $crate::__private::downcast::FromRaw::from_raw(ptr) }
                }
            }

            impl<T: Unsize<Dyn> + ?Sized, Dyn: ?Sized>
                UnsizeImpl<Dyn> for $crate::alloc::rc::Rc<T>
            {
                type Result = $crate::alloc::rc::Rc<Dyn>;
            }

            impl<T: Unsize<Dyn> + ?Sized, Dyn: ?Sized>
                UnsizeImpl<Dyn> for $crate::alloc::boxed::Box<T>
            {
                type Result = $crate::alloc::boxed::Box<Dyn>;
            }

            impl<T: Unsize<Dyn> + ?Sized, Dyn: ?Sized>
                UnsizeImpl<Dyn> for $crate::alloc::sync::Arc<T>
            {
                type Result = $crate::alloc::sync::Arc<Dyn>;
            }

            impl<T: Unsize<Dyn> + ?Sized, Dyn: ?Sized>
                UnsizeImpl<Dyn> for ::core::pin::Pin<Box<T>>
            {
                type Result = ::core::pin::Pin<Box<Dyn>>;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_unsize {
    ($trait:path) => {
        impl<T: $trait> Unsize<dyn $trait> for T {
            fn unsize_ref(&self) -> &dyn $trait {
                self
            }
        }

        impl Unsize<dyn $trait> for dyn $trait {
            fn unsize_ref(&self) -> &dyn $trait {
                self
            }
        }
    };
}
