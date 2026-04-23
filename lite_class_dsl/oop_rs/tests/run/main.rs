use std::cell::RefCell;

thread_local! {
    pub static BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        $crate::BUF.with_borrow_mut(|buf| {
            buf.push(format!($($args)*));
        })
    };
}

mod combination;
mod ctor;
mod downcast_ty;
mod eq_hash;
mod field_access;
mod format;
mod function_helper;
mod gallery_page;
mod get_set;
mod inheritance;
mod interface;
mod mixin;
mod single;
mod static_items;
