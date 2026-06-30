use crate::{
    __private::{dynamic::ClassConcreteOrDynTypeInfo, streq},
    alloc,
};

pub unsafe trait Generic: 'static {
    const TYPE: &TypeInfo;
}

pub enum TypeInfo {
    Class(&'static ClassConcreteOrDynTypeInfo),
    Simple(SimpleType),
    Slice(&'static TypeInfo),
    Ref(&'static TypeInfo, Mutability),
    Array(&'static TypeInfo, usize),
    Tuple(&'static [&'static TypeInfo]),
    Adt(AdtTypeInfo),
}

impl TypeInfo {
    pub const fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeInfo::Class(this), TypeInfo::Class(other)) => this.eq(other),
            (TypeInfo::Simple(this), TypeInfo::Simple(other)) => this.eq(other),
            (TypeInfo::Slice(this), TypeInfo::Slice(other)) => this.eq(other),
            (TypeInfo::Ref(this, this_mutability), TypeInfo::Ref(other, other_mutability)) => {
                this.eq(other) && this_mutability.eq(other_mutability)
            }
            (&TypeInfo::Array(this, this_len), &TypeInfo::Array(other, other_len)) => {
                this.eq(other) && this_len == other_len
            }
            (TypeInfo::Tuple(this), TypeInfo::Tuple(other)) => Self::eq_slice(this, other),
            (TypeInfo::Adt(this), TypeInfo::Adt(other)) => this.eq(other),
            _ => false,
        }
    }

    pub(super) const fn eq_slice(this: &[&Self], other: &[&Self]) -> bool {
        if this.len() != other.len() {
            return false;
        }
        let mut i = 0;
        while i < this.len() {
            if !this[i].eq(other[i]) {
                return false;
            }
            i += 1;
        }
        true
    }
}

pub enum SimpleType {
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    Isize,
    Usize,
    F32,
    F64,
    Str,
}

impl SimpleType {
    pub const fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SimpleType::Bool, SimpleType::Bool) => true,
            (SimpleType::Char, SimpleType::Char) => true,
            (SimpleType::I8, SimpleType::I8) => true,
            (SimpleType::U8, SimpleType::U8) => true,
            (SimpleType::I16, SimpleType::I16) => true,
            (SimpleType::U16, SimpleType::U16) => true,
            (SimpleType::I32, SimpleType::I32) => true,
            (SimpleType::U32, SimpleType::U32) => true,
            (SimpleType::I64, SimpleType::I64) => true,
            (SimpleType::U64, SimpleType::U64) => true,
            (SimpleType::I128, SimpleType::I128) => true,
            (SimpleType::U128, SimpleType::U128) => true,
            (SimpleType::Isize, SimpleType::Isize) => true,
            (SimpleType::Usize, SimpleType::Usize) => true,
            (SimpleType::F32, SimpleType::F32) => true,
            (SimpleType::F64, SimpleType::F64) => true,
            (SimpleType::Str, SimpleType::Str) => true,
            _ => false,
        }
    }
}

pub enum Mutability {
    Not,
    Mut,
}

impl Mutability {
    pub const fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Mutability::Not, Mutability::Not) => true,
            (Mutability::Mut, Mutability::Mut) => true,
            _ => false,
        }
    }
}
pub struct AdtTypeInfo {
    type_name: &'static str,
    generic_args: &'static [&'static TypeInfo],
}

impl AdtTypeInfo {
    pub const fn eq(&self, other: &Self) -> bool {
        streq(self.type_name, other.type_name)
            && TypeInfo::eq_slice(self.generic_args, other.generic_args)
    }
}

macro_rules! impl_generic_simple {
    ($($ty:ident: $kind:ident),* $(,)?) => {
        $(
            unsafe impl Generic for $ty {
                const TYPE: &TypeInfo = &TypeInfo::Simple(SimpleType::$kind);
            }
        )*
    };
}

impl_generic_simple! {
    bool: Bool,
    char: Char,
    i8: I8,
    u8: U8,
    i16: I16,
    u16: U16,
    i32: I32,
    u32: U32,
    i64: I64,
    u64: U64,
    i128: I128,
    u128: U128,
    isize: Isize,
    usize: Usize,
    f32: F32,
    f64: F64,
    str: Str,
}

unsafe impl<T: Generic> Generic for &'static T {
    const TYPE: &TypeInfo = &TypeInfo::Ref(<T as Generic>::TYPE, Mutability::Not);
}

unsafe impl<T: Generic> Generic for &'static mut T {
    const TYPE: &TypeInfo = &TypeInfo::Ref(<T as Generic>::TYPE, Mutability::Mut);
}

unsafe impl<T: Generic> Generic for [T] {
    const TYPE: &TypeInfo = &TypeInfo::Slice(<T as Generic>::TYPE);
}

unsafe impl<T: Generic, const N: usize> Generic for [T; N] {
    const TYPE: &TypeInfo = &TypeInfo::Array(<T as Generic>::TYPE, N);
}

macro_rules! impl_generic_tuple {
    ($($t:ident),* $(,)?) => {
        unsafe impl<$($t: Generic),*> Generic for ($($t,)*) {
            const TYPE: &TypeInfo = &TypeInfo::Tuple(&[$(<$t as Generic>::TYPE),*]);
        }
    };
}

impl_generic_tuple!();
impl_generic_tuple!(T1);
impl_generic_tuple!(T1, T2);
impl_generic_tuple!(T1, T2, T3);
impl_generic_tuple!(T1, T2, T3, T4);
impl_generic_tuple!(T1, T2, T3, T4, T5);
impl_generic_tuple!(T1, T2, T3, T4, T5, T6);
impl_generic_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_generic_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_generic_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_generic_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_generic_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_generic_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

macro_rules! impl_generic_adt {
    ($krate:ident$(::$path:ident)*<$($t:ident),* $(,)?> ) => {
        unsafe impl<$($t: Generic),*> Generic for $krate$(::$path)*<$($t,)*> {
            const TYPE: &TypeInfo = &TypeInfo::Adt(AdtTypeInfo {
                type_name: concat!(stringify!($krate), $("::", stringify!($path)),*),
                generic_args: &[$(<$t as Generic>::TYPE),*],
            });
        }
    }
}

impl_generic_adt!(core::option::Option<T>);
impl_generic_adt!(core::result::Result<T, E>);
impl_generic_adt!(alloc::vec::Vec<T>);
impl_generic_adt!(alloc::collections::VecDeque<T>);
impl_generic_adt!(alloc::collections::LinkedList<T>);
impl_generic_adt!(alloc::collections::BTreeMap<K, V>);
impl_generic_adt!(alloc::collections::BTreeSet<T>);
impl_generic_adt!(alloc::collections::BinaryHeap<T>);

#[cfg(feature = "std")]
impl_generic_adt!(std::collections::HashMap<K, V>);
#[cfg(feature = "std")]
impl_generic_adt!(std::collections::HashSet<T>);
