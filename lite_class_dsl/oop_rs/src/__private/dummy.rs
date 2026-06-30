use crate::{
    __private::{
        data::{
            ClassData, ClassDataTypeInfo, ClassHasData, ClassOrMixinData, DataTypeInfo, HasData,
        },
        dynamic::{ClassConcreteOrDyn, ClassConcreteOrDynTypeInfo, ClassDyn, ClassHasDyn},
        vtable::{
            ClassHasVtable, ClassVtable, ClassVtableTypeInfo, HasVtable, HasVtableImpl, Vtable,
            VtableTypeInfo,
        },
    },
    class::ClassConcrete,
};

const TYPE_NAME: &str = "__DUMMY";

pub struct Dummy;

impl ClassConcreteOrDyn for Dummy {
    const TYPE: &ClassConcreteOrDynTypeInfo = &ClassConcreteOrDynTypeInfo::new_concrete(TYPE_NAME);
}
impl ClassConcrete for Dummy {}
impl ClassDyn for Dummy {
    type Class<__Self: ClassConcrete> = Dummy;
}
impl ClassHasDyn for Dummy {
    type Dyn = Dummy;
}
impl HasVtable for Dummy {
    type Vtable = DummyVtable;
}
impl ClassHasVtable for Dummy {}
impl HasVtableImpl for Dummy {
    const VTABLE: Self::Vtable = DummyVtable;
}
impl HasData for Dummy {
    type Data = Dummy;
}
impl ClassHasData for Dummy {}
impl ClassOrMixinData for Dummy {
    const TYPE: DataTypeInfo<'static> = DataTypeInfo::Class(&ClassDataTypeInfo::new(TYPE_NAME));
}
impl ClassData for Dummy {}
impl ClassVtable for DummyVtable {}
impl Vtable for DummyVtable {
    const TYPE: VtableTypeInfo<'static> =
        VtableTypeInfo::Class(&ClassVtableTypeInfo::new(TYPE_NAME));
}

#[derive(Default, Copy, Clone)]
pub struct DummyVtable;
