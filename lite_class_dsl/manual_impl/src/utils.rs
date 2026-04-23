use std::cell::RefCell;

thread_local! {
    pub static BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        $crate::utils::BUF.with_borrow_mut(|buf| {
            buf.push(format!($($args)*));
        })
    };
}
