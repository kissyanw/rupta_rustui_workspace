use std::{cell::Cell, marker::PhantomData, };

use oop_rs::{
    __private::{
        accessor::{Access, AccessMode, Get, Set},
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        downcast::{Downcast, IDowncastImpl},
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn, Dyn},
        generic::{Generic, TypeInfo},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, DynHasVtable, HasVtable,
            HasVtableImpl, Vtable, VtableTypeInfo,
        },
    },
    cell::CellGetCloned,
    class::{ClassConcrete},
    def_unsize, impl_unsize,
    rc::{CRc, CWeak, RcRef},
    read_vtable, write_vtable,
};

use crate::{
    build_context::BuildContext,
    element::Element,
    stateful_widget::{IStatefulWidget, StatefulWidget},
    widget::Widget,
};

const TYPE_NAME: &str = concat!(module_path!(), "::State");

#[repr(C)]
pub struct __SState<__Self, T: ?Sized = StatefulWidget> {
    __self: PhantomData<__Self>,
    data: __DState<T>,
}

impl<__Self: ClassConcrete, T: Generic + IStatefulWidget> HasData for __SState<__Self, T> {
    type Data = __DState<T>;
}
impl<__Self: ClassConcrete, T: Generic + IStatefulWidget> ClassHasData for __SState<__Self, T> {}

impl<__Self: ClassConcrete + RcRef, T: Generic + IStatefulWidget> HasVtable
    for __SState<__Self, T>
{
    type Vtable = __VState<__Self, T>;
}
impl<__Self: ClassConcrete + RcRef, T: Generic + IStatefulWidget> ClassHasVtable
    for __SState<__Self, T>
{
}
impl<__Self: ClassConcrete + RcRef, T: Generic + IStatefulWidget> HasVtableImpl
    for __SState<__Self, T>
{
    const VTABLE: Self::Vtable = __SState::<__Self, T>::__VTABLE;
}
impl<__Self: ClassConcrete, T: Generic + IStatefulWidget> ClassHasDyn for __SState<__Self, T> {
    type Dyn = StateT<T>;
}

impl<__Self: ClassConcrete> HasData for __SState<__Self> {
    type Data = __DState;
}

impl<__Self: ClassConcrete> ClassHasData for __SState<__Self> {}

impl<__Self: ClassConcrete> HasVtable for __SState<__Self> {
    type Vtable = __VState<__Self>;
}
impl<__Self: ClassConcrete> ClassHasVtable for __SState<__Self> {}
impl<__Self: ClassConcrete + RcRef> HasVtableImpl for __SState<__Self> {
    const VTABLE: Self::Vtable = __SState::<__Self>::__VTABLE;
}
impl<__Self: ClassConcrete> ClassHasDyn for __SState<__Self> {
    type Dyn = State;
}

pub struct __DState<T: ?Sized = StatefulWidget> {
    widget: Cell<Option<CRc<Dyn<StatefulWidget, T>>>>,
    element: Cell<Option<CRc<Element>>>,
}

impl<T: IStatefulWidget + Generic> ClassOrMixinData for __DState<T> {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_generics(&[T::TYPE]));
}

impl<T: IStatefulWidget + Generic> ClassData for __DState<T> {}

impl ClassOrMixinData for __DState {
    const TYPE: DataTypeInfo<'static> = DataTypeInfo::Class(
        &ClassDataTypeInfo::new(TYPE_NAME).with_generics(&[&TypeInfo::Class(
            <StatefulWidget as ClassConcreteOrDyn>::TYPE,
        )]),
    );
}

impl ClassData for __DState {}

unsafe impl<__Self: RcRef, T: 'static> RcRef for __SState<__Self, T> {}

pub trait IState: RcRef {
    fn widget(&self) -> CRc<StatefulWidget>;
    fn init_state(&self);
    fn build(&self, cx: &BuildContext) -> CRc<Widget>;
    fn set_state(&self, f: &mut dyn FnMut());
}

pub type State = dyn IState;

impl HasData for State {
    type Data = __DState;
}
impl ClassHasData for State {}

impl DynHasVtable for State {
    type Vtable<__Self: ClassConcrete> = __VState<__Self>;
}

impl ClassConcreteOrDyn for State {
    const TYPE: &ClassConcreteOrDynTypeInfo = &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME);
}

impl ClassDyn for State {
    type Class<__Self: ClassConcrete> = __SState<__Self>;
}

pub trait __IStateT<T: IStatefulWidget + ?Sized = StatefulWidget>: RcRef {
    fn widget(&self) -> CRc<T>;
    fn init_state(&self);
    fn build(&self, cx: &BuildContext) -> CRc<Widget>;
    fn set_state(&self, f: &mut dyn FnMut());
}

pub trait IStateT: RcRef {
    type T: IStatefulWidget + Unsize<StatefulWidget> + ?Sized;
}

pub type StateT<T = StatefulWidget> = dyn __IStateT<T>;

impl<T: IStatefulWidget + Generic> HasData for StateT<T> {
    type Data = __DState<T>;
}

impl HasData for StateT {
    type Data = __DState;
}

impl<T: IStatefulWidget + Generic + ?Sized> ClassConcreteOrDyn for StateT<T> {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_dyn(TYPE_NAME).with_generics(&[T::TYPE]);
}

impl<T: IStatefulWidget + Generic> ClassDyn for StateT<T> {
    type Class<__Self: ClassConcrete> = __SState<__Self, T>;
}

impl<T: IStatefulWidget + Generic + ?Sized> DynHasVtable for StateT<T> {
    type Vtable<__Self: ClassConcrete> = __VState<__Self, T>;
}

def_unsize!();
impl_unsize!(IStatefulWidget);

impl<__Self: ClassConcrete + RcRef, T: IStatefulWidget + Generic> __IStateT<T> for __Self {
    fn widget(&self) -> CRc<T> {
        read_vtable!(<StateT<T>>::widget)(self)
    }
    fn init_state(&self) {
        read_vtable!(<StateT<T>>::init_state)(self)
    }
    fn build(&self, cx: &BuildContext) -> CRc<Widget> {
        read_vtable!(<StateT<T>>::build)(self, cx)
    }
    fn set_state(&self, f: &mut dyn FnMut()) {
        read_vtable!(<StateT<T>>::set_state)(self, f)
    }
}

impl<__Self: ClassConcrete + RcRef> __IStateT for __Self {
    fn widget(&self) -> CRc<StatefulWidget> {
        read_vtable!(<StateT>::widget)(self)
    }
    fn init_state(&self) {
        read_vtable!(<StateT>::init_state)(self)
    }
    fn build(&self, cx: &BuildContext) -> CRc<Widget> {
        read_vtable!(<StateT>::build)(self, cx)
    }
    fn set_state(&self, f: &mut dyn FnMut()) {
        read_vtable!(<StateT>::set_state)(self, f)
    }
}

impl<__Self: IStateT<T: IStatefulWidget> + __IStateT<__Self::T>> IState for __Self {
    fn widget(&self) -> CRc<StatefulWidget> {
        __IStateT::<__Self::T>::widget(self).unsize()
    }

    fn init_state(&self) {
        __IStateT::<__Self::T>::init_state(self)
    }

    fn build(&self, cx: &BuildContext) -> CRc<Widget> {
        __IStateT::<__Self::T>::build(self, cx)
    }

    fn set_state(&self, f: &mut dyn FnMut()) {
        __IStateT::<__Self::T>::set_state(self, f)
    }
}

#[repr(C)]
pub struct __VState<__Self, T: ?Sized = StatefulWidget> {
    __self: PhantomData<__Self>,
    pub widget: Option<fn(&__Self) -> CRc<T>>,
    pub init_state: Option<fn(&__Self)>,
    pub build: Option<fn(&__Self, &BuildContext) -> CRc<Widget>>,
    pub set_state: Option<fn(&__Self, &mut dyn FnMut())>,
}

oop_rs::impl_vtable_copy!(<(__Self, T: ?Sized)> __VState<__Self, T>);

impl<__Self, T: ?Sized> __VState<__Self, T> {
    pub const DEFAULT: Self = Self {
        __self: PhantomData,
        widget: None,
        init_state: None,
        build: None,
        set_state: None,
    };
}

impl<__Self, T: Generic + ?Sized> Vtable for __VState<__Self, T> {
    const TYPE: VtableTypeInfo<'static> =
        VtableTypeInfo::Class(&ClassVtableTypeInfo::new(TYPE_NAME).with_generics(&[T::TYPE]));
}

impl<__Self, T: Generic + ?Sized> ClassVtable for __VState<__Self, T> {}

impl<__Self: ClassConcrete + RcRef, T: IStatefulWidget + Generic> __SState<__Self, T> {
    pub const __VTABLE: __VState<__Self, T> = {
        let mut vtable = __VState::<__Self, T>::DEFAULT;
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VState<__Self, T>) {
        write_vtable!(vtable.widget = Self::widget);
        write_vtable!(vtable.init_state = Self::init_state);
        write_vtable!(vtable.set_state = Self::set_state);
    }

    pub fn widget(__self: &__Self) -> CRc<T> {
        let __self = __self as &StateT<T>;
        __self.get_widget().unwrap()
    }
    pub fn init_state(__self: &__Self) {
        println!("State::init_state");
    }
    pub fn set_state(__self: &__Self, f: &mut dyn FnMut()) {
        let __self = __self as &StateT<T>;
        println!("State::set_state");
        f();
        __self.get_element().unwrap().mark_needs_build();
    }
}

impl<__Self: ClassConcrete + RcRef> __SState<__Self> {
    pub const __VTABLE: __VState<__Self> = {
        let mut vtable = __VState::<__Self>::DEFAULT;
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VState<__Self>) {
        write_vtable!(vtable.widget = Self::widget);
        write_vtable!(vtable.init_state = Self::init_state);
        write_vtable!(vtable.set_state = Self::set_state);
    }
    pub fn widget(__self: &__Self) -> CRc<StatefulWidget> {
        let __self = __self as &StateT;
        __self.get().widget().unwrap()
    }
    pub fn init_state(__self: &__Self) {
        println!("State::init_state");
    }
    pub fn set_state(__self: &__Self, f: &mut dyn FnMut()) {
        let __self = __self as &StateT;
        println!("State::set_state");
        f();
        __self.get().element().unwrap().mark_needs_build();
    }
}

impl<__Self: ClassConcrete + RcRef, T: IStatefulWidget + Generic + ?Sized> __SState<__Self, T> {
    pub fn __default(__init: impl FnOnce(Self, &CWeak<__Self>) -> __Self) -> CRc<__Self> {
        let __self = CRc::new_cyclic(|self_weak| {
            let __self = Self {
                __self: PhantomData,
                data: __DState {
                    widget: Cell::new(None),
                    element: Cell::new(None),
                },
            };
            __init(__self, self_weak)
        });
        __self
    }
}

impl<T: IStatefulWidget + Generic> StateT<T> {
    fn __data(&self) -> &__DState<T> {
        let offset = const { <StateT<T> as HasData>::Data::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
    pub fn get_widget(&self) -> Option<CRc<T>> {
        self.__data()
            .widget
            .get_cloned()
            .map(|w| (w.into_rc_dyn() as CRc<Downcast>).downcast().ok().unwrap())
    }
    pub fn set_widget(&self, widget: Option<CRc<T>>) {
        self.__data()
            .widget
            .set(widget.map(|w| Dyn::from_rc_dyn(w as CRc<_>)));
    }
    pub fn get_element(&self) -> Option<CRc<Element>> {
        self.__data().element.get_cloned()
    }
    pub fn set_element(&self, element: Option<CRc<Element>>) {
        self.__data().element.set(element);
    }
}

pub struct __AStateT<Mode: AccessMode, T: IStatefulWidget + Generic + ?Sized = StatefulWidget>(
    PhantomData<Mode>,
    StateT<T>,
);
unsafe impl<T: IStatefulWidget + Generic + ?Sized> Access for StateT<T> {
    type Accessor<Mode: AccessMode> = __AStateT<Mode>;
}

impl StateT {
    fn __data(&self) -> &__DState {
        let offset = const { <StateT as HasData>::Data::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

impl<T: IStatefulWidget + Generic> __AStateT<Get, T> {
    pub fn widget(&self) -> Option<CRc<T>> {
        self.1
            .__data()
            .widget
            .get_cloned()
            .map(|w| (w.into_rc_dyn() as CRc<Downcast>).downcast().ok().unwrap())
    }
    pub fn element(&self) -> Option<CRc<Element>> {
        self.1.__data().element.get_cloned()
    }
}

impl<T: IStatefulWidget + Generic> __AStateT<Set, T> {
    pub fn widget(&self, widget: Option<CRc<T>>) {
        self.1
            .__data()
            .widget
            .set(widget.map(|w| Dyn::from_rc_dyn(w as CRc<_>)));
    }
    pub fn element(&self, element: Option<CRc<Element>>) {
        self.1.__data().element.set(element);
    }
}

impl __AStateT<Get> {
    pub fn widget(&self) -> Option<CRc<StatefulWidget>> {
        self.1.__data().widget.get_cloned().map(Dyn::into_rc_dyn)
    }
    pub fn element(&self) -> Option<CRc<Element>> {
        self.1.__data().element.get_cloned()
    }
}

pub struct __AState<Mode: AccessMode>(PhantomData<Mode>, State);

unsafe impl Access for State {
    type Accessor<Mode: AccessMode> = __AState<Mode>;
}

impl State {
    fn __data(&self) -> &__DState {
        let offset = const { <State as HasData>::Data::CLASS_TYPE.offset() };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

impl __AState<Get> {
    pub fn widget(&self) -> Option<CRc<StatefulWidget>> {
        self.1.__data().widget.get_cloned().map(Dyn::into_rc_dyn)
    }
    pub fn element(&self) -> Option<CRc<Element>> {
        self.1.__data().element.get_cloned()
    }
}

impl __AState<Set> {
    pub fn widget(&self, widget: Option<CRc<StatefulWidget>>) {
        self.1.__data().widget.set(widget.map(Dyn::from_rc_dyn));
    }
    pub fn element(&self, element: Option<CRc<Element>>) {
        self.1.__data().element.set(element);
    }
}
