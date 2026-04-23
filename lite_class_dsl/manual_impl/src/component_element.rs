use std::{cell::Cell, marker::PhantomData, mem::offset_of, ops::Deref};

use oop_rs::{
    __private::{
        Dummy,
        accessor::{Access, AccessMode, Get, Set},
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, DynHasVtable, HasSuperVtable,
            HasSuperVtableImpl, HasVtable, HasVtableImpl, Vtable, VtableTypeInfo,
        },
    },
    assert_subclass,
    cell::CellGetCloned,
    class::{ClassConcrete, HasSuper},
    prelude::{CClass, CData, CVtable},
    rc::{CRc, CRcUninit, RcRef},
    read_vtable, write_vtable,
};

use crate::{
    element::{Element, IElement},
    widget::Widget,
};

const TYPE_NAME: &str = concat!(module_path!(), "::ComponentElement");

#[repr(C)]
pub struct __SComponentElement<__Self: ClassConcrete> {
    __super: CClass<Element, __Self>,
    __self: PhantomData<__Self>,
    data: __DComponentElement,
}

impl<__Self: ClassConcrete + IElement + RcRef> HasData for __SComponentElement<__Self> {
    type Data = __DComponentElement;
}

impl<__Self: ClassConcrete + IElement + RcRef> ClassHasData for __SComponentElement<__Self> {}

impl<__Self: ClassConcrete + IElement + RcRef> HasVtable for __SComponentElement<__Self> {
    type Vtable = __VComponentElement<__Self>;
}
impl<__Self: ClassConcrete + IElement + RcRef> ClassHasVtable for __SComponentElement<__Self> {}
impl<__Self: ClassConcrete + IElement + RcRef> HasVtableImpl for __SComponentElement<__Self> {
    const VTABLE: Self::Vtable = __SComponentElement::<__Self>::__VTABLE;
}
impl<__Self: ClassConcrete + IElement + RcRef> HasSuperVtable for __SComponentElement<__Self> {
    type SuperVtable = CVtable<Element, __Self>;
}
impl<__Self: ClassConcrete + IElement + RcRef> HasSuperVtableImpl for __SComponentElement<__Self> {
    const SUPER_VTABLE: Self::SuperVtable = <CClass<Element, __Self> as HasVtableImpl>::VTABLE;
}

impl<__Self: ClassConcrete> ClassHasDyn for __SComponentElement<__Self> {
    type Dyn = ComponentElement;
}

impl<__Self: ClassConcrete> Deref for __SComponentElement<__Self> {
    type Target = CClass<Element, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl<__Self: ClassConcrete> HasSuper for __SComponentElement<__Self> {
    type Super = CClass<Element, __Self>;
}

pub struct __DComponentElement {
    child: Cell<Option<CRc<Element>>>,
}

impl ClassOrMixinData for __DComponentElement {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<Element> as ClassData>::CLASS_TYPE,
            offset_of!(__SComponentElement<Dummy>, __super),
        ));
}

impl ClassData for __DComponentElement {}

impl HasSuper for __DComponentElement {
    type Super = CData<Element>;
}

unsafe impl<__Self: ClassConcrete + RcRef> RcRef for __SComponentElement<__Self> {}

pub trait IComponentElement: IElement {
    assert_subclass!(self, ComponentElement);
    fn first_build(&self);
    fn build(&self);
}

pub type ComponentElement = dyn IComponentElement;

impl ClassHasDyn for ComponentElement {
    type Dyn = ComponentElement;
}

impl HasData for ComponentElement {
    type Data = __DComponentElement;
}
impl ClassHasData for ComponentElement {}
impl DynHasVtable for ComponentElement {
    type Vtable<__Self: ClassConcrete> = __VComponentElement<__Self>;
}
impl ClassConcreteOrDyn for ComponentElement {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME).with_super::<Self>();
}
impl ClassDyn for ComponentElement {
    type Class<__Self: ClassConcrete> = __SComponentElement<__Self>;
}
impl HasSuper for ComponentElement {
    type Super = Element;
}

impl Deref for ComponentElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl<__Self: ClassConcrete + IElement + RcRef> IComponentElement for __Self {
    fn first_build(&self) {
        read_vtable!(<ComponentElement>::first_build)(self);
    }
    fn build(&self) {
        read_vtable!(<ComponentElement>::build)(self)
    }
}

#[repr(C)]
pub struct __VComponentElement<__Self: ClassConcrete> {
    __super: CVtable<Element, __Self>,
    __self: PhantomData<__Self>,
    pub first_build: Option<fn(&__Self)>,
    pub build: Option<fn(&__Self)>,
}

oop_rs::impl_vtable_copy!(<(__Self: ClassConcrete)> __VComponentElement<__Self>);

impl<__Self: ClassConcrete> Vtable for __VComponentElement<__Self> {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME).with_super(<CVtable<Element, __Self> as Vtable>::TYPE),
    );
}

impl<__Self: ClassConcrete> ClassVtable for __VComponentElement<__Self> {}

impl<__Self: ClassConcrete + IElement + RcRef> __SComponentElement<__Self> {
    pub const __VTABLE: __VComponentElement<__Self> = {
        let mut vtable = __VComponentElement {
            __super: <CClass<Element, __Self> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
            first_build: None,
            build: None,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VComponentElement<__Self>) {
        write_vtable!((vtable as Element).mount = Self::mount);
        write_vtable!((vtable as Element).perform_rebuild = Self::perform_rebuild);
        write_vtable!(vtable.first_build = Self::first_build);
    }

    pub fn __new(mut __self: CRcUninit<__Self, Self>, widget: CRc<Widget>) -> CRc<__Self> {
        let __data = __DComponentElement {
            child: Cell::new(None),
        };
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, PhantomData);
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        let __self = CClass::<Element, __Self>::new(__self, widget);
        println!("ComponentElement");
        __self
    }
    fn first_build(__self: &__Self) {
        println!("ComponentElement::first_build");
        __self.rebuild(false);
    }
    fn perform_rebuild(__self: &__Self) {
        println!("ComponentElement::perform_rebuild");
        __self.build();
        read_vtable!(<Super as Element>::perform_rebuild)(__self);
    }
    fn mount(__self: &__Self, parent: Option<&Element>) {
        println!("ComponentElement::mount");
        read_vtable!(<Super as Element>::mount)(__self, parent);
        __self.first_build();
    }
}

impl ComponentElement {
    fn __data(&self) -> &__DComponentElement {
        let offset = const { <__DComponentElement as ClassData>::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

#[repr(transparent)]
pub struct __AComponentElement<Mode: AccessMode>(PhantomData<Mode>, ComponentElement);

unsafe impl Access for ComponentElement {
    type Accessor<Mode: AccessMode> = __AComponentElement<Mode>;
}

impl __AComponentElement<Get> {
    pub fn child(&self) -> Option<CRc<Element>> {
        self.1.__data().child.get_cloned()
    }
}

impl __AComponentElement<Set> {
    pub fn child(&self, child: Option<CRc<Element>>) {
        self.1.__data().child.set(child);
    }
}

impl<Mode: AccessMode> Deref for __AComponentElement<Mode> {
    type Target = <Element as Access>::Accessor<Mode>;
    fn deref(&self) -> &Self::Target {
        (&self.1 as &Element).__access::<Mode>()
    }
}
