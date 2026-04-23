use thiserror::Error;

/// Parsing errors for class syntax
#[derive(Debug, Error)]
pub enum ParseError {
    // Constructor errors
    #[error("expected constructor body")]
    ExpectedCtorBody,

    #[error("constructor body must end with `Self {{ ... }}` construction")]
    CtorBodyMissingConstruction,

    #[error("unexpected `let ref self` pattern")]
    UnexpectedByRefPattern,

    #[error("unexpected `let self @ ..` pattern")]
    UnexpectedSubpatPattern,

    #[error("expected `Self {{ ... }}` construction")]
    ExpectedSelfConstruction,

    #[error("expected `Super::method(...)` call")]
    ExpectedSuperCall,

    #[error("expected `let self = Self {{ ... }}; ...; self` statements")]
    ExpectedLetSelfPattern,

    #[error("expected named field")]
    ExpectedNamedField,

    #[error("expected 'self' binding")]
    ExpectedSelfBinding,

    #[error("expected initializer")]
    ExpectedInitializer,

    #[error("expected return `self` statement")]
    ExpectedSelfReturn,

    // Method errors
    #[error("`override` and `final` cannot be used together")]
    FinalAndOverrideConflict,

    #[error("method must have `self` receiver as first parameter")]
    MethodMustHaveSelfReceiver,

    #[error("method must have `self` receiver")]
    MethodMissingSelfReceiver,

    #[error("method can only have one `self` receiver")]
    MethodMultipleSelfReceivers,

    // Function errors
    #[error("expected function body")]
    ExpectedFnBody,

    // Class parsing errors
    #[error("expected field, constructor, method, function, or const")]
    ExpectedClassItem,
}

/// Errors during checking
#[derive(Debug, Error)]
pub enum CheckError {
    #[error("expected `on` attribute in class")]
    UnexpectedClassAttrOn,
    #[error("unexpected `extends` attribute in mixin")]
    UnexpectedMixinAttrExtends,
    #[error("unexpected `with` attribute in mixin")]
    UnexpectedMixinAttrWith,
    #[error("unexpected `abstract` attribute in mixin")]
    UnexpectedMixinAttrAbstract,
    #[error("unexpected `final` attribute in mixin")]
    UnexpectedMixinAttrFinal,
    #[error("unexpected `extends` attribute in interface")]
    UnexpectedInterfaceAttrExtends,
    #[error("unexpected `abstract` attribute in interface")]
    UnexpectedInterfaceAttrAbstract,
    #[error("unexpected `final` attribute in interface")]
    UnexpectedInterfaceAttrFinal,
    #[error(
        "`implements(Downcast)` cannot be used with `extends`; consider adding `extends(Object)` to its super class"
    )]
    ExtendsAndImplsDowncast,
    #[error(
        "`implements(Downcast)` cannot be used with `on`; consider adding `on(Object)` to the mixin, or add `extends(Object)` to its super class"
    )]
    #[expect(dead_code)]
    OnAndImplsDowncast,
    #[error("`Downcast` must be the first interface in the `implements` list")]
    ImplDowncastMustBeFirst,
}

/// Errors during macro expansion
#[derive(Debug, Error)]
pub enum ExpandError {
    #[error("super call in non-overridden method")]
    SuperCallInNonOverriddenMethod,
}
