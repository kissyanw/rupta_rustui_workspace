use core::cmp::Ordering;

pub mod accessor;
pub mod data;
pub mod downcast;
mod dummy;
pub mod dynamic;
pub mod field;
pub mod generic;
pub mod unsize;
pub mod vtable;

pub use dummy::Dummy;

const fn strcmp(a: &str, b: &str) -> Ordering {
    let mut a = a.as_bytes();
    let mut b = b.as_bytes();
    loop {
        return match (a, b) {
            ([], []) => Ordering::Equal,
            ([], _) => Ordering::Less,
            (_, []) => Ordering::Greater,
            (&[a, ..], &[b, ..]) if a < b => Ordering::Less,
            (&[a, ..], &[b, ..]) if a > b => Ordering::Greater,
            ([_, a_rest @ ..], [_, b_rest @ ..]) => {
                a = a_rest;
                b = b_rest;
                continue;
            }
        };
    }
}

const fn streq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    matches!(strcmp(a, b), Ordering::Equal)
}
