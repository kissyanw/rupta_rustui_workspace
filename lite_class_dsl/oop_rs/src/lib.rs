#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
pub mod alloc;

extern crate self as oop_rs;

pub mod __private;
pub mod cell;
pub mod class;
pub mod method;
pub mod missing;
pub mod object;
pub mod rc;

pub use oop_rs_macros::class;

pub mod prelude {
    use crate::__private::data::HasData;
    use crate::__private::dynamic::{ClassDyn, MixinDyn};
    use crate::__private::vtable::{DynHasVtable, HasVtable};
    use crate::class::HasSuper;

    pub use crate::method;

    pub use crate::cell::CellGetCloned;
    pub use oop_rs_macros::class;

    pub type CClass<T, __Self> = <T as ClassDyn>::Class<__Self>;
    pub type MClass<__Super, M, __Self> = <M as MixinDyn>::Class<__Super, __Self>;
    pub type CData<T> = <T as HasData>::Data;
    pub type CVtable<T, __Self> = <T as DynHasVtable>::Vtable<__Self>;
    pub type MVtable<__Super, T, __Self> = <MClass<__Super, T, __Self> as HasVtable>::Vtable;

    pub type Super<T> = <T as HasSuper>::Super;

    pub use crate::__private::accessor::Access;
    pub use crate::rc::{CRc, CWeak, IsRcLike, RcLike, RcRefImpl};

    pub use crate::__private::downcast::{Downcast, IDowncast, IDowncastImpl};
    pub use crate::__private::vtable::Type;
    pub use crate::object::{EqHash, Format, IEqHash, IFormat, IObject, Object};

    #[cfg(feature = "dyn-hash")]
    pub use dyn_hash::DynHash;
    #[cfg(feature = "dyn-hash")]
    pub type DynHasher<'a> = dyn core::hash::Hasher + 'a;
}
