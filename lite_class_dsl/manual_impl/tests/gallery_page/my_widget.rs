use std::{marker::PhantomData, mem::offset_of, ops::Deref};

use oop_rs::{
    __private::{
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        downcast::{Downcast, ErasedPtr},
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, HasVtable, HasVtableImpl, Type,
            Vtable, VtableTypeInfo,
        },
    },
    class::{ClassConcrete, HasSuper},
    prelude::{CClass, CData, CVtable},
    rc::{CRc, CRcUninit, RcDefault, RcRef, RcRefImpl},
    read_vtable, write_vtable,
};

use manual_impl::{element::Element, widget::Widget};

use crate::my_element::MyElement;

const TYPE_NAME: &str = concat!(module_path!(), "::MyWidget");
type __Self = MyWidget;

#[repr(C)]
pub struct __SMyWidget {
    __super: CClass<Widget, __Self>,
    data: __DMyWidget,
}

pub struct __DMyWidget {}

impl ClassOrMixinData for __DMyWidget {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<Widget> as ClassData>::CLASS_TYPE,
            offset_of!(__SMyWidget, __super),
        ));
}

impl ClassData for __DMyWidget {}

impl HasData for __SMyWidget {
    type Data = __DMyWidget;
}

impl ClassHasData for __SMyWidget {}

impl HasVtable for __SMyWidget {
    type Vtable = __VMyWidget;
}

impl ClassHasVtable for __SMyWidget {}

impl HasVtableImpl for __SMyWidget {
    const VTABLE: Self::Vtable = __SMyWidget::__VTABLE;
}

impl HasSuper for __DMyWidget {
    type Super = CData<Widget>;
}

unsafe impl RcRef for __SMyWidget {}

impl Deref for __SMyWidget {
    type Target = CClass<Widget, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl HasSuper for __SMyWidget {
    type Super = CClass<Widget, __Self>;
}

pub struct __VMyWidget {
    __super: CVtable<Widget, __Self>,
    __self: PhantomData<__Self>,
}

oop_rs::impl_vtable_copy!(<()> __VMyWidget);

impl Vtable for __VMyWidget {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME).with_super(<CVtable<Widget, __Self> as Vtable>::TYPE),
    );
}

impl ClassVtable for __VMyWidget {}

impl HasSuper for __VMyWidget {
    type Super = CVtable<Widget, __Self>;
}

impl __SMyWidget {
    pub const __VTABLE: __VMyWidget = {
        let mut vtable = __VMyWidget {
            __super: <CClass<Widget, MyWidget> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VMyWidget) {
        write_vtable!((vtable as Downcast).ty = __Self::ty);
        write_vtable!((vtable as Downcast).__downcast = __Self::__downcast as _);
        write_vtable!((vtable as Widget).create_element = Self::create_element as _);
    }

    fn __default(mut __self: CRcUninit<__Self, Self>) -> CRc<__Self> {
        let __data = __DMyWidget {};
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        CClass::<Widget, __Self>::__default(__self)
    }

    pub fn create_element(__self: &__Self) -> CRc<Element> {
        MyElement::new(__self.to_rc())
    }
}

#[repr(transparent)]
pub struct MyWidget(__SMyWidget);

unsafe impl RcRef for MyWidget {}

impl HasData for MyWidget {
    type Data = __DMyWidget;
}
impl ClassHasData for MyWidget {}
impl HasVtable for MyWidget {
    type Vtable = __VMyWidget;
}
impl HasVtableImpl for MyWidget {
    const VTABLE: Self::Vtable = __SMyWidget::__VTABLE;
}
impl ClassHasVtable for MyWidget {}
impl ClassConcreteOrDyn for MyWidget {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME).with_super::<Self>();
}
impl ClassConcrete for MyWidget {}
impl HasSuper for MyWidget {
    type Super = Widget;
}

impl MyWidget {
    fn ty(&self) -> Type {
        Type::of::<MyWidget>()
    }
    fn __downcast(&self, ty: Type) -> Option<ErasedPtr> {
        if ty == Type::of::<MyWidget>() {
            return Some(ErasedPtr::new_thin(self.into()));
        }
        read_vtable!(<Downcast>::__downcast)(self, ty)
    }
}

impl RcDefault for MyWidget {
    fn default() -> CRc<Self> {
        __SMyWidget::__default(CRcUninit::new().cast_uninit())
    }
}
