use crate::BUF;
use oop_rs::prelude::*;

#[class]
type A = class<
    {
        let x: u8 = 0_u8;

        pub fn new() -> Self {
            Self { .. }
        }
        pub fn print_offsets(&self) {
            println!("offset!(A, x): {}", unsafe {
                core::ptr::from_ref(__self_dyn.raw().x())
                    .byte_offset_from(core::ptr::from_ref(__self_dyn))
            });
        }
    },
>;

#[class(on(A))]
type M = mixin<
    {
        let y: u16 = 1_u16;

        #[method(override(A))]
        pub fn print_offsets(&self) {
            super.print_offsets();
            println!("offset!(A/M, x): {}", unsafe {
                core::ptr::from_ref((__self as &A).raw().x())
                    .byte_offset_from(core::ptr::from_ref(__self))
            });
            println!("offset!(A/M, y): {}", unsafe {
                core::ptr::from_ref(__self_dyn.raw().y())
                    .byte_offset_from(core::ptr::from_ref(__self))
            });
            println!("offset!(M, x): {}", unsafe {
                core::ptr::from_ref((__self_dyn as &A).raw().x())
                    .byte_offset_from(core::ptr::from_ref(__self_dyn))
            });
            println!("offset!(M, y): {}", unsafe {
                core::ptr::from_ref(__self_dyn.raw().y())
                    .byte_offset_from(core::ptr::from_ref(__self_dyn))
            });
        }
    },
>;

#[class(extends(A), with(M))]
type B = class<
    {
        let z: u32 = 2_u32;
        pub fn new() -> Self {
            Self { ..Super::new() }
        }

        #[method(override(A))]
        pub fn print_offsets(&self) {
            super.print_offsets();
            println!("offset!(B, x): {}", unsafe {
                core::ptr::from_ref(__self_dyn.raw().x()).byte_offset_from(__self)
            });
            println!("offset!(B, y): {}", unsafe {
                core::ptr::from_ref((__self_dyn as &M).raw().y()).byte_offset_from(__self)
            });
            println!("offset!(B, z): {}", unsafe {
                core::ptr::from_ref(__self_dyn.raw().z()).byte_offset_from(__self)
            });
        }
    },
>;

static EXPECTED: &[&str] = &[
    // <CRc<B>>::print_offsets
    "offset!(A, x): 0",
    "offset!(A/M, x): 0",
    "offset!(A/M, y): 2",
    "offset!(M, x): 0",
    "offset!(M, y): 2",
    "offset!(B, x): 0",
    "offset!(B, y): 2",
    "offset!(B, z): 4",
    // <CRc<B> as CRc<M>>::print_offsets
    "offset!(A, x): 0",
    "offset!(A/M, x): 0",
    "offset!(A/M, y): 2",
    "offset!(M, x): 0",
    "offset!(M, y): 2",
    "offset!(B, x): 0",
    "offset!(B, y): 2",
    "offset!(B, z): 4",
];

#[test]
fn test_get_set_alignment() {
    let b = B::new();
    b.print_offsets();
    (&*b as &M).print_offsets();
    assert_eq!(BUF.take(), EXPECTED);
}
