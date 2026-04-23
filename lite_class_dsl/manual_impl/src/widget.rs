use std::{marker::PhantomData, ops::Deref};

use oop_rs::{
    __private::{
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        downcast::{Downcast, ErasedPtr},
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, DynHasVtable, HasVtable,
            HasVtableImpl, Type, Vtable, VtableTypeInfo,
        },
    },
    assert_subclass,
    class::{ClassConcrete, HasSuper},
    object::{IObject, Object},
    prelude::{CClass, CVtable},
    rc::{CRc, CRcUninit, RcRef},
    read_vtable, write_vtable,
};

use crate::element::Element;

const TYPE_NAME: &str = concat!(module_path!(), "::Widget");

#[repr(C)]
pub struct __SWidget<__Self> {
    __self: PhantomData<__Self>,
    data: __DWidget,
}

pub struct __DWidget {}

impl ClassOrMixinData for __DWidget {
    const TYPE: DataTypeInfo<'static> = DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME));
}

impl ClassData for __DWidget {}

impl<__Self: ClassConcrete + RcRef + IObject> HasData for __SWidget<__Self> {
    type Data = __DWidget;
}
impl<__Self: ClassConcrete + RcRef + IObject> ClassHasData for __SWidget<__Self> {}
impl<__Self: ClassConcrete + RcRef + IObject> HasVtable for __SWidget<__Self> {
    type Vtable = __VWidget<__Self>;
}
impl<__Self: ClassConcrete + RcRef + IObject> ClassHasVtable for __SWidget<__Self> {}
impl<__Self: ClassConcrete + RcRef + IObject> HasVtableImpl for __SWidget<__Self> {
    const VTABLE: Self::Vtable = __SWidget::<__Self>::__VTABLE;
}
impl<__Self: ClassConcrete> ClassHasDyn for __SWidget<__Self> {
    type Dyn = Widget;
}

unsafe impl<__Self: RcRef> RcRef for __SWidget<__Self> {}

pub trait IWidget: RcRef + IObject {
    assert_subclass!(self, Widget);
    fn create_element(&self) -> CRc<Element>;
}

pub type Widget = dyn IWidget;

impl ClassHasDyn for Widget {
    type Dyn = Widget;
}

impl HasData for Widget {
    type Data = __DWidget;
}
impl DynHasVtable for Widget {
    type Vtable<__Self: ClassConcrete> = __VWidget<__Self>;
}
impl ClassConcreteOrDyn for Widget {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME).with_super::<Self>();
}
impl ClassDyn for Widget {
    type Class<__Self: ClassConcrete> = __SWidget<__Self>;
}

impl HasSuper for Widget {
    type Super = Object;
}

impl Deref for Widget {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl<__Self: ClassConcrete + RcRef + IObject> IWidget for __Self {
    fn create_element(&self) -> CRc<Element> {
        read_vtable!(<Widget>::create_element)(self)
    }
}

#[repr(C)]
pub struct __VWidget<__Self: ClassConcrete> {
    __super: CVtable<Object, __Self>,
    __self: PhantomData<__Self>,
    pub create_element: Option<fn(&__Self) -> CRc<Element>>,
}

oop_rs::impl_vtable_copy!(<(__Self: ClassConcrete)> __VWidget<__Self>);

impl<__Self: ClassConcrete> Vtable for __VWidget<__Self> {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME).with_super(<CVtable<Object, __Self> as Vtable>::TYPE),
    );
}

impl<__Self: ClassConcrete> ClassVtable for __VWidget<__Self> {}

impl<__Self: ClassConcrete> HasSuper for __VWidget<__Self> {
    type Super = CVtable<Object, __Self>;
}

impl<__Self: ClassConcrete + RcRef + IObject> __SWidget<__Self> {
    pub const __VTABLE: __VWidget<__Self> = {
        let mut vtable = __VWidget {
            __super: <CClass<Object, __Self> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
            create_element: None,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VWidget<__Self>) {
        write_vtable!((vtable as Downcast).ty = Self::ty);
        write_vtable!((vtable as Downcast).__downcast = Self::__downcast);
    }

    pub fn __default(mut __self: CRcUninit<__Self, Self>) -> CRc<__Self> {
        let __data = __DWidget {};
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, PhantomData);
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        __self.assume_init()
    }
    pub fn ty(__self: &__Self) -> Type {
        Type::of::<Widget>()
    }
    pub fn __downcast(__self: &__Self, ty: Type) -> Option<ErasedPtr> {
        if ty == Type::of::<Widget>() {
            return Some(unsafe { std::mem::transmute(__self as &Widget) });
        }
        None
    }
}

impl Widget {
    fn __data(&self) -> &__DWidget {
        let offset = const { <__DWidget as ClassData>::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
}
