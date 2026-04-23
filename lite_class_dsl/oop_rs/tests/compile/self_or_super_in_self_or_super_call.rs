use oop_rs::prelude::*;

#[class]
type Base = class<
    {
        fn foo(&self) {
            self.bar(self.baz());
            println!("{}", self.bar(self.baz()))
        }

        fn bar(&self, x: i32) -> i32 {
            x
        }

        fn baz(&self) -> i32 {
            0
        }
    },
>;

#[class(extends(Base))]
type Derived = class<
    {
        #[method(override(Base))]
        fn foo(&self) {
            super.bar(self.baz());
            println!("{}", super.bar(self.baz()))
        }
    },
>;
