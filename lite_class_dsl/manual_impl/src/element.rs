use std::{cell::Cell, marker::PhantomData, mem::offset_of};

use oop_rs::{
    __private::{
        accessor::{Access, AccessMode, Get, Set},
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        dynamic::{AsDyn, ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, DynHasVtable, HasVtable,
            HasVtableImpl, Vtable, VtableOffsetEntry, VtableTypeInfo,
        },
    },
    assert_subclass,
    cell::CellGetCloned,
    class::ClassConcrete,
    prelude::{CClass, CVtable},
    rc::{CRc, CRcUninit, CWeak, RcRef, RcRefImpl},
    read_vtable, write_vtable,
};

use crate::{
    build_context::{BuildContext, IBuildContext},
    widget::Widget,
};

const TYPE_NAME: &str = concat!(module_path!(), "::Element");

#[repr(C)]
pub struct __SElement<__Self> {
    __self: PhantomData<__Self>,
    data: __DElement,
}

impl<__Self: ClassConcrete + IBuildContext + RcRef> HasData for __SElement<__Self> {
    type Data = __DElement;
}
impl<__Self: ClassConcrete + IBuildContext + RcRef> ClassHasData for __SElement<__Self> {}
impl<__Self: ClassConcrete + IBuildContext + RcRef> HasVtable for __SElement<__Self> {
    type Vtable = __VElement<__Self>;
}
impl<__Self: ClassConcrete + IBuildContext + RcRef> ClassHasVtable for __SElement<__Self> {}
impl<__Self: ClassConcrete + IBuildContext + RcRef> HasVtableImpl for __SElement<__Self> {
    const VTABLE: Self::Vtable = __SElement::<__Self>::__VTABLE;
}

impl<__Self: ClassConcrete> ClassHasDyn for __SElement<__Self> {
    type Dyn = Element;
}

unsafe impl<__Self: ClassConcrete + RcRef> RcRef for __SElement<__Self> {}

pub struct __DElement {
    parent: Cell<Option<CWeak<Element>>>,
    widget: Cell<Option<CRc<Widget>>>,
    dirty: Cell<bool>,
}

impl ClassOrMixinData for __DElement {
    const TYPE: DataTypeInfo<'static> = DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME));
}

impl ClassData for __DElement {}

pub trait IElement: IBuildContext + RcRef {
    assert_subclass!(self, Element);
    fn rebuild(&self, force: bool);
    fn perform_rebuild(&self);
    fn mount(&self, parent: Option<&Element>);
    fn mark_needs_build(&self);
}

pub type Element = dyn IElement;

impl HasData for Element {
    type Data = __DElement;
}
impl ClassHasData for Element {}
impl DynHasVtable for Element {
    type Vtable<__Self: ClassConcrete> = __VElement<__Self>;
}
impl ClassConcreteOrDyn for Element {
    const TYPE: &ClassConcreteOrDynTypeInfo = &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME);
}
impl ClassDyn for Element {
    type Class<__Self: ClassConcrete> = __SElement<__Self>;
}

impl<__Self: ClassConcrete + IBuildContext + RcRef> IElement for __Self {
    fn rebuild(&self, force: bool) {
        read_vtable!(<Element>::rebuild)(self, force)
    }
    fn perform_rebuild(&self) {
        read_vtable!(<Element>::perform_rebuild)(self)
    }
    fn mount(&self, parent: Option<&Element>) {
        read_vtable!(<Element>::mount)(self, parent)
    }
    fn mark_needs_build(&self) {
        read_vtable!(<Element>::mark_needs_build)(self)
    }
}

impl<__Self: ClassConcrete + IBuildContext + RcRef> AsDyn for __SElement<__Self> {
    fn as_dyn(&self) -> &Self::Dyn {
        unsafe { &*core::ptr::from_ref(self).cast::<__Self>() as &Element }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct __VElement<__Self: ClassConcrete> {
    __self: PhantomData<__Self>,
    __BuildContext: CVtable<BuildContext, __Self>,
    pub rebuild: Option<fn(&__Self, bool)>,
    pub perform_rebuild: Option<fn(&__Self)>,
    pub mount: Option<fn(&__Self, Option<&Element>)>,
    pub mark_needs_build: Option<fn(&__Self)>,
}

oop_rs::impl_vtable_copy!(<(__Self: ClassConcrete)> __VElement<__Self>);

impl<__Self: ClassConcrete> __VElement<__Self> {
    pub const DEFAULT: Self = Self {
        __self: PhantomData,
        __BuildContext: <CClass<BuildContext, __Self> as HasVtableImpl>::VTABLE,
        rebuild: None,
        perform_rebuild: None,
        mount: None,
        mark_needs_build: None,
    };
}

impl<__Self: ClassConcrete> Vtable for __VElement<__Self> {
    const TYPE: VtableTypeInfo<'static> =
        VtableTypeInfo::Class(&ClassVtableTypeInfo::new(TYPE_NAME).with_interfaces(&[
            VtableOffsetEntry::new(
                CVtable::<BuildContext, __Self>::INTERFACE_TYPE,
                offset_of!(Self, __BuildContext),
            ),
        ]));
}

impl<__Self: ClassConcrete> ClassVtable for __VElement<__Self> {}

impl<__Self: ClassConcrete + IBuildContext + RcRef> __SElement<__Self> {
    pub const __VTABLE: __VElement<__Self> = {
        let mut vtable = __VElement::<__Self>::DEFAULT;
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VElement<__Self>) {
        write_vtable!((vtable as BuildContext).widget = Self::widget);
        write_vtable!(vtable.mark_needs_build = Self::mark_needs_build);
        write_vtable!(vtable.mount = Self::mount);
        write_vtable!(vtable.perform_rebuild = Self::perform_rebuild);
        write_vtable!(vtable.rebuild = Self::rebuild);
    }

    pub fn new(mut __self: CRcUninit<__Self, Self>, widget: CRc<Widget>) -> CRc<__Self> {
        let __data = __DElement {
            parent: Cell::new(None),
            widget: Cell::new(Some(widget)),
            dirty: Cell::new(true),
        };
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, PhantomData);
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        let __self = __self.assume_init();
        println!("Element");
        __self
    }
    fn widget(__self: &__Self) -> CRc<Widget> {
        let __self = __self as &Element;
        __self.get().widget().unwrap()
    }
    fn rebuild(__self: &__Self, force: bool) {
        let __self = __self as &Element;
        println!("Element::rebuild");
        if __self.get().dirty() || force {
            __self.perform_rebuild();
        }
    }
    fn perform_rebuild(__self: &__Self) {
        let __self = __self as &Element;
        println!("Element::perform_rebuild");
        __self.set().dirty(false);
    }
    fn mount(__self: &__Self, parent: Option<&Element>) {
        let __self = __self as &Element;
        println!("Element::mount");
        __self.set().parent(parent);
    }
    fn mark_needs_build(__self: &__Self) {
        let __self = __self as &Element;
        println!("Element::mark_needs_build");
        __self.set().dirty(true);
        __self.rebuild(false);
    }
}

impl Element {
    fn __data(&self) -> &__DElement {
        let offset = const { <__DElement as ClassData>::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

#[repr(transparent)]
pub struct __AElement<Mode: AccessMode>(PhantomData<Mode>, Element);

unsafe impl Access for Element {
    type Accessor<Mode: AccessMode> = __AElement<Mode>;
}

impl __AElement<Get> {
    pub fn parent(&self) -> Option<CRc<Element>> {
        self.1
            .__data()
            .parent
            .get_cloned()
            .and_then(|w| w.upgrade())
    }
    pub fn widget(&self) -> Option<CRc<Widget>> {
        self.1.__data().widget.get_cloned()
    }
    pub fn dirty(&self) -> bool {
        self.1.__data().dirty.get()
    }
}

impl __AElement<Set> {
    pub fn parent(&self, parent: Option<&Element>) {
        self.1.__data().parent.set(parent.map(|e| e.to_weak()));
    }
    pub fn widget(&self, widget: Option<CRc<Widget>>) {
        self.1.__data().widget.set(widget);
    }
    pub fn dirty(&self, dirty: bool) {
        self.1.__data().dirty.set(dirty);
    }
}
