// Cross-module polymorphism tests
// Test polymorphic behavior between classes in different modules

use crate::*;
use classes::prelude::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_polymorphic_call_consistency(
        radius in 0.1f64..1000.0f64,
        width in 0.1f64..1000.0f64,
        height in 0.1f64..1000.0f64,
        side_a in 0.1f64..1000.0f64,
        side_b in 0.1f64..1000.0f64,
        side_c in 0.1f64..1000.0f64
    ) {
        prop_assume!(side_a + side_b > side_c);
        prop_assume!(side_a + side_c > side_b);
        prop_assume!(side_b + side_c > side_a);

        let epsilon = 1e-10;

        // 测试 Circle 的多态行为
        let circle = Circle::new(radius, "red".to_string());
        let circle_as_shape: CRc<Shape> = circle.clone().into_super();

        let direct_area = circle.area();
        let polymorphic_area = circle_as_shape.area();
        prop_assert!(
            (direct_area - polymorphic_area).abs() < epsilon,
            "Circle polymorphic area mismatch: direct={}, polymorphic={}",
            direct_area,
            polymorphic_area
        );

        let direct_perimeter = circle.perimeter();
        let polymorphic_perimeter = circle_as_shape.perimeter();
        prop_assert!(
            (direct_perimeter - polymorphic_perimeter).abs() < epsilon,
            "Circle polymorphic perimeter mismatch: direct={}, polymorphic={}",
            direct_perimeter,
            polymorphic_perimeter
        );

        let direct_description = circle.description();
        let polymorphic_description = circle_as_shape.description();
        prop_assert!(
            direct_description == polymorphic_description,
            "Circle polymorphic description mismatch: direct='{}', polymorphic='{}'",
            direct_description,
            polymorphic_description
        );

        // 测试 Rectangle 的多态行为
        let rectangle = Rectangle::new(width, height, "blue".to_string());
        let rectangle_as_shape: CRc<Shape> = rectangle.clone().into_super();

        let direct_area = rectangle.area();
        let polymorphic_area = rectangle_as_shape.area();
        prop_assert!(
            (direct_area - polymorphic_area).abs() < epsilon,
            "Rectangle polymorphic area mismatch: direct={}, polymorphic={}",
            direct_area,
            polymorphic_area
        );

        let direct_perimeter = rectangle.perimeter();
        let polymorphic_perimeter = rectangle_as_shape.perimeter();
        prop_assert!(
            (direct_perimeter - polymorphic_perimeter).abs() < epsilon,
            "Rectangle polymorphic perimeter mismatch: direct={}, polymorphic={}",
            direct_perimeter,
            polymorphic_perimeter
        );

        let direct_description = rectangle.description();
        let polymorphic_description = rectangle_as_shape.description();
        prop_assert!(
            direct_description == polymorphic_description,
            "Rectangle polymorphic description mismatch: direct='{}', polymorphic='{}'",
            direct_description,
            polymorphic_description
        );

        // 测试 Triangle 的多态行为
        let triangle = Triangle::new(side_a, side_b, side_c, "green".to_string());
        let triangle_as_shape: CRc<Shape> = triangle.clone().into_super();

        let direct_area = triangle.area();
        let polymorphic_area = triangle_as_shape.area();
        prop_assert!(
            (direct_area - polymorphic_area).abs() < epsilon,
            "Triangle polymorphic area mismatch: direct={}, polymorphic={}",
            direct_area,
            polymorphic_area
        );

        let direct_perimeter = triangle.perimeter();
        let polymorphic_perimeter = triangle_as_shape.perimeter();
        prop_assert!(
            (direct_perimeter - polymorphic_perimeter).abs() < epsilon,
            "Triangle polymorphic perimeter mismatch: direct={}, polymorphic={}",
            direct_perimeter,
            polymorphic_perimeter
        );

        let direct_description = triangle.description();
        let polymorphic_description = triangle_as_shape.description();
        prop_assert!(
            direct_description == polymorphic_description,
            "Triangle polymorphic description mismatch: direct='{}', polymorphic='{}'",
            direct_description,
            polymorphic_description
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_type_conversion_round_trip_consistency(
        radius in 0.1f64..1000.0f64,
        width in 0.1f64..1000.0f64,
        height in 0.1f64..1000.0f64,
        side_a in 0.1f64..1000.0f64,
        side_b in 0.1f64..1000.0f64,
        side_c in 0.1f64..1000.0f64
    ) {
        prop_assume!(side_a + side_b > side_c);
        prop_assume!(side_a + side_c > side_b);
        prop_assume!(side_b + side_c > side_a);

        let epsilon = 1e-10;

        // 测试 Circle 的类型转换往返一致性
        let circle = Circle::new(radius, "red".to_string());
        let original_area = circle.area();
        let original_perimeter = circle.perimeter();
        let original_description = circle.description();

        let shape: CRc<Shape> = circle.clone().into_super();
        let circle_back = shape.try_into_subtype::<CRc<Circle>>();
        prop_assert!(
            circle_back.is_some(),
            "Failed to downcast Shape back to Circle"
        );

        let circle_back = circle_back.unwrap();

        prop_assert!(
            (circle_back.area() - original_area).abs() < epsilon,
            "Circle area changed after round-trip conversion: original={}, after={}",
            original_area,
            circle_back.area()
        );

        prop_assert!(
            (circle_back.perimeter() - original_perimeter).abs() < epsilon,
            "Circle perimeter changed after round-trip conversion: original={}, after={}",
            original_perimeter,
            circle_back.perimeter()
        );

        prop_assert!(
            circle_back.description() == original_description,
            "Circle description changed after round-trip conversion: original='{}', after='{}'",
            original_description,
            circle_back.description()
        );

        prop_assert!(
            (circle_back.get_radius() - radius).abs() < epsilon,
            "Circle radius changed after round-trip conversion: original={}, after={}",
            radius,
            circle_back.get_radius()
        );

        // 测试 Rectangle 的类型转换往返一致性
        let rectangle = Rectangle::new(width, height, "blue".to_string());
        let original_area = rectangle.area();
        let original_perimeter = rectangle.perimeter();
        let original_description = rectangle.description();

        let shape: CRc<Shape> = rectangle.clone().into_super();
        let rectangle_back = shape.try_into_subtype::<CRc<Rectangle>>();
        prop_assert!(
            rectangle_back.is_some(),
            "Failed to downcast Shape back to Rectangle"
        );

        let rectangle_back = rectangle_back.unwrap();

        prop_assert!(
            (rectangle_back.area() - original_area).abs() < epsilon,
            "Rectangle area changed after round-trip conversion: original={}, after={}",
            original_area,
            rectangle_back.area()
        );

        prop_assert!(
            (rectangle_back.perimeter() - original_perimeter).abs() < epsilon,
            "Rectangle perimeter changed after round-trip conversion: original={}, after={}",
            original_perimeter,
            rectangle_back.perimeter()
        );

        prop_assert!(
            rectangle_back.description() == original_description,
            "Rectangle description changed after round-trip conversion: original='{}', after='{}'",
            original_description,
            rectangle_back.description()
        );

        prop_assert!(
            (rectangle_back.get_width() - width).abs() < epsilon,
            "Rectangle width changed after round-trip conversion: original={}, after={}",
            width,
            rectangle_back.get_width()
        );

        prop_assert!(
            (rectangle_back.get_height() - height).abs() < epsilon,
            "Rectangle height changed after round-trip conversion: original={}, after={}",
            height,
            rectangle_back.get_height()
        );

        // 测试 Triangle 的类型转换往返一致性
        let triangle = Triangle::new(side_a, side_b, side_c, "green".to_string());
        let original_area = triangle.area();
        let original_perimeter = triangle.perimeter();
        let original_description = triangle.description();

        let shape: CRc<Shape> = triangle.clone().into_super();
        let triangle_back = shape.try_into_subtype::<CRc<Triangle>>();
        prop_assert!(
            triangle_back.is_some(),
            "Failed to downcast Shape back to Triangle"
        );

        let triangle_back = triangle_back.unwrap();

        prop_assert!(
            (triangle_back.area() - original_area).abs() < epsilon,
            "Triangle area changed after round-trip conversion: original={}, after={}",
            original_area,
            triangle_back.area()
        );

        prop_assert!(
            (triangle_back.perimeter() - original_perimeter).abs() < epsilon,
            "Triangle perimeter changed after round-trip conversion: original={}, after={}",
            original_perimeter,
            triangle_back.perimeter()
        );

        prop_assert!(
            triangle_back.description() == original_description,
            "Triangle description changed after round-trip conversion: original='{}', after='{}'",
            original_description,
            triangle_back.description()
        );

        prop_assert!(
            (triangle_back.get_side_a() - side_a).abs() < epsilon,
            "Triangle side_a changed after round-trip conversion: original={}, after={}",
            side_a,
            triangle_back.get_side_a()
        );

        prop_assert!(
            (triangle_back.get_side_b() - side_b).abs() < epsilon,
            "Triangle side_b changed after round-trip conversion: original={}, after={}",
            side_b,
            triangle_back.get_side_b()
        );

        prop_assert!(
            (triangle_back.get_side_c() - side_c).abs() < epsilon,
            "Triangle side_c changed after round-trip conversion: original={}, after={}",
            side_c,
            triangle_back.get_side_c()
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_color_property_inheritance(
        radius in 0.1f64..1000.0f64,
        width in 0.1f64..1000.0f64,
        height in 0.1f64..1000.0f64,
        side_a in 0.1f64..1000.0f64,
        side_b in 0.1f64..1000.0f64,
        side_c in 0.1f64..1000.0f64,
        color in "[a-z]{3,10}"
    ) {
        prop_assume!(side_a + side_b > side_c);
        prop_assume!(side_a + side_c > side_b);
        prop_assume!(side_b + side_c > side_a);

        // 测试 Circle 的颜色属性继承
        let circle = Circle::new(radius, color.clone());
        let circle_color_direct = circle.get_color();

        let shape: CRc<Shape> = circle.clone().into_super();
        let circle_color_via_shape = shape.get_color();

        prop_assert!(
            circle_color_direct == circle_color_via_shape,
            "Circle color mismatch: direct={:?}, via Shape={:?}",
            circle_color_direct,
            circle_color_via_shape
        );

        prop_assert!(
            *circle_color_direct == Some(color.clone()),
            "Circle color should be {:?}, but got {:?}",
            Some(color.clone()),
            circle_color_direct
        );

        // 测试 Rectangle 的颜色属性继承
        let rectangle = Rectangle::new(width, height, color.clone());
        let rectangle_color_direct = rectangle.get_color();

        let shape: CRc<Shape> = rectangle.clone().into_super();
        let rectangle_color_via_shape = shape.get_color();

        prop_assert!(
            rectangle_color_direct == rectangle_color_via_shape,
            "Rectangle color mismatch: direct={:?}, via Shape={:?}",
            rectangle_color_direct,
            rectangle_color_via_shape
        );

        prop_assert!(
            *rectangle_color_direct == Some(color.clone()),
            "Rectangle color should be {:?}, but got {:?}",
            Some(color.clone()),
            rectangle_color_direct
        );

        // 测试 Triangle 的颜色属性继承
        let triangle = Triangle::new(side_a, side_b, side_c, color.clone());
        let triangle_color_direct = triangle.get_color();

        let shape: CRc<Shape> = triangle.clone().into_super();
        let triangle_color_via_shape = shape.get_color();

        prop_assert!(
            triangle_color_direct == triangle_color_via_shape,
            "Triangle color mismatch: direct={:?}, via Shape={:?}",
            triangle_color_direct,
            triangle_color_via_shape
        );

        prop_assert!(
            *triangle_color_direct == Some(color.clone()),
            "Triangle color should be {:?}, but got {:?}",
            Some(color.clone()),
            triangle_color_direct
        );
    }
}
