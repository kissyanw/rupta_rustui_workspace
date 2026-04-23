use oop_rs::prelude::*;

#[class(implements(Format))]
type DefaultFormat = class<
    {
        pub fn new() -> Self {
            Self {}
        }
    },
>;

#[class(implements(Format))]
type CustomFormat = class<
    {
        let x: i32 = 5;
        pub fn new() -> Self {
            Self {}
        }

        #[method(override(Format))]
        fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "CustomFormat(x = {})", self.get().x())
        }
    },
>;

#[class(extends(Object))]
type CustomObject = class<
    {
        let x: i32 = 8;
        pub fn new() -> Self {
            Self {}
        }

        #[method(override(Format))]
        fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "CustomObject(x = {})", self.get().x())
        }
    },
>;

#[class(implements(Format))]
type CustomInterface = interface<
    {
        #[method(override(Format))]
        fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "CustomInterface")
        }
    },
>;

#[class(implements(CustomInterface))]
type CustomInterfaceExt = class<
    {
        pub fn new() -> Self {
            Self {}
        }
    },
>;

#[class(implements(Format))]
type CustomMixin = mixin<
    {
        #[method(override(Format))]
        fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "CustomMixin")
        }
    },
>;

#[class(with(CustomMixin))]
type CustomMixinExt = class<
    {
        pub fn new() -> Self {
            Self {}
        }
    },
>;

#[test]
fn test_format() {
    let default_format = DefaultFormat::new();
    let custom_format = CustomFormat::new();
    let custom_object = CustomObject::new();
    let custom_interface: CRc<CustomInterface> = CustomInterfaceExt::new();
    let custom_mixin: CRc<CustomMixin> = CustomMixinExt::new();
    assert_eq!(
        format!("{:?}", default_format),
        format!("{:p}", default_format)
    );
    assert_eq!(format!("{:?}", custom_format), "CustomFormat(x = 5)");
    assert_eq!(
        format!("{:?}", &*custom_format as &Format),
        "CustomFormat(x = 5)"
    );
    assert_eq!(format!("{:?}", custom_object), "CustomObject(x = 8)");
    assert_eq!(
        format!("{:?}", &*custom_object as &Object),
        "CustomObject(x = 8)"
    );
    assert_eq!(format!("{:?}", custom_interface), "CustomInterface");
    assert_eq!(format!("{:?}", custom_mixin), "CustomMixin");
}
