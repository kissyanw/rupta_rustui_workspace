#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use oop_rs::prelude::*;

#[allow(unused_imports)]
use __Animal::{Animal, IAnimal};
#[allow(non_snake_case)]
#[allow(private_interfaces)]
mod __Animal {
    #[allow(unused_imports)]
    use ::oop_rs::prelude::*;
    #[allow(unused_imports)]
    use super::*;
    const __TYPE_NAME: &str = "min_class_downcast::__Animal::Animal";
    #[repr(C)]
    pub struct __SAnimal<__Self: ::oop_rs::class::ClassConcrete> {
        __super: ::oop_rs::prelude::CClass<Object, __Self>,
        __self: ::core::marker::PhantomData<__Self>,
        data: __DAnimal,
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::data::HasData for __SAnimal<__Self> {
        type Data = __DAnimal;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::data::ClassHasData for __SAnimal<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::HasVtable for __SAnimal<__Self> {
        type Vtable = __VAnimal<__Self>;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::ClassHasVtable for __SAnimal<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IObject +
        ::oop_rs::rc::RcRef> ::oop_rs::__private::vtable::HasVtableImpl for
        __SAnimal<__Self> {
        const VTABLE: Self::Vtable = __SAnimal::<__Self>::__VTABLE;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::dynamic::ClassHasDyn for __SAnimal<__Self> {
        type Dyn = Animal;
    }
    #[automatically_derived]
    unsafe impl<__Self: ::oop_rs::class::ClassConcrete + ::oop_rs::rc::RcRef>
        ::oop_rs::rc::RcRef for __SAnimal<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::oop_rs::class::HasSuper for
        __SAnimal<__Self> {
        type Super = ::oop_rs::prelude::CClass<Object, __Self>;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::HasSuperVtable for __SAnimal<__Self> {
        type SuperVtable =
            <::oop_rs::prelude::CClass<Object, __Self> as
            ::oop_rs::__private::vtable::HasVtable>::Vtable;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IObject +
        ::oop_rs::rc::RcRef> ::oop_rs::__private::vtable::HasSuperVtableImpl
        for __SAnimal<__Self> {
        const SUPER_VTABLE: Self::SuperVtable =
            <::oop_rs::prelude::CClass<Object, __Self> as
                ::oop_rs::__private::vtable::HasVtableImpl>::VTABLE;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::ops::Deref for
        __SAnimal<__Self> {
        type Target = ::oop_rs::prelude::CClass<Object, __Self>;
        fn deref(&self) -> &Self::Target { &self.__super }
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IObject +
        ::oop_rs::rc::RcRef> __SAnimal<__Self> {
        const __VTABLE: __VAnimal<__Self> =
            {
                let mut vtable =
                    __VAnimal {
                        __super: <__SAnimal<__Self> as
                            ::oop_rs::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE,
                        __self: ::core::marker::PhantomData,
                    };
                Self::__override(&mut vtable);
                vtable
            };
        #[allow(unused_variables)]
        const fn __override(vtable: &mut __VAnimal<__Self>) {







            'downcast:
                {
                let vtable = &mut *vtable;
                let ::core::option::Option::Some((v, mut offset)) =
                    ::oop_rs::__private::vtable::cast_vtable_mut::<_,
                            ::oop_rs::prelude::CVtable<Downcast,
                            __Self>>(vtable) else {
                        break 'downcast ::core::option::Option::None;
                    };
                v.ty = ::core::option::Option::Some(Self::ty);
                while let ::core::option::Option::Some(v) =
                        ::oop_rs::__private::vtable::cast_vtable_mut_next::<_,
                                ::oop_rs::prelude::CVtable<Downcast,
                                __Self>>(vtable, &mut offset) {
                    v.ty = ::core::option::Option::Some(Self::ty);
                }
                ::core::option::Option::Some(())
            };
            'downcast:
                {
                let vtable = &mut *vtable;
                let ::core::option::Option::Some((v, mut offset)) =
                    ::oop_rs::__private::vtable::cast_vtable_mut::<_,
                            ::oop_rs::prelude::CVtable<Downcast,
                            __Self>>(vtable) else {
                        break 'downcast ::core::option::Option::None;
                    };
                v.__downcast = ::core::option::Option::Some(Self::__downcast);
                while let ::core::option::Option::Some(v) =
                        ::oop_rs::__private::vtable::cast_vtable_mut_next::<_,
                                ::oop_rs::prelude::CVtable<Downcast,
                                __Self>>(vtable, &mut offset) {
                    v.__downcast =
                        ::core::option::Option::Some(Self::__downcast);
                }
                ::core::option::Option::Some(())
            };
        }
        pub fn new(mut __self: ::oop_rs::rc::CRcUninit<__Self, Self>)
            -> ::oop_rs::rc::CRc<__Self> {
            let __data = __DAnimal {};
            let __self =
                unsafe {
                    ::core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self,
                        ::core::marker::PhantomData);
                    ::core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data,
                        __data);
                    __self.assume_init_except()
                };
            let __self =
                <Self as ::oop_rs::class::HasSuper>::Super::new(__self);
            let __self_dyn = &*__self as &Animal;
            __self
        }
        fn ty(__self: &__Self) -> ::oop_rs::__private::vtable::Type {
            ::oop_rs::__private::vtable::Type::of::<Animal>()
        }
        fn __downcast(__self: &__Self, ty: ::oop_rs::__private::vtable::Type)
            -> Option<::oop_rs::__private::downcast::ErasedPtr> {
            if ty == ::oop_rs::__private::vtable::Type::of::<Animal>() {
                return ::core::option::Option::Some(unsafe {
                            ::core::mem::transmute(__self as &Animal)
                        });
            }
            if let ::core::option::Option::Some(__vtable) =
                        const {
                                match ::oop_rs::__private::vtable::cast_vtable::<_,
                                            ::oop_rs::prelude::CVtable<::oop_rs::__private::downcast::Downcast,
                                            __Self>>(&<Self as
                                                ::oop_rs::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE)
                                    {
                                    ::core::option::Option::Some(vtable) => vtable.__downcast,
                                    ::core::option::Option::None =>
                                        ::core::option::Option::None,
                                }
                            } &&
                    let ::core::option::Option::Some(__super) =
                        __vtable(__self, ty) {
                return ::core::option::Option::Some(__super);
            }
            ::core::option::Option::None
        }
    }
    pub struct __DAnimal {}
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassOrMixinData for __DAnimal {
        const TYPE: ::oop_rs::__private::data::DataTypeInfo<'static> =
            ::oop_rs::__private::data::DataTypeInfo::Class(&::oop_rs::__private::data::ClassDataTypeInfo::new(__TYPE_NAME).with_super(<::oop_rs::prelude::CData<Object>
                                as ::oop_rs::__private::data::ClassData>::CLASS_TYPE,
                            {
                                builtin # offset_of(__SAnimal<::oop_rs::__private::Dummy>,
                                    data)
                            }).with_mixins(&[]));
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassData for __DAnimal { }
    #[allow(dead_code)]
    pub trait IAnimal: IObject + ::oop_rs::rc::RcRef {
        fn __assert_subclass(&self) where
            Self: ::oop_rs::class::ClassConcrete + ::core::marker::Sized {
            const {
                    let _ =
                        <<Self as ::oop_rs::__private::data::HasData>::Data as
                                    ::oop_rs::__private::data::ClassData>::CLASS_TYPE.offset_of_class(&<<Animal
                                        as ::oop_rs::__private::data::HasData>::Data as
                                        ::oop_rs::__private::data::ClassData>::CLASS_TYPE).expect("not a subclass of `Animal`");
                }
        }
    }
    pub type Animal = dyn IAnimal;
    type __Class = Animal;
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete +
        ::oop_rs::class::ClassConcrete + IObject + ::oop_rs::rc::RcRef>
        IAnimal for __Self {
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassHasDyn for Animal {
        type Dyn = Animal;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::HasData for Animal {
        type Data = __DAnimal;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::DynHasVtable for Animal {
        type Vtable<__Self: ::oop_rs::class::ClassConcrete> =
            __VAnimal<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassConcreteOrDyn for Animal {
        const TYPE: &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo
            =
            &(::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo::new_dyn(__TYPE_NAME).with_super::<Self>());
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassDyn for Animal {
        type Class<__Self: ::oop_rs::class::ClassConcrete> =
            __SAnimal<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::class::HasSuper for Animal {
        type Super = Object;
    }
    #[automatically_derived]
    impl ::core::ops::Deref for Animal {
        type Target = Object;
        fn deref(&self) -> &Self::Target { self }
    }
    #[repr(C)]
    pub struct __VAnimal<__Self: ::oop_rs::class::ClassConcrete> {
        __super: <__SAnimal<__Self> as
        ::oop_rs::__private::vtable::HasSuperVtable>::SuperVtable,
        __self: ::core::marker::PhantomData<__Self>,
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::clone::Clone for
        __VAnimal<__Self> {
        fn clone(&self) -> Self { *self }
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::marker::Copy for
        __VAnimal<__Self> {
    }
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::Vtable for __VAnimal<__Self> {
        const TYPE: ::oop_rs::__private::vtable::VtableTypeInfo<'static> =
            ::oop_rs::__private::vtable::VtableTypeInfo::Class(&(::oop_rs::__private::vtable::ClassVtableTypeInfo::new(__TYPE_NAME).with_super(<::oop_rs::prelude::CVtable<Object,
                                __Self> as ::oop_rs::__private::vtable::Vtable>::TYPE)));
    }
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::ClassVtable for __VAnimal<__Self> {}
    type __Self = __CAnimal;
    #[repr(transparent)]
    pub struct __CAnimal(__SAnimal<__Self>);
    #[automatically_derived]
    impl ::core::ops::Deref for __CAnimal {
        type Target = Animal;
        fn deref(&self) -> &Self::Target { self }
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassHasDyn for __CAnimal {
        type Dyn = Animal;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::HasData for __CAnimal {
        type Data = __DAnimal;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassHasData for __CAnimal { }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassConcreteOrDyn for __CAnimal {
        const TYPE: &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo
            =
            &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo::new_concrete(__TYPE_NAME).with_super::<Self>();
    }
    #[automatically_derived]
    impl ::oop_rs::class::ClassConcrete for __CAnimal { }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::HasVtable for __CAnimal {
        type Vtable = __VAnimal<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::HasVtableImpl for __CAnimal {
        const VTABLE: Self::Vtable = __SAnimal::<__Self>::__VTABLE;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::ClassHasVtable for __CAnimal { }
    #[automatically_derived]
    impl ::oop_rs::class::HasSuper for __CAnimal {
        type Super = Object;
    }
    #[automatically_derived]
    unsafe impl ::oop_rs::rc::RcRef for __CAnimal { }
    impl Animal {
        pub fn new() -> ::oop_rs::rc::CRc<__Self> {
            __SAnimal::new(::oop_rs::rc::CRcUninit::new().cast_uninit())
        }
    }
    impl __CAnimal {}
    impl Animal {}
    impl Animal {
        fn __data(&self) -> &__DAnimal {
            let offset =
                const {
                        <__DAnimal as
                                ::oop_rs::__private::data::ClassData>::CLASS_TYPE.offset()
                    };
            unsafe { &*::core::ptr::from_ref(self).byte_add(offset).cast() }
        }
    }
    #[repr(transparent)]
    pub struct __AAnimal<__M: ::oop_rs::__private::accessor::AccessMode>(::core::marker::PhantomData<__M>,
        Animal);
    #[automatically_derived]
    impl<__M: ::oop_rs::__private::accessor::AccessMode> ::core::ops::Deref
        for __AAnimal<__M> {
        type Target =
            <Object as ::oop_rs::__private::accessor::Access>::Accessor<__M>;
        fn deref(&self) -> &Self::Target {
            <Object as
                    ::oop_rs::__private::accessor::Access>::__access::<__M>(&self.1)
        }
    }
    #[automatically_derived]
    unsafe impl ::oop_rs::__private::accessor::Access for Animal {
        type Accessor<__M: ::oop_rs::__private::accessor::AccessMode> =
            __AAnimal<__M>;
    }
    impl __AAnimal<::oop_rs::__private::accessor::Get> {}
    impl __AAnimal<::oop_rs::__private::accessor::GetMut> {}
    impl __AAnimal<::oop_rs::__private::accessor::Set> {}
    impl __AAnimal<::oop_rs::__private::accessor::Update> {}
    impl __AAnimal<::oop_rs::__private::accessor::Replace> {}
    impl __AAnimal<::oop_rs::__private::accessor::ReplaceWith> {}
    impl __AAnimal<::oop_rs::__private::accessor::Raw> {}
}
#[allow(unused_imports)]
use __Dog::{Dog, IDog};
#[allow(non_snake_case)]
#[allow(private_interfaces)]
mod __Dog {
    #[allow(unused_imports)]
    use ::oop_rs::prelude::*;
    #[allow(unused_imports)]
    use super::*;
    const __TYPE_NAME: &str = "min_class_downcast::__Dog::Dog";
    #[repr(C)]
    pub struct __SDog<__Self: ::oop_rs::class::ClassConcrete> {
        __super: ::oop_rs::prelude::CClass<Animal, __Self>,
        __self: ::core::marker::PhantomData<__Self>,
        data: __DDog,
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::data::HasData for __SDog<__Self> {
        type Data = __DDog;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::data::ClassHasData for __SDog<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::HasVtable for __SDog<__Self> {
        type Vtable = __VDog<__Self>;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::ClassHasVtable for __SDog<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IAnimal +
        ::oop_rs::rc::RcRef> ::oop_rs::__private::vtable::HasVtableImpl for
        __SDog<__Self> {
        const VTABLE: Self::Vtable = __SDog::<__Self>::__VTABLE;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::dynamic::ClassHasDyn for __SDog<__Self> {
        type Dyn = Dog;
    }
    #[automatically_derived]
    unsafe impl<__Self: ::oop_rs::class::ClassConcrete + ::oop_rs::rc::RcRef>
        ::oop_rs::rc::RcRef for __SDog<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::oop_rs::class::HasSuper for
        __SDog<__Self> {
        type Super = ::oop_rs::prelude::CClass<Animal, __Self>;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::HasSuperVtable for __SDog<__Self> {
        type SuperVtable =
            <::oop_rs::prelude::CClass<Animal, __Self> as
            ::oop_rs::__private::vtable::HasVtable>::Vtable;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IAnimal +
        ::oop_rs::rc::RcRef> ::oop_rs::__private::vtable::HasSuperVtableImpl
        for __SDog<__Self> {
        const SUPER_VTABLE: Self::SuperVtable =
            <::oop_rs::prelude::CClass<Animal, __Self> as
                ::oop_rs::__private::vtable::HasVtableImpl>::VTABLE;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::ops::Deref for
        __SDog<__Self> {
        type Target = ::oop_rs::prelude::CClass<Animal, __Self>;
        fn deref(&self) -> &Self::Target { &self.__super }
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IAnimal +
        ::oop_rs::rc::RcRef> __SDog<__Self> {
        const __VTABLE: __VDog<__Self> =
            {
                let mut vtable =
                    __VDog {
                        __super: <__SDog<__Self> as
                            ::oop_rs::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE,
                        __self: ::core::marker::PhantomData,
                    };
                Self::__override(&mut vtable);
                vtable
            };
        #[allow(unused_variables)]
        const fn __override(vtable: &mut __VDog<__Self>) {
            'downcast:
                {
                let vtable = &mut *vtable;
                let ::core::option::Option::Some((v, mut offset)) =
                    ::oop_rs::__private::vtable::cast_vtable_mut::<_,
                            ::oop_rs::prelude::CVtable<Downcast,
                            __Self>>(vtable) else {
                        break 'downcast ::core::option::Option::None;
                    };
                v.ty = ::core::option::Option::Some(Self::ty);
                while let ::core::option::Option::Some(v) =
                        ::oop_rs::__private::vtable::cast_vtable_mut_next::<_,
                                ::oop_rs::prelude::CVtable<Downcast,
                                __Self>>(vtable, &mut offset) {
                    v.ty = ::core::option::Option::Some(Self::ty);
                }
                ::core::option::Option::Some(())
            };
            'downcast:
                {
                let vtable = &mut *vtable;
                let ::core::option::Option::Some((v, mut offset)) =
                    ::oop_rs::__private::vtable::cast_vtable_mut::<_,
                            ::oop_rs::prelude::CVtable<Downcast,
                            __Self>>(vtable) else {
                        break 'downcast ::core::option::Option::None;
                    };
                v.__downcast = ::core::option::Option::Some(Self::__downcast);
                while let ::core::option::Option::Some(v) =
                        ::oop_rs::__private::vtable::cast_vtable_mut_next::<_,
                                ::oop_rs::prelude::CVtable<Downcast,
                                __Self>>(vtable, &mut offset) {
                    v.__downcast =
                        ::core::option::Option::Some(Self::__downcast);
                }
                ::core::option::Option::Some(())
            };
        }
        pub fn new(mut __self: ::oop_rs::rc::CRcUninit<__Self, Self>)
            -> ::oop_rs::rc::CRc<__Self> {
            let __data = __DDog {};
            let __self =
                unsafe {
                    ::core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self,
                        ::core::marker::PhantomData);
                    ::core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data,
                        __data);
                    __self.assume_init_except()
                };
            let __self =
                <Self as ::oop_rs::class::HasSuper>::Super::new(__self);
            let __self_dyn = &*__self as &Dog;
            __self
        }
        fn ty(__self: &__Self) -> ::oop_rs::__private::vtable::Type {
            ::oop_rs::__private::vtable::Type::of::<Dog>()
        }
        fn __downcast(__self: &__Self, ty: ::oop_rs::__private::vtable::Type)
            -> Option<::oop_rs::__private::downcast::ErasedPtr> {
            if ty == ::oop_rs::__private::vtable::Type::of::<Dog>() {
                return ::core::option::Option::Some(unsafe {
                            ::core::mem::transmute(__self as &Dog)
                        });
            }
            if let ::core::option::Option::Some(__vtable) =
                        const {
                                match ::oop_rs::__private::vtable::cast_vtable::<_,
                                            ::oop_rs::prelude::CVtable<::oop_rs::__private::downcast::Downcast,
                                            __Self>>(&<Self as
                                                ::oop_rs::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE)
                                    {
                                    ::core::option::Option::Some(vtable) => vtable.__downcast,
                                    ::core::option::Option::None =>
                                        ::core::option::Option::None,
                                }
                            } &&
                    let ::core::option::Option::Some(__super) =
                        __vtable(__self, ty) {
                return ::core::option::Option::Some(__super);
            }
            ::core::option::Option::None
        }
    }
    pub struct __DDog {}
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassOrMixinData for __DDog {
        const TYPE: ::oop_rs::__private::data::DataTypeInfo<'static> =
            ::oop_rs::__private::data::DataTypeInfo::Class(&::oop_rs::__private::data::ClassDataTypeInfo::new(__TYPE_NAME).with_super(<::oop_rs::prelude::CData<Animal>
                                as ::oop_rs::__private::data::ClassData>::CLASS_TYPE,
                            {
                                builtin # offset_of(__SDog<::oop_rs::__private::Dummy>,
                                    data)
                            }).with_mixins(&[]));
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassData for __DDog { }
    #[allow(dead_code)]
    pub trait IDog: IAnimal + ::oop_rs::rc::RcRef {
        fn __assert_subclass(&self) where
            Self: ::oop_rs::class::ClassConcrete + ::core::marker::Sized {
            const {
                    let _ =
                        <<Self as ::oop_rs::__private::data::HasData>::Data as
                                    ::oop_rs::__private::data::ClassData>::CLASS_TYPE.offset_of_class(&<<Dog
                                        as ::oop_rs::__private::data::HasData>::Data as
                                        ::oop_rs::__private::data::ClassData>::CLASS_TYPE).expect("not a subclass of `Dog`");
                }
        }
    }
    pub type Dog = dyn IDog;
    type __Class = Dog;
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete +
        ::oop_rs::class::ClassConcrete + IAnimal + ::oop_rs::rc::RcRef> IDog
        for __Self {
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassHasDyn for Dog {
        type Dyn = Dog;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::HasData for Dog {
        type Data = __DDog;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::DynHasVtable for Dog {
        type Vtable<__Self: ::oop_rs::class::ClassConcrete> = __VDog<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassConcreteOrDyn for Dog {
        const TYPE: &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo
            =
            &(::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo::new_dyn(__TYPE_NAME).with_super::<Self>());
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassDyn for Dog {
        type Class<__Self: ::oop_rs::class::ClassConcrete> = __SDog<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::class::HasSuper for Dog {
        type Super = Animal;
    }
    #[automatically_derived]
    impl ::core::ops::Deref for Dog {
        type Target = Animal;
        fn deref(&self) -> &Self::Target { self }
    }
    #[repr(C)]
    pub struct __VDog<__Self: ::oop_rs::class::ClassConcrete> {
        __super: <__SDog<__Self> as
        ::oop_rs::__private::vtable::HasSuperVtable>::SuperVtable,
        __self: ::core::marker::PhantomData<__Self>,
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::clone::Clone for
        __VDog<__Self> {
        fn clone(&self) -> Self { *self }
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::marker::Copy for
        __VDog<__Self> {
    }
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::Vtable for __VDog<__Self> {
        const TYPE: ::oop_rs::__private::vtable::VtableTypeInfo<'static> =
            ::oop_rs::__private::vtable::VtableTypeInfo::Class(&(::oop_rs::__private::vtable::ClassVtableTypeInfo::new(__TYPE_NAME).with_super(<::oop_rs::prelude::CVtable<Animal,
                                __Self> as ::oop_rs::__private::vtable::Vtable>::TYPE)));
    }
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::ClassVtable for __VDog<__Self> {}
    type __Self = __CDog;
    #[repr(transparent)]
    pub struct __CDog(__SDog<__Self>);
    #[automatically_derived]
    impl ::core::ops::Deref for __CDog {
        type Target = Dog;
        fn deref(&self) -> &Self::Target { self }
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassHasDyn for __CDog {
        type Dyn = Dog;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::HasData for __CDog {
        type Data = __DDog;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassHasData for __CDog { }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassConcreteOrDyn for __CDog {
        const TYPE: &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo
            =
            &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo::new_concrete(__TYPE_NAME).with_super::<Self>();
    }
    #[automatically_derived]
    impl ::oop_rs::class::ClassConcrete for __CDog { }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::HasVtable for __CDog {
        type Vtable = __VDog<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::HasVtableImpl for __CDog {
        const VTABLE: Self::Vtable = __SDog::<__Self>::__VTABLE;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::ClassHasVtable for __CDog { }
    #[automatically_derived]
    impl ::oop_rs::class::HasSuper for __CDog {
        type Super = Animal;
    }
    #[automatically_derived]
    unsafe impl ::oop_rs::rc::RcRef for __CDog { }
    impl Dog {
        pub fn new() -> ::oop_rs::rc::CRc<__Self> {
            __SDog::new(::oop_rs::rc::CRcUninit::new().cast_uninit())
        }
    }
    impl __CDog {}
    impl Dog {}
    impl Dog {
        fn __data(&self) -> &__DDog {
            let offset =
                const {
                        <__DDog as
                                ::oop_rs::__private::data::ClassData>::CLASS_TYPE.offset()
                    };
            unsafe { &*::core::ptr::from_ref(self).byte_add(offset).cast() }
        }
    }
    #[repr(transparent)]
    pub struct __ADog<__M: ::oop_rs::__private::accessor::AccessMode>(::core::marker::PhantomData<__M>,
        Dog);
    #[automatically_derived]
    impl<__M: ::oop_rs::__private::accessor::AccessMode> ::core::ops::Deref
        for __ADog<__M> {
        type Target =
            <Animal as ::oop_rs::__private::accessor::Access>::Accessor<__M>;
        fn deref(&self) -> &Self::Target {
            <Animal as
                    ::oop_rs::__private::accessor::Access>::__access::<__M>(&self.1)
        }
    }
    #[automatically_derived]
    unsafe impl ::oop_rs::__private::accessor::Access for Dog {
        type Accessor<__M: ::oop_rs::__private::accessor::AccessMode> =
            __ADog<__M>;
    }
    impl __ADog<::oop_rs::__private::accessor::Get> {}
    impl __ADog<::oop_rs::__private::accessor::GetMut> {}
    impl __ADog<::oop_rs::__private::accessor::Set> {}
    impl __ADog<::oop_rs::__private::accessor::Update> {}
    impl __ADog<::oop_rs::__private::accessor::Replace> {}
    impl __ADog<::oop_rs::__private::accessor::ReplaceWith> {}
    impl __ADog<::oop_rs::__private::accessor::Raw> {}
}
#[allow(unused_imports)]
use __Cat::{Cat, ICat};
#[allow(non_snake_case)]
#[allow(private_interfaces)]
mod __Cat {
    #[allow(unused_imports)]
    use ::oop_rs::prelude::*;
    #[allow(unused_imports)]
    use super::*;
    const __TYPE_NAME: &str = "min_class_downcast::__Cat::Cat";
    #[repr(C)]
    pub struct __SCat<__Self: ::oop_rs::class::ClassConcrete> {
        __super: ::oop_rs::prelude::CClass<Animal, __Self>,
        __self: ::core::marker::PhantomData<__Self>,
        data: __DCat,
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::data::HasData for __SCat<__Self> {
        type Data = __DCat;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::data::ClassHasData for __SCat<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::HasVtable for __SCat<__Self> {
        type Vtable = __VCat<__Self>;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::ClassHasVtable for __SCat<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IAnimal +
        ::oop_rs::rc::RcRef> ::oop_rs::__private::vtable::HasVtableImpl for
        __SCat<__Self> {
        const VTABLE: Self::Vtable = __SCat::<__Self>::__VTABLE;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::dynamic::ClassHasDyn for __SCat<__Self> {
        type Dyn = Cat;
    }
    #[automatically_derived]
    unsafe impl<__Self: ::oop_rs::class::ClassConcrete + ::oop_rs::rc::RcRef>
        ::oop_rs::rc::RcRef for __SCat<__Self> {
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::oop_rs::class::HasSuper for
        __SCat<__Self> {
        type Super = ::oop_rs::prelude::CClass<Animal, __Self>;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::HasSuperVtable for __SCat<__Self> {
        type SuperVtable =
            <::oop_rs::prelude::CClass<Animal, __Self> as
            ::oop_rs::__private::vtable::HasVtable>::Vtable;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IAnimal +
        ::oop_rs::rc::RcRef> ::oop_rs::__private::vtable::HasSuperVtableImpl
        for __SCat<__Self> {
        const SUPER_VTABLE: Self::SuperVtable =
            <::oop_rs::prelude::CClass<Animal, __Self> as
                ::oop_rs::__private::vtable::HasVtableImpl>::VTABLE;
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::ops::Deref for
        __SCat<__Self> {
        type Target = ::oop_rs::prelude::CClass<Animal, __Self>;
        fn deref(&self) -> &Self::Target { &self.__super }
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete + IAnimal +
        ::oop_rs::rc::RcRef> __SCat<__Self> {
        const __VTABLE: __VCat<__Self> =
            {
                let mut vtable =
                    __VCat {
                        __super: <__SCat<__Self> as
                            ::oop_rs::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE,
                        __self: ::core::marker::PhantomData,
                    };
                Self::__override(&mut vtable);
                vtable
            };
        #[allow(unused_variables)]
        const fn __override(vtable: &mut __VCat<__Self>) {
            'downcast:
                {
                let vtable = &mut *vtable;
                let ::core::option::Option::Some((v, mut offset)) =
                    ::oop_rs::__private::vtable::cast_vtable_mut::<_,
                            ::oop_rs::prelude::CVtable<Downcast,
                            __Self>>(vtable) else {
                        break 'downcast ::core::option::Option::None;
                    };
                v.ty = ::core::option::Option::Some(Self::ty);
                while let ::core::option::Option::Some(v) =
                        ::oop_rs::__private::vtable::cast_vtable_mut_next::<_,
                                ::oop_rs::prelude::CVtable<Downcast,
                                __Self>>(vtable, &mut offset) {
                    v.ty = ::core::option::Option::Some(Self::ty);
                }
                ::core::option::Option::Some(())
            };
            'downcast:
                {
                let vtable = &mut *vtable;
                let ::core::option::Option::Some((v, mut offset)) =
                    ::oop_rs::__private::vtable::cast_vtable_mut::<_,
                            ::oop_rs::prelude::CVtable<Downcast,
                            __Self>>(vtable) else {
                        break 'downcast ::core::option::Option::None;
                    };
                v.__downcast = ::core::option::Option::Some(Self::__downcast);
                while let ::core::option::Option::Some(v) =
                        ::oop_rs::__private::vtable::cast_vtable_mut_next::<_,
                                ::oop_rs::prelude::CVtable<Downcast,
                                __Self>>(vtable, &mut offset) {
                    v.__downcast =
                        ::core::option::Option::Some(Self::__downcast);
                }
                ::core::option::Option::Some(())
            };
        }
        pub fn new(mut __self: ::oop_rs::rc::CRcUninit<__Self, Self>)
            -> ::oop_rs::rc::CRc<__Self> {
            let __data = __DCat {};
            let __self =
                unsafe {
                    ::core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).__self,
                        ::core::marker::PhantomData);
                    ::core::ptr::write(&raw mut (*__self.as_uninit_mut_ptr()).data,
                        __data);
                    __self.assume_init_except()
                };
            let __self =
                <Self as ::oop_rs::class::HasSuper>::Super::new(__self);
            let __self_dyn = &*__self as &Cat;
            __self
        }
        fn ty(__self: &__Self) -> ::oop_rs::__private::vtable::Type {
            ::oop_rs::__private::vtable::Type::of::<Cat>()
        }
        fn __downcast(__self: &__Self, ty: ::oop_rs::__private::vtable::Type)
            -> Option<::oop_rs::__private::downcast::ErasedPtr> {
            if ty == ::oop_rs::__private::vtable::Type::of::<Cat>() {
                return ::core::option::Option::Some(unsafe {
                            ::core::mem::transmute(__self as &Cat)
                        });
            }
            if let ::core::option::Option::Some(__vtable) =
                        const {
                                match ::oop_rs::__private::vtable::cast_vtable::<_,
                                            ::oop_rs::prelude::CVtable<::oop_rs::__private::downcast::Downcast,
                                            __Self>>(&<Self as
                                                ::oop_rs::__private::vtable::HasSuperVtableImpl>::SUPER_VTABLE)
                                    {
                                    ::core::option::Option::Some(vtable) => vtable.__downcast,
                                    ::core::option::Option::None =>
                                        ::core::option::Option::None,
                                }
                            } &&
                    let ::core::option::Option::Some(__super) =
                        __vtable(__self, ty) {
                return ::core::option::Option::Some(__super);
            }
            ::core::option::Option::None
        }
    }
    pub struct __DCat {}
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassOrMixinData for __DCat {
        const TYPE: ::oop_rs::__private::data::DataTypeInfo<'static> =
            ::oop_rs::__private::data::DataTypeInfo::Class(&::oop_rs::__private::data::ClassDataTypeInfo::new(__TYPE_NAME).with_super(<::oop_rs::prelude::CData<Animal>
                                as ::oop_rs::__private::data::ClassData>::CLASS_TYPE,
                            {
                                builtin # offset_of(__SCat<::oop_rs::__private::Dummy>,
                                    data)
                            }).with_mixins(&[]));
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassData for __DCat { }
    #[allow(dead_code)]
    pub trait ICat: IAnimal + ::oop_rs::rc::RcRef {
        fn __assert_subclass(&self) where
            Self: ::oop_rs::class::ClassConcrete + ::core::marker::Sized {
            const {
                    let _ =
                        <<Self as ::oop_rs::__private::data::HasData>::Data as
                                    ::oop_rs::__private::data::ClassData>::CLASS_TYPE.offset_of_class(&<<Cat
                                        as ::oop_rs::__private::data::HasData>::Data as
                                        ::oop_rs::__private::data::ClassData>::CLASS_TYPE).expect("not a subclass of `Cat`");
                }
        }
    }
    pub type Cat = dyn ICat;
    type __Class = Cat;
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete +
        ::oop_rs::class::ClassConcrete + IAnimal + ::oop_rs::rc::RcRef> ICat
        for __Self {
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassHasDyn for Cat {
        type Dyn = Cat;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::HasData for Cat {
        type Data = __DCat;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::DynHasVtable for Cat {
        type Vtable<__Self: ::oop_rs::class::ClassConcrete> = __VCat<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassConcreteOrDyn for Cat {
        const TYPE: &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo
            =
            &(::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo::new_dyn(__TYPE_NAME).with_super::<Self>());
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassDyn for Cat {
        type Class<__Self: ::oop_rs::class::ClassConcrete> = __SCat<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::class::HasSuper for Cat {
        type Super = Animal;
    }
    #[automatically_derived]
    impl ::core::ops::Deref for Cat {
        type Target = Animal;
        fn deref(&self) -> &Self::Target { self }
    }
    #[repr(C)]
    pub struct __VCat<__Self: ::oop_rs::class::ClassConcrete> {
        __super: <__SCat<__Self> as
        ::oop_rs::__private::vtable::HasSuperVtable>::SuperVtable,
        __self: ::core::marker::PhantomData<__Self>,
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::clone::Clone for
        __VCat<__Self> {
        fn clone(&self) -> Self { *self }
    }
    #[automatically_derived]
    impl<__Self: ::oop_rs::class::ClassConcrete> ::core::marker::Copy for
        __VCat<__Self> {
    }
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::Vtable for __VCat<__Self> {
        const TYPE: ::oop_rs::__private::vtable::VtableTypeInfo<'static> =
            ::oop_rs::__private::vtable::VtableTypeInfo::Class(&(::oop_rs::__private::vtable::ClassVtableTypeInfo::new(__TYPE_NAME).with_super(<::oop_rs::prelude::CVtable<Animal,
                                __Self> as ::oop_rs::__private::vtable::Vtable>::TYPE)));
    }
    impl<__Self: ::oop_rs::class::ClassConcrete>
        ::oop_rs::__private::vtable::ClassVtable for __VCat<__Self> {}
    type __Self = __CCat;
    #[repr(transparent)]
    pub struct __CCat(__SCat<__Self>);
    #[automatically_derived]
    impl ::core::ops::Deref for __CCat {
        type Target = Cat;
        fn deref(&self) -> &Self::Target { self }
    }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassHasDyn for __CCat {
        type Dyn = Cat;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::HasData for __CCat {
        type Data = __DCat;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::data::ClassHasData for __CCat { }
    #[automatically_derived]
    impl ::oop_rs::__private::dynamic::ClassConcreteOrDyn for __CCat {
        const TYPE: &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo
            =
            &::oop_rs::__private::dynamic::ClassConcreteOrDynTypeInfo::new_concrete(__TYPE_NAME).with_super::<Self>();
    }
    #[automatically_derived]
    impl ::oop_rs::class::ClassConcrete for __CCat { }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::HasVtable for __CCat {
        type Vtable = __VCat<__Self>;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::HasVtableImpl for __CCat {
        const VTABLE: Self::Vtable = __SCat::<__Self>::__VTABLE;
    }
    #[automatically_derived]
    impl ::oop_rs::__private::vtable::ClassHasVtable for __CCat { }
    #[automatically_derived]
    impl ::oop_rs::class::HasSuper for __CCat {
        type Super = Animal;
    }
    #[automatically_derived]
    unsafe impl ::oop_rs::rc::RcRef for __CCat { }
    impl Cat {
        pub fn new() -> ::oop_rs::rc::CRc<__Self> {
            __SCat::new(::oop_rs::rc::CRcUninit::new().cast_uninit())
        }
    }
    impl __CCat {}
    impl Cat {}
    impl Cat {
        fn __data(&self) -> &__DCat {
            let offset =
                const {
                        <__DCat as
                                ::oop_rs::__private::data::ClassData>::CLASS_TYPE.offset()
                    };
            unsafe { &*::core::ptr::from_ref(self).byte_add(offset).cast() }
        }
    }
    #[repr(transparent)]
    pub struct __ACat<__M: ::oop_rs::__private::accessor::AccessMode>(::core::marker::PhantomData<__M>,
        Cat);
    #[automatically_derived]
    impl<__M: ::oop_rs::__private::accessor::AccessMode> ::core::ops::Deref
        for __ACat<__M> {
        type Target =
            <Animal as ::oop_rs::__private::accessor::Access>::Accessor<__M>;
        fn deref(&self) -> &Self::Target {
            <Animal as
                    ::oop_rs::__private::accessor::Access>::__access::<__M>(&self.1)
        }
    }
    #[automatically_derived]
    unsafe impl ::oop_rs::__private::accessor::Access for Cat {
        type Accessor<__M: ::oop_rs::__private::accessor::AccessMode> =
            __ACat<__M>;
    }
    impl __ACat<::oop_rs::__private::accessor::Get> {}
    impl __ACat<::oop_rs::__private::accessor::GetMut> {}
    impl __ACat<::oop_rs::__private::accessor::Set> {}
    impl __ACat<::oop_rs::__private::accessor::Update> {}
    impl __ACat<::oop_rs::__private::accessor::Replace> {}
    impl __ACat<::oop_rs::__private::accessor::ReplaceWith> {}
    impl __ACat<::oop_rs::__private::accessor::Raw> {}
}
fn must_succeed_downcast(animal: CRc<Animal>) -> bool {
    animal.downcast_rc::<Dog>().is_ok()
}
fn must_fail_downcast(animal: CRc<Animal>) -> bool {
    animal.downcast_rc::<Cat>().is_ok()
}
fn main() {
    let animal: CRc<Animal> = Dog::new();
    if !must_succeed_downcast(animal.clone()) {
        ::core::panicking::panic("assertion failed: must_succeed_downcast(animal.clone())")
    };
    if !!must_fail_downcast(animal) {
        ::core::panicking::panic("assertion failed: !must_fail_downcast(animal)")
    };
}
