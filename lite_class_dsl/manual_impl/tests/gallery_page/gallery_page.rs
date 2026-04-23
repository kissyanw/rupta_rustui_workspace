use std::{marker::PhantomData, mem::offset_of, ops::Deref};

use manual_impl::{state::State, stateful_widget::StatefulWidget};
use oop_rs::{
    __private::{
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        downcast::{Downcast, ErasedPtr},
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo},
        generic::{Generic, TypeInfo},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, HasVtable, HasVtableImpl, Type,
            Vtable, VtableTypeInfo,
        },
    },
    class::{ClassConcrete, HasSuper},
    prelude::{CClass, CData, CVtable},
    rc::{CRc, CRcUninit, RcDefault, RcRef},
    read_vtable, write_vtable,
};

use crate::gallery_page_state::GalleryPageState;

const TYPE_NAME: &str = concat!(module_path!(), "::GalleryPage");
type __Self = GalleryPage;

#[repr(C)]
pub struct __SGalleryPage {
    __super: CClass<StatefulWidget, GalleryPage>,
    data: __DGalleryPage,
}

pub struct __DGalleryPage {}

impl ClassOrMixinData for __DGalleryPage {
    const TYPE: DataTypeInfo<'static> =
        DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME).with_super(
            <CData<StatefulWidget> as ClassData>::CLASS_TYPE,
            offset_of!(__SGalleryPage, __super),
        ));
}

impl ClassData for __DGalleryPage {}

impl HasData for __SGalleryPage {
    type Data = __DGalleryPage;
}

impl ClassHasData for __SGalleryPage {}

impl HasVtable for __SGalleryPage {
    type Vtable = __VGalleryPage;
}

impl ClassHasVtable for __SGalleryPage {}

impl HasVtableImpl for __SGalleryPage {
    const VTABLE: Self::Vtable = __SGalleryPage::__VTABLE;
}

impl HasSuper for __DGalleryPage {
    type Super = CData<StatefulWidget>;
}

unsafe impl RcRef for __SGalleryPage {}

impl Deref for __SGalleryPage {
    type Target = CClass<StatefulWidget, GalleryPage>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl HasSuper for __SGalleryPage {
    type Super = CClass<StatefulWidget, GalleryPage>;
}

pub struct __VGalleryPage {
    __super: CVtable<StatefulWidget, GalleryPage>,
    __self: PhantomData<GalleryPage>,
}

oop_rs::impl_vtable_copy!(<()> __VGalleryPage);

impl Vtable for __VGalleryPage {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME)
            .with_super(<CVtable<StatefulWidget, GalleryPage> as Vtable>::TYPE),
    );
}

impl ClassVtable for __VGalleryPage {}

impl HasSuper for __VGalleryPage {
    type Super = CVtable<StatefulWidget, GalleryPage>;
}

impl __SGalleryPage {
    const __VTABLE: __VGalleryPage = {
        let mut vtable = __VGalleryPage {
            __super: <CClass<StatefulWidget, GalleryPage> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
        };
        Self::__override(&mut vtable);
        vtable
    };

    #[allow(unused_variables)]
    const fn __override(vtable: &mut __VGalleryPage) {
        write_vtable!((vtable as Downcast).ty = __Self::ty);
        write_vtable!((vtable as Downcast).__downcast = __Self::__downcast);
        write_vtable!((vtable as StatefulWidget).create_state = Self::create_state);
    }

    fn __default(mut __self: CRcUninit<__Self, Self>) -> CRc<__Self> {
        let __data = __DGalleryPage {};
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        };
        CClass::<StatefulWidget, __Self>::__default(__self)
    }

    pub fn create_state(__self: &__Self) -> CRc<State> {
        println!("GalleryPage::create_state");
        GalleryPageState::default()
    }
}

#[repr(transparent)]
pub struct GalleryPage(__SGalleryPage);

unsafe impl RcRef for GalleryPage {}

impl RcDefault for GalleryPage {
    fn default() -> CRc<Self> {
        __SGalleryPage::__default(CRcUninit::new().cast_uninit())
    }
}

unsafe impl Generic for GalleryPage {
    const TYPE: &TypeInfo = &TypeInfo::Class(<GalleryPage as ClassConcreteOrDyn>::TYPE);
}

impl ClassConcrete for GalleryPage {}
impl ClassConcreteOrDyn for GalleryPage {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME).with_super::<Self>();
}
impl HasData for GalleryPage {
    type Data = __DGalleryPage;
}
impl ClassHasData for GalleryPage {}
impl HasVtable for GalleryPage {
    type Vtable = __VGalleryPage;
}
impl HasVtableImpl for GalleryPage {
    const VTABLE: Self::Vtable = __SGalleryPage::__VTABLE;
}
impl ClassHasVtable for GalleryPage {}

impl HasSuper for GalleryPage {
    type Super = StatefulWidget;
}

impl Deref for GalleryPage {
    type Target = __SGalleryPage;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GalleryPage {
    fn ty(&self) -> Type {
        Type::of::<GalleryPage>()
    }
    fn __downcast(&self, ty: Type) -> Option<ErasedPtr> {
        if ty == Type::of::<GalleryPage>() {
            return Some(ErasedPtr::new_thin(self.into()));
        }
        read_vtable!(<Downcast>::__downcast)(self, ty)
    }
}

impl GalleryPage {
    pub fn on_create(&self) {
        println!("GalleryPage::on_create");
    }
}
