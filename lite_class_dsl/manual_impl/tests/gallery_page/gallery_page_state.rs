use std::{marker::PhantomData, mem::offset_of, ops::Deref};

use manual_impl::{
    build_context::BuildContext,
    state::{IStateT, State, StateT},
    widget::Widget,
};
use oop_rs::{
    __private::{
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo},
        generic::{Generic, TypeInfo},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, HasSuperVtable, HasSuperVtableImpl,
            HasVtable, HasVtableImpl, Vtable, VtableTypeInfo,
        },
    },
    class::{ClassConcrete, HasSuper},
    prelude::{CClass, CData, CVtable},
    rc::{CRc, CWeak, RcDefault, RcRef},
    read_vtable, write_vtable,
};

use crate::{gallery_page::GalleryPage, my_widget::MyWidget};

const TYPE_NAME: &str = concat!(module_path!(), "::GalleryPageState");
type __Self = GalleryPageState;

pub struct __SGalleryPageState {
    __super: CClass<StateT<GalleryPage>, __Self>,
    #[expect(dead_code)]
    data: __DGalleryPageState,
}

pub struct __DGalleryPageState {}

impl HasData for __SGalleryPageState {
    type Data = __DGalleryPageState;
}

impl ClassHasData for __SGalleryPageState {}

impl HasVtable for __SGalleryPageState {
    type Vtable = __VGalleryPageState;
}

impl ClassHasVtable for __SGalleryPageState {}

impl HasVtableImpl for __SGalleryPageState {
    const VTABLE: Self::Vtable = __SGalleryPageState::__VTABLE;
}

impl HasSuperVtable for __SGalleryPageState {
    type SuperVtable = CVtable<StateT<GalleryPage>, __Self>;
}

impl HasSuperVtableImpl for __SGalleryPageState {
    const SUPER_VTABLE: Self::SuperVtable =
        <CClass<StateT<GalleryPage>, __Self> as HasVtableImpl>::VTABLE;
}

unsafe impl RcRef for __SGalleryPageState {}

impl IStateT for __SGalleryPageState {
    type T = GalleryPage;
}

impl Deref for __SGalleryPageState {
    type Target = CClass<StateT<GalleryPage>, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl HasSuper for __SGalleryPageState {
    type Super = CClass<StateT<GalleryPage>, __Self>;
}

impl HasSuper for __DGalleryPageState {
    type Super = CData<StateT<GalleryPage>>;
}

impl ClassOrMixinData for __DGalleryPageState {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<StateT<GalleryPage>> as ClassData>::CLASS_TYPE,
            offset_of!(__SGalleryPageState, __super),
        ));
}

impl ClassData for __DGalleryPageState {}

pub struct __VGalleryPageState {
    __super: CVtable<StateT<GalleryPage>, __Self>,
    __self: PhantomData<GalleryPageState>,
}

oop_rs::impl_vtable_copy!(<()> __VGalleryPageState);

impl Vtable for __VGalleryPageState {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME)
            .with_super(<CVtable<StateT<GalleryPage>, __Self> as Vtable>::TYPE),
    );
}

impl ClassVtable for __VGalleryPageState {}

impl HasSuper for __VGalleryPageState {
    type Super = CVtable<StateT<GalleryPage>, __Self>;
}

impl __SGalleryPageState {
    const __VTABLE: __VGalleryPageState = {
        let mut vtable = __VGalleryPageState {
            __super: <CClass<StateT<GalleryPage>, __Self> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VGalleryPageState) {
        write_vtable!((vtable as StateT<GalleryPage>).build = Self::build);
        write_vtable!((vtable as StateT<GalleryPage>).init_state = Self::init_state);
    }

    pub fn init_state(__self: &__Self) {
        println!("GalleryPageState::init_state");
        read_vtable!(<Super as StateT<GalleryPage>>::init_state)(__self);
    }
    pub fn build(__self: &__Self, cx: &BuildContext) -> CRc<Widget> {
        println!("GalleryPageState::build");
        cx.widget()
            .downcast_ref::<GalleryPage>()
            .unwrap()
            .on_create();
        MyWidget::default()
    }
}

#[repr(transparent)]
pub struct GalleryPageState(__SGalleryPageState);

unsafe impl RcRef for GalleryPageState {}
unsafe impl Generic for GalleryPageState {
    const TYPE: &TypeInfo = &TypeInfo::Class(<GalleryPageState as ClassConcreteOrDyn>::TYPE);
}
impl HasData for GalleryPageState {
    type Data = __DGalleryPageState;
}
impl ClassHasData for GalleryPageState {}
impl ClassConcrete for GalleryPageState {}
impl ClassConcreteOrDyn for GalleryPageState {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME).with_super::<Self>();
}
impl HasVtable for GalleryPageState {
    type Vtable = __VGalleryPageState;
}
impl HasVtableImpl for GalleryPageState {
    const VTABLE: Self::Vtable = __SGalleryPageState::__VTABLE;
}
impl ClassHasVtable for GalleryPageState {}

impl HasSuper for GalleryPageState {
    type Super = State;
}

impl Deref for GalleryPageState {
    type Target = __SGalleryPageState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RcDefault for GalleryPageState {
    fn default() -> CRc<__Self> {
        let __init = |__super, __self_weak: &CWeak<Self>| {
            Self(__SGalleryPageState {
                __super,
                data: __DGalleryPageState {},
            })
        };
        CClass::<StateT<GalleryPage>, _>::__default(__init)
    }
}

impl IStateT for GalleryPageState {
    type T = GalleryPage;
}
