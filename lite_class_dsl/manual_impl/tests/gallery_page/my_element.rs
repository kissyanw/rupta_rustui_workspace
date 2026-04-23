use std::{marker::PhantomData, mem::offset_of, ops::Deref};

use oop_rs::{
    __private::{
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, HasVtable, HasVtableImpl, Vtable,
            VtableTypeInfo,
        },
    },
    class::{ClassConcrete, HasSuper},
    prelude::{CClass, CData, CVtable},
    rc::{CRc, CRcUninit, RcRef},
};

use manual_impl::element::Element;

use crate::my_widget::MyWidget;

const TYPE_NAME: &str = concat!(module_path!(), "::MyElement");
type __Self = MyElement;

#[repr(C)]
pub struct __SMyElement {
    __super: CClass<Element, __Self>,
    data: __DMyElement,
}

pub struct __DMyElement {}

impl ClassOrMixinData for __DMyElement {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<Element> as ClassData>::CLASS_TYPE,
            offset_of!(__SMyElement, __super),
        ));
}

impl ClassData for __DMyElement {}

impl HasData for __SMyElement {
    type Data = __DMyElement;
}

impl ClassHasData for __SMyElement {}

impl HasVtable for __SMyElement {
    type Vtable = __VMyElement;
}

impl ClassHasVtable for __SMyElement {}

impl HasVtableImpl for __SMyElement {
    const VTABLE: Self::Vtable = __SMyElement::__VTABLE;
}

impl HasSuper for __DMyElement {
    type Super = CData<Element>;
}

unsafe impl RcRef for __SMyElement {}

impl Deref for __SMyElement {
    type Target = CClass<Element, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl HasSuper for __SMyElement {
    type Super = CClass<Element, __Self>;
}

pub struct __VMyElement {
    __super: CVtable<Element, __Self>,
    __self: PhantomData<__Self>,
}

oop_rs::impl_vtable_copy!(<()> __VMyElement);

impl Vtable for __VMyElement {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME).with_super(<CVtable<Element, __Self> as Vtable>::TYPE),
    );
}

impl ClassVtable for __VMyElement {}

impl HasSuper for __VMyElement {
    type Super = CVtable<Element, __Self>;
}

impl __SMyElement {
    pub const __VTABLE: __VMyElement = {
        let mut vtable = __VMyElement {
            __super: <CClass<Element, MyElement> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
        };
        Self::__override(&mut vtable);
        vtable
    };
    fn new(mut __self: CRcUninit<__Self, Self>, widget: CRc<MyWidget>) -> CRc<__Self> {
        let __data = __DMyElement {};
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        CClass::<Element, __Self>::new(__self, widget)
    }
    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VMyElement) {}
}

#[repr(transparent)]
pub struct MyElement(__SMyElement);

impl ClassConcreteOrDyn for MyElement {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME).with_super::<MyElement>();
}
impl ClassConcrete for MyElement {}
impl HasData for MyElement {
    type Data = __DMyElement;
}
impl ClassHasData for MyElement {}
impl HasVtable for MyElement {
    type Vtable = __VMyElement;
}
impl HasVtableImpl for MyElement {
    const VTABLE: Self::Vtable = __SMyElement::__VTABLE;
}
impl ClassHasVtable for MyElement {}

unsafe impl RcRef for MyElement {}

impl Deref for MyElement {
    type Target = __SMyElement;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HasSuper for MyElement {
    type Super = Element;
}

impl MyElement {
    pub fn new(widget: CRc<MyWidget>) -> CRc<Self> {
        __SMyElement::new(CRcUninit::new().cast_uninit(), widget)
    }
}
