use oop_rs::prelude::*;
use std::hash::{BuildHasher, Hash};

fn hash<T: Hash>(t: &T) -> u64 {
    core::hash::BuildHasherDefault::<std::hash::DefaultHasher>::new().hash_one(t)
}

// test eqality in a table-like way
// a === b  ->  hash(a) == hash(b) && a == b
// a  == b  ->  hash(a) == hash(b)
// a !== b  ->  hash(a) != hash(b) && a != b
// a  != b  ->  hash(a) != hash(b)
macro_rules! eq_tests {
        (
            (() $($items:ident)*)
            $( ($head:ident $($eqs:tt)*) )*
        ) => {
            eq_tests! {
                (@ $($items)* )
                (() $($items)* )
                $( ( $head $($eqs)* ) )*
            }
        };
        (
            (@ $($saved_items:ident)* )
            (() $($items:ident)*)
        ) => {};
        (
            (@ $($items:ident)* )
            (() )
            ($cur:ident )
            $( $rows:tt )*
        ) => {
            eq_tests! {
                (@ $($items)* )
                (() $($items)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (==)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} == {}", stringify!($head), stringify!($cur));
            assert!(hash(&$head) == hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (!=)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} != {}", stringify!($head), stringify!($cur));
            assert!(hash(&$head) != hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (===)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} === {}", stringify!($head), stringify!($cur));
            assert!(&$head == &$cur);
            assert!(hash(&$head) == hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
        (
            (@           $($items:ident)* )
            (()          $cur:ident $($next:ident)* )
            ($head:ident (!==)       $($eqs:tt)* )
            $( $rows:tt )*
        ) => {
            // println!("{} !== {}", stringify!($head), stringify!($cur));
            assert!(&$head != &$cur);
            assert!(hash(&$head) != hash(&$cur));
            eq_tests! {
                (@ $($items)* )
                (() $($next)* )
                ($head $($eqs)* )
                $( $rows )*
            }
        };
    }

#[class(implements(EqHash))]
type Cls = class<
    {
        pub fn new() -> Self {
            Self {}
        }
    },
>;

#[class(implements(EqHash))]
type Interface = interface<{}>;

#[class(extends(Cls), implements(Interface))]
type MyClassExt = class<
    {
        pub fn new() -> Self {
            Self { ..Super::new() }
        }
    },
>;

#[class(extends(Object))]
type ClsWithData = class<
    {
        let x: u32;

        pub fn new(x: u32) -> Self {
            Self { x }
        }

        #[method(override(EqHash))]
        pub fn eq(&self, other: &EqHash) -> bool {
            other
                .downcast_ref::<ClsWithData>()
                .is_ok_and(|other| self.get().x() == other.get().x())
        }

        #[method(override(EqHash))]
        pub fn hash(&self, state: &mut (dyn core::hash::Hasher + '_)) {
            state.write_u32(self.get().x());
        }
    },
>;

#[class(implements(EqHash))]
type Mixin = mixin<
    {
        #[late]
        let x: u32;

        #[method(override(EqHash))]
        pub fn eq(&self, other: &EqHash) -> bool {
            other
                .downcast_ref::<Mixin>()
                .is_ok_and(|other| self.get().x() == other.get().x())
        }

        #[method(override(EqHash))]
        pub fn hash(&self, state: &mut (dyn core::hash::Hasher + '_)) {
            state.write_u32(self.get().x());
        }
    },
>;

#[class(with(Mixin))]
type MixinExt = class<
    {
        fn new(x: u32) -> Self {
            let self = Self {};
            self.set().x(x);
            self
        }
    },
>;

#[test]
fn test_eq_and_hash() {
    let c: CRc<EqHash> = Cls::new();
    // let cw = CRc::downgrade(&c);

    let ce: CRc<EqHash> = MyClassExt::new();
    // let cew = CRc::downgrade(&ce);

    let ces: CRc<EqHash> = ce.clone();
    // let cesw = CRc::downgrade(&ces);

    let cei: CRc<EqHash> = ce.clone();
    // let ceiw = CRc::downgrade(&cei);

    eq_tests! {
        (()   c     /* cw   */ ce    /* cew  */  ces  /* cesw */ cei   /* ceiw */)
        (c    (===) /* (==) */ (!==) /* (!=) */ (!==) /* (!=) */ (!==) /* (!=) */)
        // (cw   (===) /* (==) */ (!==) /* (!=) */ (!==) /* (!=) */ (!==) /* (!=) */)
        (ce   (!==) /* (!=) */ (===) /* (==) */ (===) /* (==) */ (===) /* (==) */)
        // (cew  (!==) /* (!=) */ (===) /* (==) */ (===) /* (==) */ (===) /* (==) */)
        (ces  (!==) /* (!=) */ (===) /* (==) */ (===) /* (==) */ (===) /* (==) */)
        // (cesw (!==) /* (!=) */ (===) /* (==) */ (===) /* (==) */ (===) /* (==) */)
        (cei  (!==) /* (!=) */ (===) /* (==) */ (===) /* (==) */ (===) /* (==) */)
        // (ceiw (!==) /* (!=) */ (===) /* (==) */ (===) /* (==) */ (===) /* (==) */)
    }

    let d1_1 = ClsWithData::new(1);
    let d1_2 = ClsWithData::new(1);
    let d1o1: CRc<Object> = ClsWithData::new(1);
    let d1o2: CRc<Object> = ClsWithData::new(1);
    let d2_1 = ClsWithData::new(2);
    let d2_2 = ClsWithData::new(2);
    let d2o1: CRc<Object> = ClsWithData::new(2);
    let d2o2: CRc<Object> = ClsWithData::new(2);

    eq_tests! {
        (()   d1_1  d1_2  d1o1  d1o2  d2_1   d2_2  d2o1 d2o2 )
        (d1_1 (===) (===) (== ) (== ) (!==) (!==) (!= ) (!= ))
        (d1_2 (===) (===) (== ) (== ) (!==) (!==) (!= ) (!= ))
        (d1o1 (== ) (== ) (===) (===) (!= ) (!= ) (!==) (!==))
        (d1o2 (== ) (== ) (===) (===) (!= ) (!= ) (!==) (!==))
        (d2_1 (!==) (!==) (!= ) (!= ) (===) (===) (== ) (== ))
        (d2_2 (!==) (!==) (!= ) (!= ) (===) (===) (== ) (== ))
        (d2o1 (!= ) (!= ) (!==) (!==) (== ) (== ) (===) (===))
        (d2o2 (!= ) (!= ) (!==) (!==) (== ) (== ) (===) (===))
    }

    let m1_1: CRc<Mixin> = MixinExt::new(1);
    let m1_2: CRc<Mixin> = MixinExt::new(1);
    let m2_1: CRc<Mixin> = MixinExt::new(2);
    let m2_2: CRc<Mixin> = MixinExt::new(2);

    eq_tests! {
        (()   m1_1  m1_2  m2_1  m2_2 )
        (m1_1 (===) (===) (!==) (!==) )
        (m1_2 (===) (===) (!==) (!==) )
        (m2_1 (!==) (!==) (===) (===) )
        (m2_2 (!==) (!==) (===) (===) )
    }
}
