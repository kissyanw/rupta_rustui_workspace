use quote::{ToTokens, quote};

use super::class::Class;
use super::method::{Ctor, ImplItemFn, Method};

// Helper functions for roundtrip testing

#[track_caller]
fn test_class_roundtrip_impl(input_tokens: proc_macro2::TokenStream) {
    let parsed1 = syn::parse2::<Class>(input_tokens.clone()).expect("First parse failed");
    let tokens = parsed1.to_token_stream();
    let parsed2 = syn::parse2::<Class>(tokens.clone()).expect("Second parse failed");
    let tokens2 = parsed2.to_token_stream();
    assert_eq!(
        tokens.to_string(),
        tokens2.to_string(),
        "Token streams do not match after round-trip"
    );
}

#[track_caller]
fn test_field_roundtrip_impl(input_tokens: proc_macro2::TokenStream) {
    let class_tokens = quote! {
        type Test = class<{ #input_tokens }>;
    };
    let parsed1 = syn::parse2::<Class>(class_tokens).expect("First parse failed");
    assert_eq!(parsed1.items.fields.len(), 1, "Expected exactly one field");
    let field1 = &parsed1.items.fields[0];
    let tokens = field1.to_token_stream();
    let class_tokens2 = quote! {
        type Test = class<{ #tokens }>;
    };
    let parsed2 = syn::parse2::<Class>(class_tokens2).expect("Second parse failed");
    assert_eq!(
        parsed2.items.fields.len(),
        1,
        "Expected exactly one field in second parse"
    );
    let field2 = &parsed2.items.fields[0];
    let tokens2 = field2.to_token_stream();
    assert_eq!(
        tokens.to_string(),
        tokens2.to_string(),
        "Token streams do not match after round-trip"
    );
}

#[track_caller]
fn test_ctor_roundtrip_impl(input_tokens: proc_macro2::TokenStream) {
    let item_fn1 = syn::parse2::<ImplItemFn>(input_tokens).expect("First parse failed");
    let ctor1 = Ctor::try_from(item_fn1).expect("First conversion to Ctor failed");
    let tokens = ctor1.to_token_stream();
    let item_fn2 = syn::parse2::<ImplItemFn>(tokens.clone()).expect("Second parse failed");
    let ctor2 = Ctor::try_from(item_fn2).expect("Second conversion to Ctor failed");
    let tokens2 = ctor2.to_token_stream();
    assert_eq!(
        tokens.to_string(),
        tokens2.to_string(),
        "Token streams do not match after round-trip"
    );
}

#[track_caller]
fn test_method_roundtrip_impl(input_tokens: proc_macro2::TokenStream) {
    let item_fn1 = syn::parse2::<ImplItemFn>(input_tokens).expect("First parse failed");
    let mut method1 = Method::try_from(item_fn1).expect("First conversion to Method failed");
    let modifiers1 = method1
        .extract_method_modifiers()
        .expect("Failed to extract method modifiers");
    method1.tk_final = modifiers1.tk_final;
    let tokens = method1.to_token_stream();
    let item_fn2 = syn::parse2::<ImplItemFn>(tokens.clone()).expect("Second parse failed");
    let mut method2 = Method::try_from(item_fn2).expect("Second conversion to Method failed");
    let modifiers2 = method2
        .extract_method_modifiers()
        .expect("Failed to extract method modifiers in second parse");
    method2.tk_final = modifiers2.tk_final;
    let tokens2 = method2.to_token_stream();
    assert_eq!(
        tokens.to_string(),
        tokens2.to_string(),
        "Token streams do not match after round-trip"
    );
}

// Macros for convenient test syntax

macro_rules! test_field {
    ($name:ident { $($field:tt)+ }) => {
        #[test]
        fn $name() {
            test_field_roundtrip_impl(quote! { $( $field )* });
        }
    };
}

macro_rules! test_ctor {
    ($name:ident { $($tokens:tt)* }) => {
        #[test]
        fn $name() {
            test_ctor_roundtrip_impl(quote! { $($tokens)* });
        }
    };
}

macro_rules! test_method {
    ($name:ident { $($tokens:tt)* }) => {
        #[test]
        fn $name() {
            test_method_roundtrip_impl(quote! { $($tokens)* });
        }
    };
}

macro_rules! test_class {
    ($name:ident { $($tokens:tt)* }) => {
        #[test]
        fn $name() {
            test_class_roundtrip_impl(quote! { $($tokens)* });
        }
    };
}

// Field tests

test_field!(test_simple_field { let x: i32; });
test_field!(test_field_with_init { let x: i32 = 42; });
test_field!(test_field_with_mut { let mut x: String; });
test_field!(test_field_with_late { #[late] let x: i32; });
test_field!(test_field_with_late_mut { #[late] let mut x: i32; });
test_field!(test_field_with_doc { #[doc = "A field"] let x: i32; });

// Constructor tests

test_ctor!(test_simple_constructor {
    fn new() -> Self {
        Self {}
    }
});

test_ctor!(test_constructor_with_params {
    fn new(x: i32, y: String) -> Self {
        Self { x, y }
    }
});

test_ctor!(test_constructor_with_init {
    fn new(value: i32) -> Self {
        let computed = value * 2;
        Self { x: computed }
    }
});

test_ctor!(test_constructor_with_super_call {
    fn new(x: i32, y: i32) -> Self {
        Self { y, ..Super::new(x) }
    }
});

test_ctor!(test_constructor_with_self_stmt {
    fn new(x: i32) -> Self {
        let self = Self { x };
        println!("Created with x = {}", self.x);
        self
    }
});

test_ctor!(test_constructor_with_mut_self_stmt {
    fn new(x: i32) -> Self {
        let mut self = Self { x };
        println!("Created with x = {}", self.x);
        self.x += 1;
        self
    }
});

test_ctor!(test_constructor_with_complex_super {
    fn new(x: i32, y: i32, z: i32) -> Self {
        let sum = x + y;
        Self { z, ..Super::new(sum) }
    }
});

test_ctor!(test_constructor_with_complex_self_stmt {
    fn new(x: i32) -> Self {
        let doubled = x * 2;
        let self = Self { x: doubled };
        self.validate();
        self.initialize();
        self
    }
});

test_ctor!(test_constructor_with_complex_self_stmt_and_super {
    fn new(x: i32) -> Self {
        let doubled = x * 2;
        let self = Self { x: doubled, ..Super::construct(doubled) };
        self.validate();
        self.initialize();
        self
    }
});

// Method tests

test_method!(test_simple_method {
    fn get_x(&self) -> i32 {
        self.x
    }
});

test_method!(test_mutable_method {
    fn set_x(&mut self, value: i32) {
        self.x = value;
    }
});

test_method!(test_method_with_generics {
    fn map<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&i32) -> R,
    {
        f(&self.x)
    }
});

test_method!(test_async_method {
    async fn fetch(&self) -> Result<String, Error> {
        Ok(String::new())
    }
});

test_method!(test_const_method {
    const fn value(&self) -> i32 {
        42
    }
});

test_method!(test_unsafe_method {
    unsafe fn raw_ptr(&self) -> *const i32 {
        &self.x as *const i32
    }
});

test_method!(test_final_method {
    #[method(final)]
    fn final_method(&self) -> i32 {
        42
    }
});

test_method!(test_method_with_override {
    #[method(override(Base))]
    fn method(&self) -> i32 {
        42
    }
});

#[test]
fn test_method_with_override_and_final() {
    // Combining `override` and `final` is invalid; extract_method_modifiers must return Err.
    let input_tokens = quote! {
        #[method(override(Base), final)]
        fn method(&self) -> i32 {
            42
        }
    };
    let item_fn = syn::parse2::<ImplItemFn>(input_tokens).expect("Parse failed");
    let mut method = Method::try_from(item_fn).expect("Conversion to Method failed");
    let result = method.extract_method_modifiers();
    assert!(
        result.is_err(),
        "Expected an error for combined override+final, but got Ok"
    );
}

test_method!(test_method_with_self_by_value {
    fn consume(self) -> i32 {
        self.x
    }
});

test_method!(test_method_with_pinned_self {
    fn method(self: Pin<&mut Self>) {
        // implementation
    }
});

// Class tests

test_class!(test_simple_class {
    type Point = class<{
        let x: i32;
        let y: i32;

        fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        fn distance(&self) -> f64 {
            ((self.x * self.x + self.y * self.y) as f64).sqrt()
        }
    }>;
});

test_class!(test_abstract_class {
    #[class(abstract)]
    type Shape = class<{
        fn area(&self) -> f64 {
            unimplemented!()
        }
    }>;
});

test_class!(test_final_class {
    #[class(final)]
    type Circle = class<{
        let radius: f64;

        fn new(radius: f64) -> Self {
            Self { radius }
        }
    }>;
});

test_class!(test_class_with_generics {
    type Container<T> = class<{
        let value: T;

        fn new(value: T) -> Self {
            Self { value }
        }

        fn get(&self) -> &T {
            &self.value
        }
    }>;
});

test_class!(test_class_with_visibility {
    pub type Counter = class<{
        let count: i32 = 0;

        pub fn new() -> Self {
            Self { count: 0 }
        }

        pub fn increment(&mut self) {
            self.count += 1;
        }

        pub fn get(&self) -> i32 {
            self.count
        }
    }>;
});

test_class!(test_mixin {
    type Printable = mixin<{
        fn print(&self) {
            println!("Printing...");
        }
    }>;
});

test_class!(test_class_with_consts {
    type Config = class<{
        const MAX_SIZE: usize = 100;

        let size: usize;

        fn new() -> Self {
            Self { size: 0 }
        }
    }>;
});

test_class!(test_class_with_functions {
    type Utils = class<{
        fn helper() -> i32 {
            42
        }

        fn process(x: i32) -> i32 {
            x * 2
        }
    }>;
});

test_class!(test_class_with_attributes {
    #[derive(Debug)]
    type Node = class<{
        #[doc = "The node value"]
        let value: i32;

        #[inline]
        fn new(value: i32) -> Self {
            Self { value }
        }
    }>;
});

test_class!(test_complex_class {
    pub type ComplexType<T, U> = class<{
        let field1: T;
        let mut field2: U;
        #[late]
        let field3: String;

        const DEFAULT_SIZE: usize = 10;

        pub fn new(field1: T, field2: U) -> Self {
            Self { field1, field2 }
        }

        pub fn get_field1(&self) -> &T {
            &self.field1
        }

        pub fn set_field2(&mut self, value: U) {
            self.field2 = value;
        }

        #[method(final)]
        pub fn final_method(&self) -> i32 {
            42
        }

        pub async fn async_method(&self) -> Result<(), Error> {
            Ok(())
        }

        fn helper_function() -> i32 {
            Self::DEFAULT_SIZE as i32
        }
    }>;
});

test_class!(test_class_with_all_features {
    #[derive(Debug, Clone)]
    pub type FullFeatures<T: Clone> = class<{
        #[doc = "A field"]
        let immutable: T;

        let mut mutable: i32 = 0;

        #[late]
        let late_field: String;

        const CONSTANT: usize = 100;

        pub fn new(immutable: T) -> Self {
            Self { immutable, mutable: 0 }
        }

        pub fn with_value(immutable: T, value: i32) -> Self {
            let self = Self { immutable, mutable: value };
            println!("Created with value: {}", self.mutable);
            self
        }

        pub fn get(&self) -> &T {
            &self.immutable
        }

        pub fn set(&mut self, value: i32) {
            self.mutable = value;
        }

        #[method(final)]
        pub fn final_method(&self) -> i32 {
            Self::CONSTANT as i32
        }

        pub async fn async_operation(&mut self) -> Result<(), Error> {
            self.mutable += 1;
            Ok(())
        }

        pub fn static_method() -> i32 {
            42
        }

        fn private_helper(&self) -> i32 {
            self.mutable * 2
        }
    }>;
});
