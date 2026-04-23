use crate::__private::{
    data::ClassHasData,
    dynamic::ClassConcreteOrDyn,
    vtable::{ClassHasVtableImpl, HasVtableImpl},
};

pub trait Class: ClassHasData + ClassHasVtableImpl + Sized + 'static {}

pub trait Interface: HasVtableImpl + Sized + 'static {}

/// Whether a class is a concrete class.
pub trait ClassConcrete: ClassConcreteOrDyn + Class {}

pub trait HasSuper {
    type Super: ?Sized;
}

impl<T: ClassHasData + ClassHasVtableImpl + 'static> Class for T {}
impl<T: HasVtableImpl + 'static> Interface for T {}
