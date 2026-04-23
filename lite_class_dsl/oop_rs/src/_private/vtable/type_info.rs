use core::any::TypeId;

use crate::__private::{generic::TypeInfo, streq};

#[derive(Clone, Copy)]
pub struct VtableOffsetEntry<'a, V> {
    vtable: &'a V,
    offset: usize,
}

impl<'a, V> VtableOffsetEntry<'a, V> {
    pub const fn new(vtable: &'a V, offset: usize) -> Self {
        Self { vtable, offset }
    }
}

#[derive(Clone, Copy)]
pub enum VtableTypeInfo<'a> {
    Class(&'a ClassVtableTypeInfo<'a>),
    Interface(&'a InterfaceVtableTypeInfo<'a>),
    Mixin(&'a MixinVtableTypeInfo<'a>),
    MixinInstance(&'a MixinInstanceVtableTypeInfo<'a>),
}

impl<'a> VtableTypeInfo<'a> {
    pub const fn expect_class(self) -> &'a ClassVtableTypeInfo<'a> {
        match self {
            Self::Class(class) => class,
            Self::Interface(_) | Self::Mixin(_) | Self::MixinInstance(_) => panic!("not a class"),
        }
    }

    pub const fn as_interface(self) -> &'a InterfaceVtableTypeInfo<'a> {
        match self {
            Self::Interface(interface) => interface,
            Self::Class(class) => class.as_interface(),
            Self::Mixin(mixin) => mixin.as_interface(),
            Self::MixinInstance(mixin_instance) => mixin_instance.mixin.as_interface(),
        }
    }

    pub const fn expect_mixin(self) -> &'a MixinVtableTypeInfo<'a> {
        match self {
            Self::Mixin(mixin) => mixin,
            Self::MixinInstance(mixin_instance) => mixin_instance.as_mixin(),
            Self::Class(_) | Self::Interface(_) => panic!("not a mixin"),
        }
    }

    pub const fn expect_mixin_instance(self) -> &'a MixinInstanceVtableTypeInfo<'a> {
        match self {
            Self::MixinInstance(mixin_instance) => mixin_instance,
            Self::Class(_) | Self::Interface(_) | Self::Mixin(_) => panic!("not a mixin instance"),
        }
    }

    pub(crate) const fn next_offset_of(self, other: Self, last_offset: usize) -> Option<usize> {
        self.offset_of_rec(other, Some(last_offset), 0)
    }

    pub const fn offset_of(self, other: Self) -> Option<usize> {
        self.offset_of_rec(other, None, 0)
    }

    const fn __super(&self) -> Option<VtableTypeInfo<'a>> {
        match self {
            Self::Class(class) => class.__super,
            Self::MixinInstance(mixin_instance) => Some(mixin_instance.__super),
            Self::Interface(_) | Self::Mixin(_) => None,
        }
    }

    pub(crate) const fn offset_of_rec(
        self,
        other: Self,
        last_offset: Option<usize>,
        offset_acc: usize,
    ) -> Option<usize> {
        assert!(self.super_offset() <= self.self_offset());

        // Step 1: Search in __super (inheritance chain)
        if let Some(__super) = self.__super()
            && let Some(offset) =
                __super.offset_of_rec(other, last_offset, offset_acc + self.super_offset())
            && filter(last_offset, offset)
        {
            return Some(offset);
        }

        let offset_acc = offset_acc + self.self_offset();

        // Step 2: Check if self matches (for class searches)
        if filter(last_offset, offset_acc) && self.as_interface().eq(other.as_interface()) {
            return Some(offset_acc);
        }

        // Step 3: Search in interfaces (last priority)
        let mut i = 0;
        while i < self.as_interface().interfaces.len() {
            let entry = &self.as_interface().interfaces[i];
            if let Some(offset) = Self::Interface(entry.vtable).offset_of_rec(
                other,
                last_offset,
                offset_acc + entry.offset,
            ) && filter(last_offset, offset)
            {
                return Some(offset);
            }
            i += 1;
        }

        None
    }

    const fn super_offset(&self) -> usize {
        match self {
            Self::Class(_) | Self::Interface(_) | Self::Mixin(_) | Self::MixinInstance(_) => 0,
        }
    }

    const fn self_offset(&self) -> usize {
        match self {
            Self::Class(_) | Self::Interface(_) | Self::Mixin(_) => 0,
            Self::MixinInstance(mixin_instance) => mixin_instance.mixin_offset,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MixinVtableTypeInfo<'a>(InterfaceVtableTypeInfo<'a>);

#[derive(Clone, Copy)]
pub struct MixinInstanceVtableTypeInfo<'a> {
    __super: VtableTypeInfo<'a>,
    mixin: &'a MixinVtableTypeInfo<'a>,
    mixin_offset: usize,
}

#[derive(Clone, Copy)]
pub struct InterfaceVtableTypeInfo<'a> {
    // FIXME: This should use `TypeId` for comparison, but `TypeId` cannot be
    // compared at compile time yet, so we use `type_name` for working around.
    // type_id: TypeId,
    type_name: &'static str,
    generics: &'a [&'a TypeInfo],
    interfaces: &'a [VtableOffsetEntry<'a, InterfaceVtableTypeInfo<'a>>],
}

#[derive(Clone, Copy)]
pub struct ClassVtableTypeInfo<'a> {
    __super: Option<VtableTypeInfo<'a>>,
    interface: InterfaceVtableTypeInfo<'a>,
}

impl<'a> ClassVtableTypeInfo<'a> {
    pub const fn new(type_name: &'static str) -> Self {
        Self {
            __super: None,
            interface: InterfaceVtableTypeInfo::new(type_name),
        }
    }

    pub const fn with_super(self, __super: VtableTypeInfo<'a>) -> Self {
        Self {
            __super: Some(__super),
            ..self
        }
    }

    pub const fn with_generics(self, generics: &'a [&'a TypeInfo]) -> Self {
        Self {
            interface: self.interface.with_generics(generics),
            ..self
        }
    }

    pub const fn with_interfaces(
        self,
        interfaces: &'a [VtableOffsetEntry<'a, InterfaceVtableTypeInfo>],
    ) -> Self {
        Self {
            interface: self.interface.with_interfaces(interfaces),
            ..self
        }
    }

    pub const fn as_interface(&self) -> &InterfaceVtableTypeInfo {
        &self.interface
    }

    pub const fn into_vtable(&'a self) -> VtableTypeInfo<'a> {
        VtableTypeInfo::Class(self)
    }
}

impl<'a> InterfaceVtableTypeInfo<'a> {
    pub const fn new(type_name: &'static str) -> Self {
        Self {
            type_name,
            generics: &[],
            interfaces: &[],
        }
    }

    pub const fn with_generics(self, generics: &'a [&'a TypeInfo]) -> Self {
        Self { generics, ..self }
    }

    pub const fn with_interfaces(
        self,
        interfaces: &'a [VtableOffsetEntry<'a, InterfaceVtableTypeInfo>],
    ) -> Self {
        Self { interfaces, ..self }
    }

    pub(super) const fn eq(&self, other: &Self) -> bool {
        streq(self.type_name, other.type_name) && TypeInfo::eq_slice(self.generics, other.generics)
    }

    pub const fn into_vtable(&'a self) -> VtableTypeInfo<'a> {
        VtableTypeInfo::Interface(self)
    }
}

impl<'a> MixinVtableTypeInfo<'a> {
    pub const fn new(type_name: &'static str) -> Self {
        Self(InterfaceVtableTypeInfo::new(type_name))
    }

    pub const fn with_generics(self, generics: &'a [&'a TypeInfo]) -> Self {
        Self(self.0.with_generics(generics))
    }

    pub const fn with_interfaces(
        self,
        interfaces: &'a [VtableOffsetEntry<'a, InterfaceVtableTypeInfo<'a>>],
    ) -> Self {
        Self(self.0.with_interfaces(interfaces))
    }

    pub const fn as_interface(&self) -> &InterfaceVtableTypeInfo<'a> {
        &self.0
    }

    pub const fn into_vtable(&'a self) -> VtableTypeInfo<'a> {
        VtableTypeInfo::Mixin(self)
    }
}

impl<'a> MixinInstanceVtableTypeInfo<'a> {
    pub const fn new(
        __super: VtableTypeInfo<'a>,
        mixin: &'a MixinVtableTypeInfo<'a>,
        mixin_offset: usize,
    ) -> Self {
        Self {
            __super,
            mixin,
            mixin_offset,
        }
    }

    pub const fn as_mixin(&self) -> &MixinVtableTypeInfo<'a> {
        &self.mixin
    }

    pub const fn as_interface(&self) -> &InterfaceVtableTypeInfo<'a> {
        &self.mixin.0
    }

    pub const fn into_vtable(&'a self) -> VtableTypeInfo<'a> {
        VtableTypeInfo::MixinInstance(self)
    }
}

const fn filter(last_offset: Option<usize>, offset: usize) -> bool {
    match last_offset {
        None => true,
        Some(last_offset) if offset > last_offset => true,
        Some(_) => false,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Type {
    type_id: TypeId,
    #[cfg(debug_assertions)]
    type_name: &'static str,
}

impl Type {
    pub fn of<T: 'static + ?Sized>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            #[cfg(debug_assertions)]
            type_name: core::any::type_name::<T>(),
        }
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    #[cfg(debug_assertions)]
    pub fn type_name(&self) -> &'static str {
        self.type_name
    }
}
