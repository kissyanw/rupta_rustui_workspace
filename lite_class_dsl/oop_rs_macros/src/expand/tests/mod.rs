use std::path::{Path, PathBuf};

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

use crate::syntax::Class;

trait PathBufExt {
    fn with(self, path: impl AsRef<Path>) -> Self;
}

impl PathBufExt for PathBuf {
    fn with(mut self, path: impl AsRef<Path>) -> Self {
        self.push(path);
        self
    }
}

struct Classes {
    classes: Vec<Class>,
}

impl Parse for Classes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut classes = Vec::new();
        while !input.is_empty() {
            classes.push(input.parse()?);
        }
        Ok(Classes { classes })
    }
}

impl Classes {
    fn extract_class_attrs(&mut self) {
        for class in &mut self.classes {
            class.extract_class_attrs();
        }
    }
    fn expand(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        for class in &self.classes {
            tokens.extend(class.expand());
        }
        tokens
    }
}

fn test_impl(test: &str) {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .with("src")
        .with("expand")
        .with("tests")
        .with(test);

    let input = std::fs::read_to_string(test_dir.join("input.rs")).expect("Failed to read file");

    let mut parsed: Classes = syn::parse_str(&input).expect("Failed to parse");
    parsed.extract_class_attrs();
    let expanded = parsed.expand();
    let expanded_rs = std::env::temp_dir().with(format!("{test}.rs"));
    std::fs::write(&expanded_rs, format!("{expanded:#}")).expect("Failed to write file");
    std::process::Command::new("rustfmt")
        .arg(&expanded_rs)
        .spawn()
        .expect("Failed to run `rustfmt`")
        .wait()
        .expect("Failed to format file");
    let expanded_txt = std::fs::read_to_string(&expanded_rs).expect("Failed to read file");

    let expected_rs = test_dir.join("expand.rs");
    if std::env::var("EXPAND_TEST_OVERWRITE")
        .map(|mut s| {
            s.make_ascii_lowercase();
            s
        })
        .is_ok_and(|s| matches!(s.as_str(), "1" | "true" | "yes"))
    {
        std::fs::write(&expected_rs, expanded_txt).expect("Failed to write file");
    } else {
        let expected_txt = std::fs::read_to_string(&expected_rs).expect("Failed to read file");
        pretty_assertions::assert_eq!(
            expanded_txt,
            expected_txt,
            "test failed, consider rerun with `EXPAND_TEST_OVERWRITE=1` to update the expected output"
        );
    }
}

macro_rules! tests {
    ( $($test:ident),*  $(,)?) => { $(
        #[test]
        fn $test() {
            test_impl(const { strip_test_prefix(stringify!($test)) });
        }
    )* };
}

const fn strip_test_prefix(test: &str) -> &str {
    match test.as_bytes() {
        [b't', b'e', b's', b't', b'_', rest @ ..] => unsafe { str::from_utf8_unchecked(rest) },
        _ => panic!("test case must start with `test_`"),
    }
}

tests!(
    test_element,
    test_animal,
    test_state,
    test_object,
    test_modifiers,
    test_change_notifier,
    test_mixin_withs,
    test_mixin_on_mixin,
    test_multi_impls,
    test_mixin_downcast,
    test_eq_hash,
    test_gesture_recognizer,
    test_override_no_impl,
    test_three_mixins,
);
