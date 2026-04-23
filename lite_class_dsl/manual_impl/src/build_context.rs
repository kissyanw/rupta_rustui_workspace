use std::marker::PhantomData;

use oop_rs::{
    __private::{
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn},
        vtable::{
            DynHasVtable, HasVtable, HasVtableImpl, InterfaceVtableTypeInfo, Vtable, VtableTypeInfo,
        },
    },
    class::ClassConcrete,
    rc::{CRc, RcRef},
    read_vtable,
};

use crate::widget::Widget;

const TYPE_NAME: &str = concat!(module_path!(), "::BuildContext");

#[repr(C)]
pub struct __SBuildContext<__Self> {
    __self: PhantomData<__Self>,
}

pub trait IBuildContext: RcRef {
    fn widget(&self) -> CRc<Widget>;
}

pub type BuildContext = dyn IBuildContext;

impl ClassConcreteOrDyn for dyn IBuildContext {
    const TYPE: &ClassConcreteOrDynTypeInfo = &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME);
}

impl ClassDyn for dyn IBuildContext {
    type Class<__Self: ClassConcrete> = __SBuildContext<__Self>;
}
impl DynHasVtable for dyn IBuildContext {
    type Vtable<__Self: ClassConcrete> = __VBuildContext<__Self>;
}

impl<__Self: ClassConcrete> HasVtable for __SBuildContext<__Self> {
    type Vtable = __VBuildContext<__Self>;
}

impl<__Self: ClassConcrete> HasVtableImpl for __SBuildContext<__Self> {
    const VTABLE: Self::Vtable = __SBuildContext::<__Self>::__VTABLE;
}

impl<__Self: ClassConcrete> ClassHasDyn for __SBuildContext<__Self> {
    type Dyn = BuildContext;
}

impl<__Self: ClassConcrete + RcRef> IBuildContext for __Self {
    fn widget(&self) -> CRc<Widget> {
        read_vtable!(<BuildContext>::widget)(self)
    }
}

#[repr(C)]
pub struct __VBuildContext<__Self> {
    __self: PhantomData<__Self>,
    pub widget: Option<fn(&__Self) -> CRc<Widget>>,
}

oop_rs::impl_vtable_copy!(<(__Self)> __VBuildContext<__Self>);

impl<__Self: ClassConcrete> Vtable for __VBuildContext<__Self> {
    const TYPE: VtableTypeInfo<'static> =
        VtableTypeInfo::Interface(&InterfaceVtableTypeInfo::new(TYPE_NAME));
}

impl<__Self: ClassConcrete> __SBuildContext<__Self> {
    pub const __VTABLE: __VBuildContext<__Self> = {
        let mut vtable = __VBuildContext::<__Self> {
            __self: PhantomData,
            widget: None,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VBuildContext<__Self>) {}
}
