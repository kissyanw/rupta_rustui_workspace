use std::{marker::PhantomData, mem::offset_of, ops::Deref};

use oop_rs::{
    __private::{
        Dummy,
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        downcast::{Downcast, ErasedPtr},
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn},
        generic::{Generic, TypeInfo},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, DynHasVtable, HasSuperVtable,
            HasSuperVtableImpl, HasVtable, HasVtableImpl, Type, Vtable, VtableTypeInfo,
        },
    },
    assert_subclass,
    class::{ClassConcrete, HasSuper},
    prelude::{CClass, CData, CVtable},
    rc::{CRc, CRcUninit, RcRef, RcRefImpl},
    read_vtable, write_vtable,
};

use crate::{
    element::Element,
    state::State,
    stateful_element::StatefulElement,
    widget::{IWidget, Widget},
};

const TYPE_NAME: &str = concat!(module_path!(), "::StatefulWidget");

#[repr(C)]
pub struct __SStatefulWidget<__Self: ClassConcrete> {
    __super: CClass<Widget, __Self>,
    __self: PhantomData<__Self>,
    data: __DStatefulWidget,
}
pub struct __DStatefulWidget {}

impl HasSuper for __DStatefulWidget {
    type Super = CData<Widget>;
}

impl ClassOrMixinData for __DStatefulWidget {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<Widget> as ClassData>::CLASS_TYPE,
            offset_of!(__SStatefulWidget<Dummy>, __super),
        ));
}

impl ClassData for __DStatefulWidget {}

impl<__Self: ClassConcrete + IWidget> HasData for __SStatefulWidget<__Self> {
    type Data = __DStatefulWidget;
}
impl<__Self: ClassConcrete + IWidget> ClassHasData for __SStatefulWidget<__Self> {}
impl<__Self: ClassConcrete + IWidget> HasVtable for __SStatefulWidget<__Self> {
    type Vtable = __VStatefulWidget<__Self>;
}
impl<__Self: ClassConcrete + IWidget> ClassHasVtable for __SStatefulWidget<__Self> {}
impl<__Self: ClassConcrete + IWidget + RcRef> HasVtableImpl for __SStatefulWidget<__Self> {
    const VTABLE: Self::Vtable = __SStatefulWidget::<__Self>::__VTABLE;
}
impl<__Self: ClassConcrete + IWidget> HasSuperVtable for __SStatefulWidget<__Self> {
    type SuperVtable = CVtable<Widget, __Self>;
}
impl<__Self: ClassConcrete + IWidget + RcRef> HasSuperVtableImpl for __SStatefulWidget<__Self> {
    const SUPER_VTABLE: Self::SuperVtable = <CClass<Widget, __Self> as HasVtableImpl>::VTABLE;
}
impl<__Self: ClassConcrete> ClassHasDyn for __SStatefulWidget<__Self> {
    type Dyn = StatefulWidget;
}

unsafe impl<__Self: ClassConcrete + RcRef> RcRef for __SStatefulWidget<__Self> {}

impl<__Self: ClassConcrete> Deref for __SStatefulWidget<__Self> {
    type Target = CClass<Widget, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl<__Self: ClassConcrete> HasSuper for __SStatefulWidget<__Self> {
    type Super = CClass<Widget, __Self>;
}

pub trait IStatefulWidget: IWidget {
    assert_subclass!(self, StatefulWidget);
    fn create_state(&self) -> CRc<State>;
}

pub type StatefulWidget = dyn IStatefulWidget;

impl DynHasVtable for StatefulWidget {
    type Vtable<__Self: ClassConcrete> = __VStatefulWidget<__Self>;
}

impl ClassDyn for StatefulWidget {
    type Class<__Self: ClassConcrete> = __SStatefulWidget<__Self>;
}

impl ClassConcreteOrDyn for StatefulWidget {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME).with_super::<Self>();
}

impl HasSuper for StatefulWidget {
    type Super = Widget;
}

impl HasData for StatefulWidget {
    type Data = __DStatefulWidget;
}

impl ClassHasData for StatefulWidget {}

unsafe impl Generic for StatefulWidget {
    const TYPE: &TypeInfo = &TypeInfo::Class(<StatefulWidget as ClassConcreteOrDyn>::TYPE);
}

impl<__Self: ClassConcrete + IWidget + RcRef> IStatefulWidget for __Self {
    fn create_state(&self) -> CRc<State> {
        read_vtable!(<StatefulWidget>::create_state)(self)
    }
}

#[repr(C)]
pub struct __VStatefulWidget<__Self: ClassConcrete> {
    __super: CVtable<Widget, __Self>,
    __self: PhantomData<__Self>,
    pub create_state: Option<fn(&__Self) -> CRc<State>>,
}

oop_rs::impl_vtable_copy!(<(__Self: ClassConcrete)> __VStatefulWidget<__Self>);

impl<__Self: ClassConcrete> Vtable for __VStatefulWidget<__Self> {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME).with_super(<CVtable<Widget, __Self> as Vtable>::TYPE),
    );
}

impl<__Self: ClassConcrete> ClassVtable for __VStatefulWidget<__Self> {}

impl<__Self: ClassConcrete> HasSuper for __VStatefulWidget<__Self> {
    type Super = CVtable<Widget, __Self>;
}

impl<__Self: ClassConcrete + IWidget + RcRef> __SStatefulWidget<__Self> {
    const __VTABLE: __VStatefulWidget<__Self> = {
        let mut vtable = __VStatefulWidget {
            __super: <CClass<Widget, __Self> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
            create_state: None,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VStatefulWidget<__Self>) {
        write_vtable!((vtable as Downcast).ty = Self::ty);
        write_vtable!((vtable as Widget).create_element = Self::create_element as _);
        write_vtable!((vtable as Downcast).__downcast = Self::__downcast as _);
    }

    pub fn __default(mut __self: CRcUninit<__Self, Self>) -> CRc<__Self> {
        let __data = __DStatefulWidget {};
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, PhantomData);
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        CClass::<Widget, __Self>::__default(__self)
    }
    pub fn create_element(__self: &__Self) -> CRc<Element> {
        StatefulElement::new(__self.to_rc())
    }
    pub fn ty(__self: &__Self) -> Type {
        Type::of::<StatefulWidget>()
    }
    pub fn __downcast(__self: &__Self, ty: Type) -> Option<ErasedPtr> {
        if ty == Type::of::<StatefulWidget>() {
            return Some(unsafe { std::mem::transmute(__self as &StatefulWidget) });
        }
        if let Some(__super) = read_vtable!(<Super as Downcast>::__downcast)(__self, ty) {
            return Some(__super);
        }
        None
    }
}

impl StatefulWidget {
    fn __data(&self) -> &__DStatefulWidget {
        let offset = const { <__DStatefulWidget as ClassData>::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
}
