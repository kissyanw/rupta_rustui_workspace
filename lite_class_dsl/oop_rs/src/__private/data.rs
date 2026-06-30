//! Placeholder module initialized for vtable private internals.
use crate::{
    __private::{generic::TypeInfo, streq},
    class::ClassConcrete,
};

pub trait HasData: 'static {
    type Data: ClassOrMixinData;
}

pub trait ClassHasData: HasData<Data: ClassData> {}

pub trait MixinHasData: HasData<Data: MixinData> {}

pub trait MixinDataOffset {
    const OFFSET: usize;
}

pub trait ClassOrMixinData: 'static {
    const TYPE: DataTypeInfo<'static>;
}

pub trait ClassData: ClassOrMixinData {
    const CLASS_TYPE: &ClassDataTypeInfo<'_> = Self::TYPE.expect_class();
}

pub trait MixinData: ClassOrMixinData {
    const MIXIN_TYPE: &MixinDataTypeInfo<'_> = Self::TYPE.expect_mixin();
}

pub trait EmptyData: ClassOrMixinData + Sized {
    const CHECK_SIZE: () = {
        assert!(core::mem::size_of::<Self>() == 0, "Data type must be empty");
    };
}

pub trait AsData<C: HasData> {
    fn as_data(&self) -> &C::Data;
}

pub trait AsMutData<C: HasData> {
    fn as_mut_data(&mut self) -> &mut C::Data;
}

pub trait MixinAsData<Data: HasData> {
    fn as_data(&self) -> &Data;
}

pub trait MixinAsMutData<Data: ClassData> {
    fn as_mut_data(&mut self) -> &mut Data;
}

#[diagnostic::do_not_recommend]
impl<__Self: ClassConcrete, C: HasData> AsData<C> for __Self {
    fn as_data(&self) -> &C::Data {
        let offset = const {
            __Self::Data::CLASS_TYPE
                .offset_of(&C::Data::TYPE)
                .expect("not a subclass")
        };
        unsafe { &*core::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

#[diagnostic::do_not_recommend]
impl<__Self: ClassConcrete, C: HasData> AsMutData<C> for __Self {
    fn as_mut_data(&mut self) -> &mut C::Data {
        let offset = const {
            __Self::Data::CLASS_TYPE
                .offset_of(&C::Data::TYPE)
                .expect("not a subclass")
        };
        unsafe { &mut *core::ptr::from_mut(self).byte_add(offset).cast() }
    }
}

#[derive(Clone, Copy)]
pub enum DataTypeInfo<'a> {
    Class(&'a ClassDataTypeInfo<'a>),
    Mixin(&'a MixinDataTypeInfo<'a>),
}

impl<'a> DataTypeInfo<'a> {
    const fn expect_class(self) -> &'a ClassDataTypeInfo<'a> {
        match self {
            DataTypeInfo::Class(class) => class,
            DataTypeInfo::Mixin(_) => panic!("not a class"),
        }
    }

    const fn expect_mixin(self) -> &'a MixinDataTypeInfo<'a> {
        match self {
            DataTypeInfo::Class(_) => panic!("not a mixin"),
            DataTypeInfo::Mixin(mixin) => mixin,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ClassDataTypeInfo<'a> {
    __super: Option<&'a ClassDataTypeInfo<'a>>,
    // FIXME: This should use `TypeId` for comparison, but `TypeId` cannot be
    // compared at compile time yet, so we use `type_name` for working around.
    // type_id: TypeId,
    type_name: &'static str,
    offset: usize,
    mixins: &'a [DataOffsetEntry<'a>],
    generics: &'a [&'a TypeInfo],
}

#[derive(Clone, Copy)]
pub struct DataOffsetEntry<'a> {
    mixin: &'a MixinDataTypeInfo<'a>,
    offset: usize,
}

impl<'a> DataOffsetEntry<'a> {
    pub const fn new(mixin: &'a MixinDataTypeInfo<'a>, offset: usize) -> Self {
        Self { mixin, offset }
    }
}

#[derive(Clone, Copy)]
pub struct MixinDataTypeInfo<'a> {
    // FIXME: This should use `TypeId` for comparison, but `TypeId` cannot be
    // compared at compile time yet, so we use `type_name` for working around.
    // type_id: TypeId,
    type_name: &'static str,
    generics: &'a [&'a TypeInfo],
}

impl<'a> ClassDataTypeInfo<'a> {
    pub const fn new(type_name: &'static str) -> Self {
        Self {
            __super: None,
            type_name,
            mixins: &[],
            generics: &[],
            offset: 0,
        }
    }

    pub const fn with_mixins(self, mixins: &'a [DataOffsetEntry<'a>]) -> Self {
        Self { mixins, ..self }
    }

    pub const fn with_super(self, __super: &'a ClassDataTypeInfo<'a>, offset: usize) -> Self {
        Self {
            __super: Some(__super),
            offset,
            ..self
        }
    }

    pub const fn with_generics(self, generics: &'a [&'a TypeInfo]) -> Self {
        Self { generics, ..self }
    }

    pub const fn eq(&self, other: &Self) -> bool {
        streq(self.type_name, other.type_name) && TypeInfo::eq_slice(self.generics, other.generics)
    }

    pub const fn offset(&self) -> usize {
        self.offset
    }

    pub const fn offset_of_mixin(&self, other: &MixinDataTypeInfo<'_>) -> Option<usize> {
        self.offset_of(&DataTypeInfo::Mixin(other))
    }

    pub const fn offset_of_class(&self, other: &ClassDataTypeInfo<'_>) -> Option<usize> {
        self.offset_of(&DataTypeInfo::Class(other))
    }

    pub const fn offset_of(&self, other: &DataTypeInfo<'_>) -> Option<usize> {
        match other {
            DataTypeInfo::Class(other) if self.eq(other) => return Some(self.offset),
            DataTypeInfo::Mixin(other) => {
                let mut i = self.mixins.len();
                while i > 0 {
                    i -= 1;
                    if self.mixins[i].mixin.eq(other) {
                        return Some(self.mixins[i].offset);
                    }
                }
            }
            _ => {}
        }

        match self.__super {
            Some(super_class) => super_class.offset_of(other),
            None => None,
        }
    }
}

impl<'a> MixinDataTypeInfo<'a> {
    pub const fn new(type_name: &'static str) -> Self {
        Self {
            type_name,
            generics: &[],
        }
    }

    pub const fn with_generics(self, generics: &'a [&'a TypeInfo]) -> Self {
        Self { generics, ..self }
    }

    pub const fn eq(&self, other: &Self) -> bool {
        streq(self.type_name, other.type_name) && TypeInfo::eq_slice(self.generics, other.generics)
    }
}

#[macro_export]
macro_rules! assert_subclass {
    ($self:ident, $super:ident) => {
        fn __assert_subclass(&self)
        where
            Self: $crate::class::ClassConcrete + ::core::marker::Sized,
        {
            const {
                let _ = <<Self as $crate::__private::data::HasData>::Data as $crate::__private::data::ClassData>::CLASS_TYPE
                        .offset_of_class(
                            &<<$super as $crate::__private::data::HasData>::Data as $crate::__private::data::ClassData>::CLASS_TYPE,
                        ).expect(concat!("not a subclass of `", stringify!($super), "`"));
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_offset_of() {
        let element = ClassDataTypeInfo::new("Element");
        let root_element_mixin = MixinDataTypeInfo::new("RootElementMixin");
        let root_element_mixins = [DataOffsetEntry::new(&root_element_mixin, 0)];
        let root_element = ClassDataTypeInfo::new("RootElement")
            .with_super(&element, 0)
            .with_mixins(&root_element_mixins);
        assert_eq!(root_element.offset_of_class(&element), Some(0));
        assert_eq!(root_element.offset_of_mixin(&root_element_mixin), Some(0));
    }
}
