use std::{marker::PhantomData, mem::offset_of, ops::Deref};

use oop_rs::{
    __private::{
        accessor::{AccessMode, AccessorUpcast},
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataOffsetEntry,
            DataTypeInfo, HasData, MixinData, MixinDataOffset,
        },
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, HasSuperVtable, HasSuperVtableImpl,
            HasVtable, HasVtableImpl, Vtable, VtableTypeInfo,
        },
    },
    class::{ClassConcrete, HasSuper},
    prelude::{Access, CClass, CData, CVtable, MClass, MVtable},
    rc::{CRc, CRcUninit, RcRef},
    read_vtable, write_vtable,
};

use manual_impl::{
    element::{Element, IElement},
    root_element_mixin::RootElementMixin,
};

use crate::root_widget::RootWidget;

const TYPE_NAME: &str = concat!(module_path!(), "::RootElement");
type __Self = RootElement;

#[repr(C)]
pub struct __SRootElement {
    __super: MClass<CClass<Element, __Self>, RootElementMixin, __Self>,
    data: __DRootElement,
}

pub struct __DRootElement {}

impl ClassOrMixinData for __DRootElement {
    const TYPE: DataTypeInfo<'static> = DataTypeInfo::Class(
        &ClassDataTypeInfo::new(TYPE_NAME)
            .with_super(
                <CData<Element> as ClassData>::CLASS_TYPE,
                offset_of!(__SRootElement, data),
            )
            .with_mixins(&[DataOffsetEntry::new(
                <CData<RootElementMixin> as MixinData>::MIXIN_TYPE,
                <MClass<CClass<Element, __Self>, RootElementMixin, __Self> as MixinDataOffset>::OFFSET,
            )]),
    );
}

impl ClassData for __DRootElement {}

impl HasData for __SRootElement {
    type Data = __DRootElement;
}

impl ClassHasData for __SRootElement {}

impl HasVtable for __SRootElement {
    type Vtable = __VRootElement;
}

impl ClassHasVtable for __SRootElement {}

impl HasVtableImpl for __SRootElement {
    const VTABLE: Self::Vtable = __SRootElement::__VTABLE;
}

impl HasSuperVtable for __SRootElement {
    type SuperVtable = MVtable<CClass<Element, __Self>, RootElementMixin, __Self>;
}

impl HasSuperVtableImpl for __SRootElement {
    const SUPER_VTABLE: Self::SuperVtable =
        <MClass<CClass<Element, __Self>, RootElementMixin, __Self> as HasVtableImpl>::VTABLE;
}

impl HasSuper for __DRootElement {
    type Super = CData<MClass<CClass<Element, __Self>, RootElementMixin, __Self>>;
}

unsafe impl RcRef for __SRootElement {}

impl Deref for __SRootElement {
    type Target = MClass<CClass<Element, __Self>, RootElementMixin, __Self>;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl HasSuper for __SRootElement {
    type Super = MClass<CClass<Element, __Self>, RootElementMixin, __Self>;
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct __VRootElement {
    pub __super: MVtable<CClass<Element, __Self>, RootElementMixin, __Self>,
    __self: PhantomData<__Self>,
}

impl Vtable for __VRootElement {
    const TYPE: VtableTypeInfo<'static> = VtableTypeInfo::Class(
        &ClassVtableTypeInfo::new(TYPE_NAME)
            .with_super(<<__SRootElement as HasSuperVtable>::SuperVtable as Vtable>::TYPE),
    );
}

impl ClassVtable for __VRootElement {}

impl HasSuper for __VRootElement {
    type Super = CVtable<Element, __Self>;
}

oop_rs::impl_vtable_copy!(<()> __VRootElement);

impl __SRootElement {
    const __VTABLE: __VRootElement = {
        let mut vtable = __VRootElement {
            __super:
                <MClass<CClass<Element, __Self>, RootElementMixin, __Self> as HasVtableImpl>::VTABLE,
            __self: PhantomData,
        };
        Self::__override(&mut vtable);
        vtable
    };

    const fn __override(vtable: &mut __VRootElement) {
        write_vtable!((vtable as Element).perform_rebuild = Self::perform_rebuild);
        write_vtable!((vtable as Element).mount = Self::mount);
    }

    pub fn new(mut __self: CRcUninit<__Self, Self>, widget: CRc<RootWidget>) -> CRc<__Self> {
        let data = __DRootElement {};
        let __self = unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, data);
            __self.assume_init_except()
        };
        let __self = MClass::<CClass<Element, __Self>, RootElementMixin, __Self>::__mixin(__self);
        let __self = CClass::<Element, __Self>::new(__self, widget);
        println!("RootElement");
        __self
    }

    pub fn perform_rebuild(__self: &__Self) {
        println!("RootElement::perform_rebuild");
        read_vtable!(<Super as Element>::perform_rebuild)(__self);
    }
    pub fn mount(__self: &__Self, parent: Option<&Element>) {
        println!("RootElement::mount");
        read_vtable!(<Super as Element>::mount)(__self, parent);
        __self.perform_rebuild();
    }
}

#[repr(transparent)]
pub struct RootElement(__SRootElement);

unsafe impl RcRef for RootElement {}

impl HasData for RootElement {
    type Data = __DRootElement;
}
impl ClassHasData for RootElement {}
impl HasVtable for RootElement {
    type Vtable = __VRootElement;
}
impl HasVtableImpl for RootElement {
    const VTABLE: Self::Vtable = __SRootElement::__VTABLE;
}
impl ClassHasVtable for RootElement {}
impl ClassConcrete for RootElement {}
impl ClassConcreteOrDyn for RootElement {
    const TYPE: &ClassConcreteOrDynTypeInfo =
        &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME).with_super::<RootElement>();
}

impl HasSuper for RootElement {
    type Super = RootElementMixin;
}

impl Deref for RootElement {
    type Target = __SRootElement;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RootElement {
    pub fn new(widget: CRc<RootWidget>) -> CRc<__Self> {
        __SRootElement::new(CRcUninit::new().cast_uninit(), widget)
    }
}

unsafe impl Access for RootElement {
    type Accessor<__M: AccessMode> = __ARootElement<__M>;
}

#[repr(transparent)]
pub struct __ARootElement<__M: AccessMode>(PhantomData<__M>, RootElement);

pub struct __Accessors<const N: usize>;

impl<__M: AccessMode> Deref for __ARootElement<__M> {
    type Target = <RootElementMixin as Access>::Accessor<__M>;
    fn deref(&self) -> &Self::Target {
        (&self.1 as &RootElementMixin).__access::<__M>()
    }
}

impl AccessorUpcast<RootElementMixin> for __Accessors<0> {
    type SuperAccessor<M: AccessMode> = <Element as Access>::Accessor<M>;

    fn upcast<M: AccessMode>(this: &RootElementMixin) -> &Self::SuperAccessor<M> {
        &(this as &Element).__access::<M>()
    }
}
