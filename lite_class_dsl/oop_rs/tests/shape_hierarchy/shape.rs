// Shape abstract base class module

use oop_rs::prelude::*;

#[class(abstract, extends(Object))]
pub type Shape = class<
    {
        #[vis(pub)]
        let ref color: Option<String>;

        pub fn new(color: String) -> Self {
            Self {
                color: Some(color),
            }
        }

        pub fn area(&self) -> f64;
        pub fn perimeter(&self) -> f64;
        pub fn description(&self) -> String;
    },
>;
