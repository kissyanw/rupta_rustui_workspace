pub use type_info::{
    ClassVtableTypeInfo, InterfaceVtableTypeInfo, MixinInstanceVtableTypeInfo, MixinVtableTypeInfo,
    VtableOffsetEntry, VtableTypeInfo, Type,
};

use crate::class::ClassConcrete;

#[cfg(test)]
mod tests;
mod type_info;

pub trait HasVtable {
    type Vtable: Vtable;
}

pub trait ClassHasVtable: HasVtable<Vtable: ClassVtable> {}

pub trait HasVtableImpl: HasVtable {
    const VTABLE: Self::Vtable;
}

pub trait HasSuperVtable {
    type SuperVtable: Vtable;
}

pub trait HasSuperVtableImpl: HasSuperVtable {
    const SUPER_VTABLE: Self::SuperVtable;
}

pub trait ClassHasVtableImpl: ClassHasVtable + HasVtableImpl<Vtable: ClassVtable> {}

impl<T: ClassHasVtable + HasVtableImpl<Vtable: ClassVtable>> ClassHasVtableImpl for T {}

pub trait DynHasVtable {
    type Vtable<__Self: ClassConcrete>: Vtable;
}

pub trait Vtable: Copy + Sized {
    const TYPE: VtableTypeInfo<'_>;
    const INTERFACE_TYPE: &InterfaceVtableTypeInfo<'_> = Self::TYPE.as_interface();
}

pub trait MixinVtable: Vtable {
    const MIXIN_TYPE: &MixinVtableTypeInfo<'_> = Self::TYPE.expect_mixin();
}

pub trait MixinInstanceVtable: Vtable {
    const MIXIN_INSTANCE_TYPE: &MixinInstanceVtableTypeInfo<'_> =
        Self::TYPE.expect_mixin_instance();
}

pub trait ClassVtable: Vtable {
    const CLASS_TYPE: &ClassVtableTypeInfo<'_> = Self::TYPE.expect_class();
}

macro_rules! r#try {
    ($expr:expr $(,)?) => {
        match $expr {
            Some(val) => val,
            None => return None,
        }
    };
}

pub const fn cast_vtable<__Self: Vtable, __Super: Vtable>(vtable: &__Self) -> Option<&__Super> {
    let offset = r#try!(__Self::TYPE.offset_of(__Super::TYPE));
    Some(unsafe { &*core::ptr::from_ref(vtable).byte_add(offset).cast() })
}

pub const fn cast_vtable_mut<__Self: Vtable, __Super: Vtable>(
    vtable: &mut __Self,
) -> Option<(&mut __Super, usize)> {
    let offset = r#try!(__Self::TYPE.offset_of(__Super::TYPE));
    let __super = unsafe { &mut *core::ptr::from_mut(vtable).byte_add(offset).cast() };
    Some((__super, offset))
}

pub const fn cast_vtable_mut_next<'v, __Self: Vtable, __Super: Vtable>(
    vtable: &'v mut __Self,
    last_offset: &mut usize,
) -> Option<&'v mut __Super> {
    let offset = r#try!(__Self::TYPE.next_offset_of(__Super::TYPE, *last_offset));
    *last_offset = offset;
    Some(unsafe { &mut *core::ptr::from_mut(vtable).byte_add(offset).cast() })
}

#[macro_export]
macro_rules! write_vtable {
    ($vtable:ident.$method:ident = $expr:expr) => {
        $vtable.$method = ::core::option::Option::Some($expr);
    };
    (($vtable:ident as Downcast).$method:ident = $expr:expr) => {
        $crate::write_vtable!(try ($vtable as Downcast => $crate::prelude::CVtable<Downcast, __Self>).$method = $expr)
    };
    (($vtable:ident as $Class:path).$method:ident = $expr:expr) => {
        $crate::write_vtable!(($vtable as $Class => $crate::prelude::CVtable<$Class, __Self>).$method = $expr)
    };
    (try ($vtable:ident as $Class:path => $Vtable:path).$method:ident = $expr:expr) => {'downcast: {
        let vtable = &mut *$vtable;
        let ::core::option::Option::Some((v, mut offset)) =
            $crate::__private::vtable::cast_vtable_mut::<_, $Vtable>(vtable)
        else {
            break 'downcast ::core::option::Option::None;
        };
        v.$method = ::core::option::Option::Some($expr);
        while let ::core::option::Option::Some(v) = $crate::__private::vtable::cast_vtable_mut_next::<_, $Vtable>(
            vtable,
            &mut offset,
        ) {
            v.$method = ::core::option::Option::Some($expr);
        }
        ::core::option::Option::Some(())
    }};
    (($vtable:ident as $Class:path => $Vtable:path).$method:ident = $expr:expr) => {
        $crate::write_vtable!(try ($vtable as $Class => $Vtable).$method = $expr)
            .expect(concat!("not a subtype of `", stringify!($Class), "`"))
    };
}

#[macro_export]
macro_rules! read_vtable {
    ($method:ident) => {
        const {
            $crate::__private::vtable::cast_vtable::<_, $crate::prelude::CVtable<__Class, __Self>>(
                &<__Self as $crate::__private::vtable::HasVtableImpl>::VTABLE,
            )
            .expect(concat!("not a subtype"))
            .$method
            .expect(concat!("method `", stringify!($method),
                "` is not implemented"
            ))
        }
    };
    (<$Class:path>::$method:ident) => {
        const {
            $crate::__private::vtable::cast_vtable::<_, $crate::prelude::CVtable<$Class, __Self>>(
                &<__Self as $crate::__private::vtable::HasVtableImpl>::VTABLE,
            )
            .expect(concat!("not a subtype of `", stringify!($Class), "`"))
            .$method
            .expect(concat!(
                "method `",
                stringify!($Class),
                "::",
                stringify!($method),
                "` is not implemented"
            ))
        }
    };
    (<Super as $Class:path>::$method:ident) => {
        const {
            $crate::__private::vtable::cast_vtable::<_, $crate::prelude::CVtable<$Class, __Self>>(
                &<Self as $crate::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE,
            )
            .expect(concat!("not a subtype of `", stringify!($Class), "`"))
            .$method
            .expect(concat!(
                "method `",
                stringify!($Class),
                "::",
                stringify!($method),
                "` is not implemented"
            ))
        }
    };
    (<Super as [$($Class:path),* $(,)?]>::$method:ident) => {
        const { 'result: {
            $(
                if let Some(method) = $crate::read_vtable!(try <Super as $Class>::$method) {
                    break 'result method;
                }
            )*
            panic!(concat!("method `", stringify!($method), "` is unimplemented in none of ", $("`", stringify!($Class), "`, "),*))
        } }
    };
    (try <Super as $Class:path>::$method:ident) => {
        const {
            match $crate::__private::vtable::cast_vtable::<_, $crate::prelude::CVtable<$Class, __Self>>(
                &<Self as $crate::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE,
            ) {
                ::core::option::Option::Some(vtable) => vtable.$method,
                ::core::option::Option::None => ::core::option::Option::None,
            }
        }
    };
    (try <$interface:path as $Class:path>::$method:ident) => {
        const {
            match $crate::__private::vtable::cast_vtable::<_, $crate::prelude::CVtable<$Class, __Self>>(
                &<$crate::prelude::CClass<$interface, __Self> as $crate::__private::vtable::HasVtableImpl>::VTABLE,
            ) {
                ::core::option::Option::Some(vtable) => vtable.$method,
                ::core::option::Option::None => ::core::option::Option::None,
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vtable_copy {
    (<($($generics:tt)*)> $vtable:ty) => {
        #[automatically_derived]
        impl<$($generics)*> ::core::clone::Clone for $vtable {
            fn clone(&self) -> Self {
                *self
            }
        }
        #[automatically_derived]
        impl<$($generics)*> ::core::marker::Copy for $vtable {}
    };
}
