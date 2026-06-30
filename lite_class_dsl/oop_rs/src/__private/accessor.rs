//! Placeholder module initialized for vtable private internals.
use crate::__private::Dummy;

/// # Safety
///
/// `Self::Accessor` be `#[repr(transparent)]` of `Self`.

pub unsafe trait Access {
    type Accessor<Mode: AccessMode>: ?Sized;

    #[inline(always)]
    fn __access<Mode: AccessMode>(&self) -> &Self::Accessor<Mode> {
        unsafe { core::mem::transmute_copy(&self) }
    }

    #[inline(always)]
    fn get(&self) -> &Self::Accessor<Get> {
        self.__access()
    }
    #[inline(always)]
    fn get_mut(&self) -> &Self::Accessor<GetMut> {
        self.__access()
    }
    #[inline(always)]
    fn set(&self) -> &Self::Accessor<Set> {
        self.__access()
    }
    #[inline(always)]
    fn update(&self) -> &Self::Accessor<Update> {
        self.__access()
    }
    #[inline(always)]
    fn replace(&self) -> &Self::Accessor<Replace> {
        self.__access()
    }
    #[inline(always)]
    fn replace_with(&self) -> &Self::Accessor<ReplaceWith> {
        self.__access()
    }
    #[inline(always)]
    fn raw(&self) -> &Self::Accessor<Raw> {
        self.__access()
    }
}

/// # Safety
///
/// `Self::Accessor<_, _, __T>` be `#[repr(transparent)]` of `__T`.
pub unsafe trait MixinAccessor {
    type Accessor<Mode: AccessMode, Upcast: AccessorUpcast<__T>, __T: ?Sized>: ?Sized;

    fn __access<Mode: AccessMode, Upcast: AccessorUpcast<__T>, __T: ?Sized>(
        input: &__T,
    ) -> &Self::Accessor<Mode, Upcast, __T> {
        unsafe { core::mem::transmute_copy(&input) }
    }
}

pub trait AccessorUpcast<SelfDyn: ?Sized> {
    type SuperAccessor<M: AccessMode>: ?Sized;

    fn upcast<M: AccessMode>(this: &SelfDyn) -> &Self::SuperAccessor<M>;
}

impl<SelfDyn: ?Sized> AccessorUpcast<SelfDyn> for Dummy {
    type SuperAccessor<M: AccessMode> = Dummy;

    fn upcast<M: AccessMode>(_this: &SelfDyn) -> &Dummy {
        &Dummy
    }
}

pub struct Get;
pub struct GetMut;
pub struct Set;
pub struct Update;
pub struct Replace;
pub struct ReplaceWith;
pub struct Raw;

pub trait AccessMode: sealed::Sealed {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Get {}
    impl Sealed for super::GetMut {}
    impl Sealed for super::Set {}
    impl Sealed for super::Update {}
    impl Sealed for super::Replace {}
    impl Sealed for super::ReplaceWith {}
    impl Sealed for super::Raw {}

    impl super::AccessMode for super::Get {}
    impl super::AccessMode for super::GetMut {}
    impl super::AccessMode for super::Set {}
    impl super::AccessMode for super::Update {}
    impl super::AccessMode for super::Replace {}
    impl super::AccessMode for super::ReplaceWith {}
    impl super::AccessMode for super::Raw {}
}
