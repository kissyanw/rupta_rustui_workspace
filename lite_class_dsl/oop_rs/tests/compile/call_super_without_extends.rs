use oop_rs::prelude::*;

#[class]
pub type ChangeNotifier = mixin<
    {
        pub fn dispose(&self) {}
    },
>;

#[class(with(ChangeNotifier))]
pub type RestorationManager = class<
    {
        #[method(override(ChangeNotifier))]
        pub fn dispose(&self) {
            super.dispose();
        }
    },
>;
