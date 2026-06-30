use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;

use crate::__private::data::HasData;
use crate::alloc::boxed::Box;
use crate::alloc::rc::{Rc, Weak};
use crate::class::ClassConcrete;
use crate::rc::RcRef;
use crate::{
    __private::{generic::TypeInfo, streq},
    class::HasSuper,
};

pub trait ClassHasDyn {
    type Dyn: ClassDyn + ?Sized;
}

pub trait AsDyn: ClassHasDyn {
    fn as_dyn(&self) -> &Self::Dyn;
}

pub trait ClassDyn: ClassConcreteOrDyn {
    type Class<__Self: ClassConcrete>: ClassHasDyn<Dyn = Self>;
}

pub trait MixinDyn {
    type Class<__Super: HasData, __Self: ClassConcrete>: MixinHasDyn<Dyn = Self>;
}

pub trait MixinHasDyn {
    type Dyn: MixinDyn + ?Sized;
}

pub trait MixinInstanceDyn: ClassConcreteOrDyn {
    type InstanceDyn<SuperDyn: ClassConcreteOrDyn>: ClassConcreteOrDyn + Deref<Target = SuperDyn>;
}

#[repr(transparent)]
pub struct Dyn<Dyn: ?Sized, T: ?Sized = Dyn>(PhantomData<T>, Dyn);

unsafe impl<Dyn: RcRef + ?Sized, T: ?Sized + RcRef> RcRef for self::Dyn<Dyn, T> {}

impl<Dyn: ?Sized, T> self::Dyn<Dyn, T> {
    pub unsafe fn into_rc_unchecked(self: Rc<Self>) -> Rc<T> {
        unsafe { Rc::from_raw(Rc::into_raw(self).cast()) }
    }
    pub unsafe fn into_weak_unchecked(this: Weak<Self>) -> Weak<T> {
        unsafe { Weak::from_raw(Weak::into_raw(this).cast()) }
    }
    pub unsafe fn into_box_unchecked(self: Box<Self>) -> Box<T> {
        unsafe { Box::from_raw(Box::into_raw(self).cast()) }
    }
    pub unsafe fn into_pin_box_unchecked(self: Pin<Box<Self>>) -> Pin<Box<T>> {
        unsafe {
            Pin::new_unchecked(Box::from_raw(
                Box::into_raw(Pin::into_inner_unchecked(self)).cast(),
            ))
        }
    }
}

impl<Dyn: ?Sized, T: ?Sized> self::Dyn<Dyn, T> {
    pub fn as_dyn(&self) -> &Dyn {
        &self.1
    }
    pub fn as_mut_dyn(&mut self) -> &mut Dyn {
        &mut self.1
    }
    pub fn into_rc_dyn(self: Rc<Self>) -> Rc<Dyn> {
        unsafe { core::mem::transmute(self) }
    }
    pub fn into_weak_dyn(this: Weak<Self>) -> Weak<Dyn> {
        unsafe { core::mem::transmute(this) }
    }
    pub fn into_box_dyn(self: Box<Self>) -> Box<Dyn> {
        unsafe { core::mem::transmute(self) }
    }
    pub fn into_pin_box_dyn(self: Pin<Box<Self>>) -> Pin<Box<Dyn>> {
        unsafe { core::mem::transmute(self) }
    }

    pub fn from_ref_dyn(this: &Dyn) -> &Self {
        unsafe { core::mem::transmute(this) }
    }
    pub fn from_mut_dyn(this: &mut Dyn) -> &mut Self {
        unsafe { core::mem::transmute(this) }
    }
    pub fn from_rc_dyn(this: Rc<Dyn>) -> Rc<Self> {
        unsafe { core::mem::transmute(this) }
    }
    pub fn from_weak_dyn(this: Weak<Dyn>) -> Weak<Self> {
        unsafe { core::mem::transmute(this) }
    }
    pub fn from_box_dyn(this: Box<Dyn>) -> Box<Self> {
        unsafe { core::mem::transmute(this) }
    }
    pub fn from_pin_box_dyn(this: Pin<Box<Dyn>>) -> Pin<Box<Self>> {
        unsafe { core::mem::transmute(this) }
    }
}

impl<Dyn: ?Sized> Deref for self::Dyn<Dyn> {
    type Target = Dyn;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<Dyn: ?Sized> DerefMut for self::Dyn<Dyn> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

/// Whether a class is a concrete or a ClassDyn.
pub trait ClassConcreteOrDyn: 'static {
    const TYPE: &ClassConcreteOrDynTypeInfo;
}

#[derive(Clone, Copy)]
enum ConcreteOrDyn {
    Concrete,
    MixinDyn,
    Dyn,
}

#[derive(Clone, Copy)]
pub struct ClassConcreteOrDynTypeInfo {
    __super: Option<&'static ClassConcreteOrDynTypeInfo>,
    // FIXME: This should use `TypeId` for comparison, but `TypeId` cannot be
    // compared at compile time yet, so we use `type_name` for working around.
    // type_id: TypeId,
    type_name: &'static str,
    generics: &'static [&'static TypeInfo],
    kind: ConcreteOrDyn,
}

impl ClassConcreteOrDynTypeInfo {
    pub const fn new_dyn(type_name: &'static str) -> Self {
        Self {
            __super: None,
            type_name,
            generics: &[],
            kind: ConcreteOrDyn::Dyn,
        }
    }

    pub const fn new_concrete(type_name: &'static str) -> Self {
        Self {
            __super: None,
            type_name,
            generics: &[],
            kind: ConcreteOrDyn::Concrete,
        }
    }

    pub const fn new_mixin(type_name: &'static str) -> Self {
        Self {
            __super: None,
            type_name,
            generics: &[],
            kind: ConcreteOrDyn::MixinDyn,
        }
    }

    pub const fn with_super<
        Data: ClassConcreteOrDyn + HasSuper<Super: ClassConcreteOrDyn> + ?Sized,
    >(
        self,
    ) -> Self {
        Self {
            __super: Some(&Data::Super::TYPE),
            kind: self.kind,
            ..self
        }
    }

    pub const fn with_generics(self, generics: &'static [&'static TypeInfo]) -> Self {
        Self { generics, ..self }
    }

    pub(super) const fn eq(&self, other: &Self) -> bool {
        streq(self.type_name, other.type_name)
            && self.mixin_eq(other)
            && TypeInfo::eq_slice(self.generics, other.generics)
    }

    const fn is_mixin(&self) -> bool {
        matches!(self.kind, ConcreteOrDyn::MixinDyn)
    }

    const fn mixin_eq(&self, other: &Self) -> bool {
        if self.is_mixin() != other.is_mixin() {
            return false;
        }
        if self.is_mixin()
            && let (Some(__super), Some(__super_other)) = (self.__super, other.__super)
        {
            return __super.eq(__super_other);
        }
        true
    }

    pub const fn is_subclass_of(&self, other: &Self) -> bool {
        if self.eq(other) {
            true
        } else if let Some(super_class) = self.__super {
            super_class.is_subclass_of(other)
        } else {
            false
        }
    }
}
