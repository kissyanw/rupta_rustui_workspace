use std::{marker::PhantomData, mem::offset_of, ops::Deref};

use oop_rs::{
    __private::{
        Dummy,
        accessor::{AccessMode, AccessorUpcast, MixinAccessor},
        data::{
            ClassData, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData, MixinData,
            MixinDataOffset, MixinDataTypeInfo, MixinHasData,
        },
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, MixinDyn, MixinHasDyn},
        vtable::{
            DynHasVtable, HasSuperVtable, HasSuperVtableImpl, HasVtable, HasVtableImpl,
            MixinInstanceVtable, MixinInstanceVtableTypeInfo, MixinVtable, MixinVtableTypeInfo,
            Vtable, VtableTypeInfo,
        },
    },
    class::{Class, ClassConcrete, HasSuper},
    prelude::Access,
    rc::{CRcUninit, RcRef},
    read_vtable, write_vtable,
};

use crate::element::{Element, IElement};

const TYPE_NAME: &str = concat!(module_path!(), "::RootElementMixin");

#[repr(C)]
pub struct __SRootElementMixin<__Super: HasData, __Self: ClassConcrete> {
    __super: __Super,
    __self: PhantomData<__Self>,
    data: __DRootElementMixin,
}

impl<__Super: HasData, __Self: ClassConcrete> MixinDataOffset
    for __SRootElementMixin<__Super, __Self>
{
    const OFFSET: usize = offset_of!(__SRootElementMixin<__Super, __Self>, data);
}
impl<__Super: ClassHasData, __Self: ClassConcrete + IElement + RcRef> HasData
    for __SRootElementMixin<__Super, __Self>
{
    type Data = __DRootElementMixin;
}

impl<__Super: Class, __Self: ClassConcrete + IElement + RcRef> MixinHasData
    for __SRootElementMixin<__Super, __Self>
{
}
impl<__Super: HasVtable + HasData, __Self: ClassConcrete> HasVtable
    for __SRootElementMixin<__Super, __Self>
{
    type Vtable = __VMRootElementMixin<__Super::Vtable, __Self>;
}

impl<__Super: HasVtable + HasData, __Self: ClassConcrete> HasSuperVtable
    for __SRootElementMixin<__Super, __Self>
{
    type SuperVtable = <__Super as HasVtable>::Vtable;
}

impl<__Super: HasVtableImpl + HasData, __Self: ClassConcrete + IElement + RcRef> HasVtableImpl
    for __SRootElementMixin<__Super, __Self>
{
    const VTABLE: Self::Vtable = __SRootElementMixin::<__Super, __Self>::__VTABLE;
}

impl<__Super: HasVtableImpl + HasData, __Self: ClassConcrete + IElement + RcRef> HasSuperVtableImpl
    for __SRootElementMixin<__Super, __Self>
{
    const SUPER_VTABLE: Self::SuperVtable = <__Super as HasVtableImpl>::VTABLE;
}

impl<__Super: Class, __Self: ClassConcrete> Deref for __SRootElementMixin<__Super, __Self> {
    type Target = __Super;
    fn deref(&self) -> &Self::Target {
        &self.__super
    }
}

impl<__Super: Class, __Self: ClassConcrete> HasSuper for __SRootElementMixin<__Super, __Self> {
    type Super = __Super;
}

pub struct __DRootElementMixin {}

impl ClassOrMixinData for __DRootElementMixin {
    const TYPE: DataTypeInfo<'_> = DataTypeInfo::Mixin(&MixinDataTypeInfo::new(TYPE_NAME));
}
impl MixinData for __DRootElementMixin {}

impl ClassConcreteOrDyn for RootElementMixin {
    const TYPE: &ClassConcreteOrDynTypeInfo = &ClassConcreteOrDynTypeInfo::new_mixin(TYPE_NAME);
}

unsafe impl<__Super: Class, __Self: ClassConcrete + RcRef> RcRef
    for __SRootElementMixin<__Super, __Self>
{
}

pub trait IRootElementMixin: IElement {
    fn __offset(&self) -> usize;
}

pub type RootElementMixin = dyn IRootElementMixin;

impl<__Self: ClassConcrete + IElement> IRootElementMixin for __Self {
    fn __offset(&self) -> usize {
        const {
            <__Self as HasData>::Data::CLASS_TYPE
                .offset_of_mixin(__DRootElementMixin::MIXIN_TYPE)
                .expect("not a subclass of RootElementMixin")
        }
    }
}

impl MixinHasData for RootElementMixin {}
impl HasData for RootElementMixin {
    type Data = __DRootElementMixin;
}

impl DynHasVtable for RootElementMixin {
    type Vtable<__Self: ClassConcrete> = __VRootElementMixin<__Self>;
}

#[repr(C)]
pub struct __VMRootElementMixin<__SuperVtable: Vtable, __Self: ClassConcrete> {
    __super: __SuperVtable,
    __self: __VRootElementMixin<__Self>,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct __VRootElementMixin<__Self: ClassConcrete> {
    __self: PhantomData<__Self>,
}

oop_rs::impl_vtable_copy!(<(__Self: ClassConcrete)> __VRootElementMixin< __Self>);
oop_rs::impl_vtable_copy!(<(__SuperVtable: Vtable, __Self: ClassConcrete)> __VMRootElementMixin<__SuperVtable, __Self>);

impl<__SuperVtable: Vtable, __Self: ClassConcrete> Vtable
    for __VMRootElementMixin<__SuperVtable, __Self>
{
    const TYPE: VtableTypeInfo<'static> =
        VtableTypeInfo::MixinInstance(&MixinInstanceVtableTypeInfo::new(
            <__SuperVtable as Vtable>::TYPE,
            <__VRootElementMixin<__Self> as MixinVtable>::MIXIN_TYPE,
            offset_of!(Self, __self),
        ));
}

impl<__Self: ClassConcrete> Vtable for __VRootElementMixin<__Self> {
    const TYPE: VtableTypeInfo<'static> =
        VtableTypeInfo::Mixin(&MixinVtableTypeInfo::new(TYPE_NAME));
}

impl<__Super: Vtable, __Self: ClassConcrete> MixinInstanceVtable
    for __VMRootElementMixin<__Super, __Self>
{
}
impl<__Self: ClassConcrete> MixinVtable for __VRootElementMixin<__Self> {}

impl<__Super: HasVtableImpl + HasData, __Self: ClassConcrete + IElement>
    __SRootElementMixin<__Super, __Self>
{
    const __VTABLE: __VMRootElementMixin<__Super::Vtable, __Self> = {
        let mut vtable = __VMRootElementMixin {
            __super: <__Super as HasVtableImpl>::VTABLE,
            __self: __VRootElementMixin {
                __self: PhantomData,
            },
        };
        Self::__override(&mut vtable);
        vtable
    };

    const fn __override(vtable: &mut __VMRootElementMixin<__Super::Vtable, __Self>) {
        write_vtable!((vtable as Element).mount = Self::mount);
    }

    pub fn __mixin(mut __self: CRcUninit<__Self, Self>) -> CRcUninit<__Self, __Super> {
        let __data = __DRootElementMixin {};
        unsafe {
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self, PhantomData);
            core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data, __data);
            __self.assume_init_except()
        }
    }
    #[allow(unconditional_recursion)]
    fn mount(__self: &__Self, parent: Option<&Element>) {
        let __self_dyn = __self as &RootElementMixin;
        println!("RootElementMixin::mount");
        read_vtable!(<Super as Element>::mount)(__self, parent);
    }
}

impl RootElementMixin {
    fn __data(&self) -> &__DRootElementMixin {
        unsafe {
            &*core::ptr::from_ref(self)
                .byte_add(Self::__offset(self))
                .cast()
        }
    }
}

impl MixinDyn for RootElementMixin {
    type Class<__Super: HasData, __Self: ClassConcrete> = __SRootElementMixin<__Super, __Self>;
}

impl<__Super: HasData, __Self: ClassConcrete> MixinHasDyn for __SRootElementMixin<__Super, __Self> {
    type Dyn = RootElementMixin;
}

#[repr(transparent)]
pub struct __ARootElementMixin<
    __M: AccessMode,
    __Upcast: AccessorUpcast<__T> = Dummy,
    __T: ?Sized = RootElementMixin,
>(PhantomData<__M>, PhantomData<__Upcast>, __T);

unsafe impl Access for RootElementMixin {
    type Accessor<__M: AccessMode> = __ARootElementMixin<__M>;
}

unsafe impl MixinAccessor for RootElementMixin {
    type Accessor<__M: AccessMode, __Upcast: AccessorUpcast<__T>, __T: ?Sized> =
        __ARootElementMixin<__M, __Upcast, __T>;
}

impl<Mode: AccessMode, __Upcast: AccessorUpcast<__T>, __T: ?Sized> Deref
    for __ARootElementMixin<Mode, __Upcast, __T>
{
    type Target = <__Upcast as AccessorUpcast<__T>>::SuperAccessor<Mode>;

    fn deref(&self) -> &Self::Target {
        __Upcast::upcast(&self.2)
    }
}
