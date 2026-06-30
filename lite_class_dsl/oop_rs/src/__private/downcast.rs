use core::any::Any;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Deref;
use core::pin::Pin;
use core::ptr::NonNull;

use crate::{
    __private::{
        Dummy,
        data::{ClassHasData, HasData},
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn},
        vtable::{
            DynHasVtable, HasVtable, HasVtableImpl, InterfaceVtableTypeInfo, Type, Vtable,
            VtableTypeInfo,
        },
    },
    alloc::{boxed::Box, rc::Rc, sync::Arc},
    class::ClassConcrete,
    rc::{RcRef, RcRefImpl},
    read_vtable,
};

pub union ErasedPtr {
    _thin: NonNull<()>,
    _fat: NonNull<Downcast>,
}

impl ErasedPtr {
    pub fn new_thin<T>(ptr: NonNull<T>) -> Self {
        Self { _thin: ptr.cast() }
    }
}

const TYPE_NAME: &str = "Downcast";

pub struct __SDowncast<__Self: ClassConcrete> {
    __self: PhantomData<__Self>,
}

impl<__Self: ClassConcrete> HasData for __SDowncast<__Self> {
    type Data = Dummy;
}

impl<__Self: ClassConcrete> ClassHasData for __SDowncast<__Self> {}

impl<__Self: ClassConcrete> HasVtable for __SDowncast<__Self> {
    type Vtable = __VDowncast<__Self>;
}

impl<__Self: ClassConcrete> ClassHasDyn for __SDowncast<__Self> {
    type Dyn = Downcast;
}

pub unsafe trait IDowncast: Any {
    /// Returns the class-level Type (i.e. `Type::of::<dyn IMyClass>()`).
    ///
    /// Unlike `Any::type_id()` which returns the concrete struct's TypeId,
    /// this method is dispatched through the vtable, and each `#[class]`
    /// overrides it to return `Type::of::<ClassName>()` where `ClassName`
    /// is the public type alias (`dyn IMyClass`).
    ///
    /// Use this instead of `Any::type_id()` whenever you need a TypeId that
    /// is consistent with `TypeId::of::<ClassName>()`.
    fn ty(&self) -> Type;

    /// If `self` is type `T` where `ty == Type::of::<T>()`,
    /// then return a thin pointer to `self` as `&T`.
    /// If `self` implements trait `T` where `ty == Type::of::<dyn T>()`,
    /// return an (erased) fat pointer to `self` as `&dyn T`.
    fn __downcast(&self, ty: Type) -> Option<ErasedPtr>;
}

impl std::fmt::Debug for Downcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Downcast")?;
        #[cfg(debug_assertions)]
        write!(f, "<{}>", self.ty().type_name())?;
        Ok(())
    }
}

pub type Downcast = dyn IDowncast;

impl DynHasVtable for Downcast {
    type Vtable<__Self: ClassConcrete> = __VDowncast<__Self>;
}
impl ClassConcreteOrDyn for Downcast {
    const TYPE: &ClassConcreteOrDynTypeInfo = &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME);
}
impl ClassDyn for Downcast {
    type Class<__Self: ClassConcrete> = __SDowncast<__Self>;
}

#[repr(C)]
pub struct __VDowncast<__Self> {
    __self: PhantomData<__Self>,
    pub ty: Option<fn(&__Self) -> Type>,
    pub __downcast: Option<fn(&__Self, Type) -> Option<ErasedPtr>>,
}

crate::impl_vtable_copy!(<(__Self)> __VDowncast<__Self>);

impl<__Self: ClassConcrete> __VDowncast<__Self> {
    const __VTABLE: Self = Self {
        __self: PhantomData,
        ty: None,
        __downcast: Some(__downcast),
    };
}

fn __downcast<__Self: ClassConcrete>(__self: &__Self, ty: Type) -> Option<ErasedPtr> {
    if ty == __self.ty() {
        return Some(ErasedPtr::new_thin(__self.into()));
    }
    None
}

impl<__Self: ClassConcrete> HasVtableImpl for __SDowncast<__Self> {
    const VTABLE: Self::Vtable = __VDowncast::<__Self>::__VTABLE;
}
impl<__Self: ClassConcrete> Vtable for __VDowncast<__Self> {
    const TYPE: VtableTypeInfo<'static> =
        VtableTypeInfo::Interface(&InterfaceVtableTypeInfo::new(TYPE_NAME));
}

unsafe impl<__Self: ClassConcrete> IDowncast for __Self {
    fn ty(&self) -> Type {
        read_vtable!(<Downcast>::ty)(self)
    }
    fn __downcast(&self, ty: Type) -> Option<ErasedPtr> {
        read_vtable!(<Downcast>::__downcast)(self, ty)
    }
}

impl Downcast {
    pub fn is_type(&self, ty: Type) -> bool {
        self.__downcast(ty).is_some()
    }
    pub fn is<T: IDowncast + 'static + ?Sized>(&self) -> bool {
        self.is_type(Type::of::<T>())
    }
    fn downcast_ptr<T: IDowncast + 'static + ?Sized>(&self) -> Option<NonNull<T>> {
        Some(unsafe { std::mem::transmute_copy(&self.__downcast(Type::of::<T>())?) })
    }
    pub fn downcast_ref<T: IDowncast + 'static + ?Sized>(&self) -> Result<&T, DowncastError<T>> {
        self.downcast_ptr()
            .map(|ptr| unsafe { ptr.as_ref() })
            .ok_or_else(|| {
                DowncastError::<T>::new(
                    #[cfg(debug_assertions)]
                    self.ty().type_name(),
                )
            })
    }
    pub fn downcast_mut<T: IDowncast + 'static + ?Sized>(
        &mut self,
    ) -> Result<&mut T, DowncastError<T>> {
        self.downcast_ptr()
            .map(|mut ptr| unsafe { ptr.as_mut() })
            .ok_or_else(|| {
                DowncastError::<T>::new(
                    #[cfg(debug_assertions)]
                    self.ty().type_name(),
                )
            })
    }
    pub fn downcast_rc<T: IDowncast + 'static + ?Sized + ClassConcreteOrDyn + RcRef>(
        &self,
    ) -> Result<Rc<T>, DowncastError<T>> {
        self.downcast_ref().map(RcRefImpl::to_rc)
    }
}

pub struct DowncastError<T: ?Sized> {
    expected_type: PhantomData<T>,
    #[cfg(debug_assertions)]
    actual_type: &'static str,
}

impl<T: ?Sized> DowncastError<T> {
    fn new(#[cfg(debug_assertions)] actual_type: &'static str) -> Self {
        Self {
            expected_type: PhantomData,
            #[cfg(debug_assertions)]
            actual_type,
        }
    }
}

impl<T: ?Sized> fmt::Debug for DowncastError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot downcast to `{}`", core::any::type_name::<T>())?;
        #[cfg(debug_assertions)]
        write!(f, " (actual type: `{}`)", self.actual_type)?;
        Ok(())
    }
}

impl<T: ?Sized> fmt::Display for DowncastError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub trait FromRaw<T: ?Sized> {
    unsafe fn from_raw(ptr: NonNull<T>) -> Self;
}

pub trait IDowncastImpl: Deref<Target = Downcast> + Sized {
    type Result<T: ?Sized>: FromRaw<T>;
    fn downcast<T: IDowncast + 'static + ?Sized>(self) -> Result<Self::Result<T>, Self> {
        match (*self).downcast_ptr() {
            Some(ptr) => {
                std::mem::forget(self);
                Ok(unsafe { Self::Result::from_raw(ptr) })
            }
            None => Err(self),
        }
    }
}

impl<T: ?Sized> FromRaw<T> for Rc<T> {
    unsafe fn from_raw(ptr: NonNull<T>) -> Self {
        unsafe { Rc::from_raw(ptr.as_ptr()) }
    }
}

impl<T: ?Sized> FromRaw<T> for Box<T> {
    unsafe fn from_raw(ptr: NonNull<T>) -> Self {
        unsafe { Box::from_raw(ptr.as_ptr()) }
    }
}

impl<T: ?Sized> FromRaw<T> for Arc<T> {
    unsafe fn from_raw(ptr: NonNull<T>) -> Self {
        unsafe { Arc::from_raw(ptr.as_ptr()) }
    }
}

impl<T: ?Sized> FromRaw<T> for Pin<Box<T>> {
    unsafe fn from_raw(ptr: NonNull<T>) -> Self {
        unsafe { Pin::new_unchecked(Box::from_raw(ptr.as_ptr())) }
    }
}

impl IDowncastImpl for Rc<Downcast> {
    type Result<U: ?Sized> = Rc<U>;
}

impl IDowncastImpl for Box<Downcast> {
    type Result<U: ?Sized> = Box<U>;
}

impl IDowncastImpl for Arc<Downcast> {
    type Result<U: ?Sized> = Arc<U>;
}

impl IDowncastImpl for Pin<Box<Downcast>> {
    type Result<U: ?Sized> = Pin<Box<U>>;
}
