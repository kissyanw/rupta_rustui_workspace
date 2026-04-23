macro_rules! def {
    ($name:ident = $($tokens:tt)*) => {
        #[allow(non_camel_case_types)]
        pub(super) struct $name;
        impl quote::ToTokens for $name {
            fn to_tokens(&self, mut tokens: &mut proc_macro2::TokenStream) {
                quote::quote_each_token!(tokens $($tokens)*);
            }
        }
    };
}

def!(_TypeName = __TYPE_NAME);

def!(_core = ::core);
def!(_Cell = #_core::cell::Cell);
def!(_Ref = #_core::cell::Ref);
def!(_RefMut = #_core::cell::RefMut);
def!(_RefCell = #_core::cell::RefCell);
def!(_OnceCell = #_core::cell::OnceCell);
def!(_Option = #_core::option::Option);
def!(_Some = #_Option::Some);
def!(_None = #_Option::None);
def!(_PhantomData = #_core::marker::PhantomData);
def!(_FnOnce = #_core::ops::FnOnce);
def!(_Deref = #_core::ops::Deref);
def!(_Into = #_core::convert::Into);
def!(_Borrow = #_core::borrow::Borrow);
def!(_TypeId = #_core::any::TypeId);
def!(_offset_of = #_core::mem::offset_of);

def!(_crate = ::oop_rs);
def!(_CRc = #_crate::rc::CRc);
def!(_CWeak = #_crate::rc::CWeak);
def!(_RcLike = #_crate::rc::RcLike);
def!(_CRcUninit = #_crate::rc::CRcUninit);
def!(_Missing = #_crate::missing::Missing);
def!(_Dummy = #_crate::__private::Dummy);
def!(_HasData = #_crate::__private::data::HasData);
def!(_ClassData = #_crate::__private::data::ClassData);
def!(_ClassHasData = #_crate::__private::data::ClassHasData);
def!(_ClassOrMixinData = #_crate::__private::data::ClassOrMixinData);
def!(_DataTypeInfo = #_crate::__private::data::DataTypeInfo);
def!(_ClassDataTypeInfo = #_crate::__private::data::ClassDataTypeInfo);
def!(_HasVtable = #_crate::__private::vtable::HasVtable);
def!(_DynHasVtable = #_crate::__private::vtable::DynHasVtable);
def!(_Vtable = #_crate::__private::vtable::Vtable);
def!(_VtableTypeInfo = #_crate::__private::vtable::VtableTypeInfo);
def!(_HasVtableImpl = #_crate::__private::vtable::HasVtableImpl);
def!(_ClassHasVtable = #_crate::__private::vtable::ClassHasVtable);
def!(_MixinHasVtable = #_crate::__private::vtable::MixinHasVtable);
def!(_MixinVtable = #_crate::__private::vtable::MixinVtable);
def!(_MixinInstanceVtable = #_crate::__private::vtable::MixinInstanceVtable);
def!(_MixinVtableTypeInfo = #_crate::__private::vtable::MixinVtableTypeInfo);
def!(_MixinInstanceVtableTypeInfo = #_crate::__private::vtable::MixinInstanceVtableTypeInfo);
def!(_Type = #_crate::__private::vtable::Type);
def!(_HasSuperVtable = #_crate::__private::vtable::HasSuperVtable);
def!(_HasSuperVtableImpl = #_crate::__private::vtable::HasSuperVtableImpl);
def!(_Class = #_crate::class::Class);
def!(_ClassConcrete = #_crate::class::ClassConcrete);
def!(_ClassConcreteOrDyn = #_crate::__private::dynamic::ClassConcreteOrDyn);
def!(_ClassConcreteOrDynTypeInfo = #_crate::__private::dynamic::ClassConcreteOrDynTypeInfo);
def!(_ClassVtableTypeInfo = #_crate::__private::vtable::ClassVtableTypeInfo);
def!(_VtableOffsetEntry = #_crate::__private::vtable::VtableOffsetEntry);
def!(_ClassVtable = #_crate::__private::vtable::ClassVtable);
def!(_ClassVtableImpl = #_crate::__private::vtable::ClassVtableImpl);
def!(_ClassDyn = #_crate::__private::dynamic::ClassDyn);
def!(_HasSuper = #_crate::class::HasSuper);
def!(_ErasedPtr = #_crate::__private::downcast::ErasedPtr);

def!(_ClassHasDyn = #_crate::__private::dynamic::ClassHasDyn);
def!(_MixinHasData = #_crate::__private::data::MixinHasData);
def!(_DataOffsetEntry = #_crate::__private::data::DataOffsetEntry);
def!(_MixinData = #_crate::__private::data::MixinData);
def!(_MixinDataOffset = #_crate::__private::data::MixinDataOffset);
def!(_MixinDataTypeInfo = #_crate::__private::data::MixinDataTypeInfo);
def!(_MixinDyn = #_crate::__private::dynamic::MixinDyn);
def!(_MixinHasDyn = #_crate::__private::dynamic::MixinHasDyn);
def!(_CClass = #_crate::prelude::CClass);
def!(_MClass = #_crate::prelude::MClass);
def!(_CData = #_crate::prelude::CData);
def!(_CVtable = #_crate::prelude::CVtable);

def!(_Access = #_crate::__private::accessor::Access);
def!(_AccessMode = #_crate::__private::accessor::AccessMode);
def!(_AccessorUpcast = #_crate::__private::accessor::AccessorUpcast);
def!(_MixinAccessor = #_crate::__private::accessor::MixinAccessor);
def!(_FieldCopy = #_crate::__private::field::FieldCopy);
def!(_FieldRc = #_crate::__private::field::FieldRc);
def!(_FieldRef = #_crate::__private::field::FieldRef);
def!(_FieldMutCopy = #_crate::__private::field::FieldMutCopy);
def!(_FieldMutRc = #_crate::__private::field::FieldMutRc);
def!(_FieldRefMut = #_crate::__private::field::FieldRefMut);
def!(_FieldLateCopy = #_crate::__private::field::FieldLateCopy);
def!(_FieldLateRc = #_crate::__private::field::FieldLateRc);
def!(_FieldLateRef = #_crate::__private::field::FieldLateRef);
def!(_FieldLateMutCopy = #_crate::__private::field::FieldLateMutCopy);
def!(_FieldLateMutRc = #_crate::__private::field::FieldLateMutRc);
def!(_FieldLateRefMut = #_crate::__private::field::FieldLateRefMut);

def!(_Get = #_crate::__private::accessor::Get);
def!(_GetMut = #_crate::__private::accessor::GetMut);
def!(_Set = #_crate::__private::accessor::Set);
def!(_Update = #_crate::__private::accessor::Update);
def!(_Replace = #_crate::__private::accessor::Replace);
def!(_ReplaceWith = #_crate::__private::accessor::ReplaceWith);
def!(_Raw = #_crate::__private::accessor::Raw);

def!(_RcRef = #_crate::rc::RcRef);
def!(_IDowncast = #_crate::__private::downcast::IDowncast);
def!(_Downcast = #_crate::__private::downcast::Downcast);
def!(_IEqHash = #_crate::object::IEqHash);
def!(_EqHash = #_crate::object::EqHash);
def!(_IFormat = #_crate::object::IFormat);
def!(_Format = #_crate::object::Format);

def!(_write_vtable = #_crate::write_vtable);
def!(_read_vtable = #_crate::read_vtable);
def!(_assert_subclass = #_crate::assert_subclass);

def!(_inline = #[inline]);
def!(_track_caller = #[track_caller]);
