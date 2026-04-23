use oop_rs::prelude::*;
#[class(abstract)]
type TestSuper = class<
    {
        let z: i32;
        let w: i32;

        #[builder]
        fn new(z: i32, #[default(5)] w: i32) -> Self {
            Self { z, w }
        }

        #[builder]
        fn two_w(#[default(1)] z: i32) -> Self {
            Self { z, w: 2 }
        }

        #[builder = "free_w"]
        fn two_z(#[default(1)] w: i32) -> Self {
            Self { z: 2, w }
        }

        #[helper]
        fn static_fn(#[default(1)] x: i32) -> i32 {
            x
        }

        #[helper = "static_fn2_helper"]
        fn static_fn2(#[default(2)] x: i32) -> i32 {
            x
        }
    },
>;
#[class(extends(TestSuper))]
type Test = class<
    {
        let x: i32;
        let y: i32;

        #[builder]
        fn new(x: i32, #[default(5)] y: i32, #[default(3)] z: i32, #[default(3)] w: i32) -> Self {
            Self {
                x,
                y,
                ..Super::new(z, w)
            }
        }

        #[builder]
        #[allow(unused_variables)]
        fn one_y(x: i32, #[default(5)] y: i32, #[default(3)] z: i32, #[default(3)] w: i32) -> Self {
            Self {
                x,
                y: 1,
                ..Super::two_w_builder().build()
            }
        }

        #[builder = "free_y"]
        fn one_x(#[default(5)] y: i32) -> Self {
            Self {
                x: 1,
                y,
                ..Super::free_w().build()
            }
        }

        fn x(&self) -> i32 {
            self.get().x()
        }

        fn y(&self) -> i32 {
            self.get().y()
        }

        fn z(&self) -> i32 {
            self.get().z()
        }

        fn w(&self) -> i32 {
            self.get().w()
        }

        #[helper = "x_rec_helper"]
        fn cal_x_rectangle(&self, #[default(5)] y: i32) -> i32 {
            self.x() * y
        }

        #[helper = "x_ref_helper"]
        fn test_reference(&self, x: &i32, #[default(None)] y: Option<&i32>) -> i32 {
            *y.unwrap_or(x)
        }

        #[helper]
        fn static_fn(#[default(1)] x: i32) -> i32 {
            x
        }

        #[helper = "static_fn2_helper"]
        fn static_fn2(#[default(2)] x: i32) -> i32 {
            x
        }
    },
>;

#[test]
fn test_builder() {
    let sample = Test::builder(1).y(2).z(4).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 2);
    assert_eq!(sample.z(), 4);
    assert_eq!(sample.w(), 3);

    let sample = Test::builder(1).y(2).z(4).w(5).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 2);
    assert_eq!(sample.z(), 4);
    assert_eq!(sample.w(), 5);

    let sample = Test::builder(1).y(2).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 2);
    assert_eq!(sample.z(), 3);
    assert_eq!(sample.w(), 3);

    let sample = Test::builder(1).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 5);
    assert_eq!(sample.z(), 3);
    assert_eq!(sample.w(), 3);

    let sample = Test::builder(1).x(3).y(4).build();
    assert_eq!(sample.x(), 3);
    assert_eq!(sample.y(), 4);
    assert_eq!(sample.z(), 3);
    assert_eq!(sample.w(), 3);

    let sample = Test::one_y_builder(7).build();
    assert_eq!(sample.x(), 7);
    assert_eq!(sample.y(), 1);
    assert_eq!(sample.z(), 1);
    assert_eq!(sample.w(), 2);

    let sample = Test::free_y().y(4).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 4);
    assert_eq!(sample.z(), 2);
    assert_eq!(sample.w(), 1);
}

#[test]
fn test_helper() {
    let sample = Test::builder(3).build();

    let result = sample.x_rec_helper().call();
    assert_eq!(result, 15);

    let result = sample.x_rec_helper().y(7).call();
    assert_eq!(result, 21);

    let result = sample.x_rec_helper().y(9).call();
    assert_eq!(result, 27);

    let result = sample.x_ref_helper(&10).call();
    assert_eq!(result, 10);

    let result = sample.x_ref_helper(&10).y(Some(&20)).call();
    assert_eq!(result, 20);
}

#[test]
fn test_static_fn() {
    let result = TestSuper::static_fn_helper().call();
    assert_eq!(result, 1);

    let result = TestSuper::static_fn_helper().x(2).call();
    assert_eq!(result, 2);

    let result = TestSuper::static_fn2_helper().call();
    assert_eq!(result, 2);

    let result = TestSuper::static_fn2_helper().x(3).call();
    assert_eq!(result, 3);

    let result = Test::static_fn_helper().call();
    assert_eq!(result, 1);

    let result = Test::static_fn_helper().x(2).call();
    assert_eq!(result, 2);

    let result = Test::static_fn2_helper().call();
    assert_eq!(result, 2);

    let result = Test::static_fn2_helper().x(3).call();
    assert_eq!(result, 3);
}
