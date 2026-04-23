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
    component_element::{ComponentElement, IComponentElement},
    element::Element,
    state::State,
    stateful_widget::StatefulWidget,
};

const TYPE_NAME: &str = concat!(module_path!(), "::StatefulElement");

#[repr(C)]
pub struct __SStatefulElement<__Self: ClassConcrete> {
    __super: CClass<ComponentElement, __Self>,
    __self: PhantomData<__Self>,
    data: __DStatefulElement,
}

impl<__Self: ClassConcrete + IComponentElement + RcRef> HasData for __SStatefulElement<__Self> {
    type Data = __DStatefulElement;
}

impl<__Self: ClassConcrete + IComponentElement + RcRef> ClassHasData
    for __SStatefulElement<__Self>
{
}

impl<__Self: ClassConcrete + IComponentElement + RcRef> HasVtable for __SStatefulElement<__Self> {
    type Vtable = __VStatefulElement<__Self>;
}
impl<__Self: ClassConcrete + IComponentElement + RcRef> HasSuperVtable
    for __SStatefulElement<__Self>
{
    type SuperVtable = CVtable<ComponentElement, __Self>;
}

impl<__Self: ClassConcrete + IComponentElement + RcRef> ClassHasVtable
    for __SStatefulElement<__Self>
{
}

impl<__Self: ClassConcrete + IComponentElement + RcRef> HasVtableImpl
    for __SStatefulElement<__Self>
{
    const VTABLE: Self::Vtable = __SStatefulElement::<__Self>::__VTABLE;
}

impl<__Self: ClassConcrete + IComponentElement + RcRef> HasSuperVtableImpl
    for __SStatefulElement<__Self>
{
    const SUPER_VTABLE: Self::SuperVtable =
        <CClass<ComponentElement, __Self> as HasVtableImpl>::VTABLE;
}

impl<__Self: ClassConcrete> ClassHasDyn for __SStatefulElement<__Self> {
    type Dyn = StatefulElement;
}

unsafe impl<__Self: ClassConcrete + RcRef> RcRef for __SStatefulElement<__Self> {}

impl<__Self: ClassConcrete> Deref for __SStatefulElement<__Self> {
    type Target = CClass<ComponentElement, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl<__Self: ClassConcrete> HasSuper for __SStatefulElement<__Self> {
    type Super = CClass<ComponentElement, __Self>;
}

pub struct __DStatefulElement {
    state: Cell<Option<CRc<State>>>,
}

impl HasSuper for __DStatefulElement {
    type Super = CData<Element>;
}

impl ClassOrMixinData for __DStatefulElement {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<ComponentElement> as ClassData>::CLASS_TYPE,
            offset_of!(__SStatefulElement<Dummy>, data),
        ));
}

impl ClassData for __DStatefulElement {}

pub trait IStatefulElement: IComponentElement {
    assert_subclass!(self, __CStatefulElement);
    fn state(&self) -> CRc<State>;
}

impl ClassConcreteOrDyn for StatefulElement {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME).with_super::<Self>();
}

impl HasData for StatefulElement {
    type Data = __DStatefulElement;
}

impl ClassHasData for StatefulElement {}

impl DynHasVtable for StatefulElement {
    type Vtable<__Self: ClassConcrete> = __VStatefulElement<__Self>;
}

impl ClassDyn for StatefulElement {
    type Class<__Self: ClassConcrete> = __SStatefulElement<__Self>;
}

impl HasSuper for StatefulElement {
    type Super = ComponentElement;
}

impl Deref for StatefulElement {
    type Target = ComponentElement;
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl<__Self: ClassConcrete + IComponentElement + RcRef> IStatefulElement for __Self {
    fn state(&self) -> CRc<State> {
        read_vtable!(<StatefulElement>::state)(self)
    }
}

#[repr(C)]
pub struct __VStatefulElement<__Self: ClassConcrete> {
    __super: CVtable<ComponentElement, __Self>,
    __self: PhantomData<__Self>,
    state: Option<fn(&__Self) -> CRc<State>>,
}

oop_rs::impl_vtable_copy!(<(__Self: ClassConcrete)> __VStatefulElement<__Self>);

impl<__Self: ClassConcrete> Vtable for __VStatefulElement<__Self> {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME)
            .with_super(<CVtable<ComponentElement, __Self> as Vtable>::TYPE),
    );
}

impl<__Self: ClassConcrete> ClassVtable for __VStatefulElement<__Self> {}

impl<__Self: ClassConcrete + IComponentElement + RcRef> __SStatefulElement<__Self> {
    const __VTABLE: __VStatefulElement<__Self> = {
        let mut vtable = __VStatefulElement {
            __super: <CClass<ComponentElement, __Self> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
            state: None,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VStatefulElement<__Self>) {
        write_vtable!((vtable as ComponentElement).build = Self::build);
        write_vtable!((vtable as ComponentElement).first_build = Self::first_build);
        write_vtable!((vtable as Element).perform_rebuild = Self::perform_rebuild);
        write_vtable!((vtable as Element).mount = Self::mount);
        write_vtable!(vtable.state = Self::state);
    }

    pub fn __new(mut __self: CRcUninit<__Self, Self>, widget: CRc<StatefulWidget>) -> CRc<__Self> {
        let __data = __DStatefulElement {
            state: Cell::new(Some(widget.create_state())),
        };
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, PhantomData);
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        let __self = <<Self as HasSuper>::Super>::__new(__self, widget);
        println!("StatefulElement");
        __self
    }

    fn state(__self: &__Self) -> CRc<State> {
        let __self = __self as &StatefulElement;
        __self.get().state().unwrap()
    }
    fn build(__self: &__Self) {
        __self.state().build(__self);
    }
    fn first_build(__self: &__Self) {
        println!("StatefulElement::first_build");
        __self.state().init_state();
        read_vtable!(<Super as ComponentElement>::first_build)(__self);
    }
    fn perform_rebuild(__self: &__Self) {
        println!("StatefulElement::perform_rebuild");
        read_vtable!(<Super as Element>::perform_rebuild)(__self);
    }
    fn mount(__self: &__Self, parent: Option<&Element>) {
        println!("StatefulElement::mount");
        read_vtable!(<Super as Element>::mount)(__self, parent);
    }
}

#[repr(transparent)]
pub struct __CStatefulElement(__SStatefulElement<Self>);

impl Deref for __CStatefulElement {
    type Target = __SStatefulElement<Self>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ClassHasDyn for __CStatefulElement {
    type Dyn = StatefulElement;
}
impl HasData for __CStatefulElement {
    type Data = __DStatefulElement;
}

impl ClassHasData for __CStatefulElement {}

impl HasVtable for __CStatefulElement {
    type Vtable = __VStatefulElement<Self>;
}

impl ClassHasVtable for __CStatefulElement {}

impl HasVtableImpl for __CStatefulElement {
    const VTABLE: Self::Vtable = __SStatefulElement::<Self>::__VTABLE;
}

impl ClassConcreteOrDyn for __CStatefulElement {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME).with_super::<Self>();
}

impl ClassConcrete for __CStatefulElement {}

impl HasSuper for __CStatefulElement {
    type Super = ComponentElement;
}

unsafe impl RcRef for __CStatefulElement {}

pub type StatefulElement = dyn IStatefulElement;

impl ClassHasDyn for StatefulElement {
    type Dyn = StatefulElement;
}

impl StatefulElement {
    pub fn new(widget: CRc<StatefulWidget>) -> CRc<__CStatefulElement> {
        __SStatefulElement::__new(CRcUninit::new().cast_uninit(), widget)
    }
}

impl StatefulElement {
    fn __data(&self) -> &__DStatefulElement {
        let offset = const { <__DStatefulElement as ClassData>::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }

    fn __data_mut(&mut self) -> &mut __DStatefulElement {
        let offset = const { <__DStatefulElement as ClassData>::CLASS_TYPE.offset() };
        unsafe { &mut *core::ptr::from_mut(self).byte_add(offset).cast() }
    }
}

#[repr(transparent)]
pub struct __AStatefulElement<Mode: AccessMode>(PhantomData<Mode>, StatefulElement);

unsafe impl Access for StatefulElement {
    type Accessor<Mode: AccessMode> = __AStatefulElement<Mode>;
}

impl __AStatefulElement<Get> {
    pub fn state(&self) -> Option<CRc<State>> {
        self.1.__data().state.get_cloned()
    }
}

impl __AStatefulElement<Set> {
    pub fn state(&self, state: Option<CRc<State>>) {
        self.1.__data().state.set(state);
    }
}

impl<Mode: AccessMode> Deref for __AStatefulElement<Mode> {
    type Target = <ComponentElement as Access>::Accessor<Mode>;
    fn deref(&self) -> &Self::Target {
        (&self.1 as &ComponentElement).__access::<Mode>()
    }
}
