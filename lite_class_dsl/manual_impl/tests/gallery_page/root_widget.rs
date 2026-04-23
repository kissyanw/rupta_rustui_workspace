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
    impl_vtable_copy,
    prelude::{CClass, CData, CVtable},
    rc::{CRc, CRcUninit, RcDefault, RcRef, RcRefImpl},
    read_vtable, write_vtable,
};

use manual_impl::{element::Element, widget::Widget};

use crate::root_element::RootElement;

const TYPE_NAME: &str = concat!(module_path!(), "::RootWidget");
type __Self = RootWidget;

#[repr(C)]
pub struct __SRootWidget {
    __super: CClass<Widget, __Self>,
    data: __DRootWidget,
}

pub struct __DRootWidget {}

impl ClassOrMixinData for __DRootWidget {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<Widget> as ClassData>::CLASS_TYPE,
            offset_of!(__SRootWidget, __super),
        ));
}

impl ClassData for __DRootWidget {}

impl HasData for __SRootWidget {
    type Data = __DRootWidget;
}

impl ClassHasData for __SRootWidget {}

impl HasVtable for __SRootWidget {
    type Vtable = __VRootWidget;
}

impl ClassHasVtable for __SRootWidget {}

impl HasVtableImpl for __SRootWidget {
    const VTABLE: Self::Vtable = __SRootWidget::__VTABLE;
}

impl HasSuper for __DRootWidget {
    type Super = CData<Widget>;
}

unsafe impl RcRef for __SRootWidget {}

impl Deref for __SRootWidget {
    type Target = CClass<Widget, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl HasSuper for __SRootWidget {
    type Super = CClass<Widget, __Self>;
}

pub struct __VRootWidget {
    __super: CVtable<Widget, __Self>,
    __self: PhantomData<__Self>,
}

impl_vtable_copy!(<()> __VRootWidget);

impl Vtable for __VRootWidget {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME).with_super(<CVtable<Widget, __Self> as Vtable>::TYPE),
    );
}

impl ClassVtable for __VRootWidget {}

impl HasSuper for __VRootWidget {
    type Super = CVtable<Widget, __Self>;
}

impl __SRootWidget {
    pub const __VTABLE: __VRootWidget = {
        let mut vtable = __VRootWidget {
            __super: <CClass<Widget, __Self> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VRootWidget) {
        write_vtable!((vtable as Downcast).ty = __Self::ty);
        write_vtable!((vtable as Downcast).__downcast = __Self::__downcast);
        write_vtable!((vtable as Widget).create_element = Self::create_element);
    }

    fn __default(mut __self: CRcUninit<__Self, Self>) -> CRc<__Self> {
        let __data = __DRootWidget {};
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        CClass::<Widget, __Self>::__default(__self)
    }

    pub fn create_element(__self: &__Self) -> CRc<Element> {
        RootElement::new(__self.to_rc())
    }
}

#[repr(transparent)]
pub struct RootWidget(__SRootWidget);

unsafe impl RcRef for RootWidget {}

impl HasData for RootWidget {
    type Data = __DRootWidget;
}
impl ClassHasData for RootWidget {}
impl HasVtable for RootWidget {
    type Vtable = __VRootWidget;
}
impl HasVtableImpl for RootWidget {
    const VTABLE: Self::Vtable = __SRootWidget::__VTABLE;
}
impl ClassHasVtable for RootWidget {}
impl ClassConcrete for RootWidget {}
impl ClassConcreteOrDyn for RootWidget {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME).with_super::<Self>();
}

impl HasSuper for RootWidget {
    type Super = Widget;
}

impl RootWidget {
    fn ty(&self) -> Type {
        Type::of::<RootWidget>()
    }
    fn __downcast(&self, ty: Type) -> Option<ErasedPtr> {
        if ty == Type::of::<RootWidget>() {
            return Some(ErasedPtr::new_thin(self.into()));
        }
        read_vtable!(<Downcast>::__downcast)(self, ty)
    }
}

impl RcDefault for RootWidget {
    fn default() -> CRc<Self> {
        __SRootWidget::__default(CRcUninit::new().cast_uninit())
    }
}
