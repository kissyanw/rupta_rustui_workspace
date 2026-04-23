// Multi-level inheritance tests
// Test polymorphic behavior and type conversion of second-level derived classes

use crate::*;
use classes::prelude::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_multilevel_polymorphic_consistency(
        radius in 0.1f64..1000.0f64,
        border_color in "[a-z]{3,10}",
        fill_color in "[a-z]{3,10}",
        semi_major_axis in 0.1f64..1000.0f64,
        semi_minor_axis in 0.1f64..1000.0f64,
        side in 0.1f64..1000.0f64,
        width in 0.1f64..1000.0f64,
        height in 0.1f64..1000.0f64,
        corner_radius in 0.1f64..100.0f64
    ) {
        let epsilon = 1e-10;

        // 测试 ColoredCircle 的多级继承多态行为
        let colored_circle = ColoredCircle::new(
            radius,
            border_color.clone(),
            fill_color.clone()
        );

        let direct_area = colored_circle.area();
        let direct_perimeter = colored_circle.perimeter();
        let direct_description = colored_circle.description();

        let colored_as_circle: CRc<Circle> = colored_circle.clone().into_super();
        let area_via_circle = colored_as_circle.area();
        let perimeter_via_circle = colored_as_circle.perimeter();
        let description_via_circle = colored_as_circle.description();

        prop_assert!(
            (direct_area - area_via_circle).abs() < epsilon,
            "ColoredCircle area via Circle mismatch: direct={}, via Circle={}",
            direct_area,
            area_via_circle
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_circle).abs() < epsilon,
            "ColoredCircle perimeter via Circle mismatch: direct={}, via Circle={}",
            direct_perimeter,
            perimeter_via_circle
        );

        prop_assert!(
            direct_description == description_via_circle,
            "ColoredCircle description via Circle mismatch: direct='{}', via Circle='{}'",
            direct_description,
            description_via_circle
        );

        let colored_as_shape: CRc<Shape> = colored_as_circle.into_super();
        let area_via_shape = colored_as_shape.area();
        let perimeter_via_shape = colored_as_shape.perimeter();
        let description_via_shape = colored_as_shape.description();

        prop_assert!(
            (direct_area - area_via_shape).abs() < epsilon,
            "ColoredCircle area via Shape mismatch: direct={}, via Shape={}",
            direct_area,
            area_via_shape
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_shape).abs() < epsilon,
            "ColoredCircle perimeter via Shape mismatch: direct={}, via Shape={}",
            direct_perimeter,
            perimeter_via_shape
        );

        prop_assert!(
            direct_description == description_via_shape,
            "ColoredCircle description via Shape mismatch: direct='{}', via Shape='{}'",
            direct_description,
            description_via_shape
        );

        // 测试 Ellipse 的多级继承多态行为
        let ellipse = Ellipse::new(semi_major_axis, semi_minor_axis, "test".to_string());

        let direct_area = ellipse.area();
        let direct_perimeter = ellipse.perimeter();
        let direct_description = ellipse.description();

        let ellipse_as_circle: CRc<Circle> = ellipse.clone().into_super();
        let area_via_circle = ellipse_as_circle.area();
        let perimeter_via_circle = ellipse_as_circle.perimeter();
        let description_via_circle = ellipse_as_circle.description();

        prop_assert!(
            (direct_area - area_via_circle).abs() < epsilon,
            "Ellipse area via Circle mismatch: direct={}, via Circle={}",
            direct_area,
            area_via_circle
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_circle).abs() < epsilon,
            "Ellipse perimeter via Circle mismatch: direct={}, via Circle={}",
            direct_perimeter,
            perimeter_via_circle
        );

        prop_assert!(
            direct_description == description_via_circle,
            "Ellipse description via Circle mismatch: direct='{}', via Circle='{}'",
            direct_description,
            description_via_circle
        );

        let ellipse_as_shape: CRc<Shape> = ellipse_as_circle.into_super();
        let area_via_shape = ellipse_as_shape.area();
        let perimeter_via_shape = ellipse_as_shape.perimeter();
        let description_via_shape = ellipse_as_shape.description();

        prop_assert!(
            (direct_area - area_via_shape).abs() < epsilon,
            "Ellipse area via Shape mismatch: direct={}, via Shape={}",
            direct_area,
            area_via_shape
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_shape).abs() < epsilon,
            "Ellipse perimeter via Shape mismatch: direct={}, via Shape={}",
            direct_perimeter,
            perimeter_via_shape
        );

        prop_assert!(
            direct_description == description_via_shape,
            "Ellipse description via Shape mismatch: direct='{}', via Shape='{}'",
            direct_description,
            description_via_shape
        );

        // 测试 Square 的多级继承多态行为
        let square = Square::new(side, "test".to_string());

        let direct_area = square.area();
        let direct_perimeter = square.perimeter();
        let direct_description = square.description();

        let square_as_rectangle: CRc<Rectangle> = square.clone().into_super();
        let area_via_rectangle = square_as_rectangle.area();
        let perimeter_via_rectangle = square_as_rectangle.perimeter();
        let description_via_rectangle = square_as_rectangle.description();

        prop_assert!(
            (direct_area - area_via_rectangle).abs() < epsilon,
            "Square area via Rectangle mismatch: direct={}, via Rectangle={}",
            direct_area,
            area_via_rectangle
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_rectangle).abs() < epsilon,
            "Square perimeter via Rectangle mismatch: direct={}, via Rectangle={}",
            direct_perimeter,
            perimeter_via_rectangle
        );

        prop_assert!(
            direct_description == description_via_rectangle,
            "Square description via Rectangle mismatch: direct='{}', via Rectangle='{}'",
            direct_description,
            description_via_rectangle
        );

        let square_as_shape: CRc<Shape> = square_as_rectangle.into_super();
        let area_via_shape = square_as_shape.area();
        let perimeter_via_shape = square_as_shape.perimeter();
        let description_via_shape = square_as_shape.description();

        prop_assert!(
            (direct_area - area_via_shape).abs() < epsilon,
            "Square area via Shape mismatch: direct={}, via Shape={}",
            direct_area,
            area_via_shape
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_shape).abs() < epsilon,
            "Square perimeter via Shape mismatch: direct={}, via Shape={}",
            direct_perimeter,
            perimeter_via_shape
        );

        prop_assert!(
            direct_description == description_via_shape,
            "Square description via Shape mismatch: direct='{}', via Shape='{}'",
            direct_description,
            description_via_shape
        );

        // 测试 RoundedRectangle 的多级继承多态行为
        let max_radius = (width.min(height)) / 2.0;
        prop_assume!(corner_radius <= max_radius);

        let rounded_rect = RoundedRectangle::new(
            width,
            height,
            corner_radius,
            "test".to_string()
        );

        let direct_area = rounded_rect.area();
        let direct_perimeter = rounded_rect.perimeter();
        let direct_description = rounded_rect.description();

        let rounded_as_rectangle: CRc<Rectangle> = rounded_rect.clone().into_super();
        let area_via_rectangle = rounded_as_rectangle.area();
        let perimeter_via_rectangle = rounded_as_rectangle.perimeter();
        let description_via_rectangle = rounded_as_rectangle.description();

        prop_assert!(
            (direct_area - area_via_rectangle).abs() < epsilon,
            "RoundedRectangle area via Rectangle mismatch: direct={}, via Rectangle={}",
            direct_area,
            area_via_rectangle
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_rectangle).abs() < epsilon,
            "RoundedRectangle perimeter via Rectangle mismatch: direct={}, via Rectangle={}",
            direct_perimeter,
            perimeter_via_rectangle
        );

        prop_assert!(
            direct_description == description_via_rectangle,
            "RoundedRectangle description via Rectangle mismatch: direct='{}', via Rectangle='{}'",
            direct_description,
            description_via_rectangle
        );

        let rounded_as_shape: CRc<Shape> = rounded_as_rectangle.into_super();
        let area_via_shape = rounded_as_shape.area();
        let perimeter_via_shape = rounded_as_shape.perimeter();
        let description_via_shape = rounded_as_shape.description();

        prop_assert!(
            (direct_area - area_via_shape).abs() < epsilon,
            "RoundedRectangle area via Shape mismatch: direct={}, via Shape={}",
            direct_area,
            area_via_shape
        );

        prop_assert!(
            (direct_perimeter - perimeter_via_shape).abs() < epsilon,
            "RoundedRectangle perimeter via Shape mismatch: direct={}, via Shape={}",
            direct_perimeter,
            perimeter_via_shape
        );

        prop_assert!(
            direct_description == description_via_shape,
            "RoundedRectangle description via Shape mismatch: direct='{}', via Shape='{}'",
            direct_description,
            description_via_shape
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_multilevel_type_conversion_round_trip(
        radius in 0.1f64..1000.0f64,
        border_color in "[a-z]{3,10}",
        fill_color in "[a-z]{3,10}",
        semi_major_axis in 0.1f64..1000.0f64,
        semi_minor_axis in 0.1f64..1000.0f64,
        side in 0.1f64..1000.0f64,
        width in 0.1f64..1000.0f64,
        height in 0.1f64..1000.0f64,
        corner_radius in 0.1f64..100.0f64
    ) {
        let epsilon = 1e-10;

        // 测试 ColoredCircle 的多级类型转换往返一致性
        let colored_circle = ColoredCircle::new(
            radius,
            border_color.clone(),
            fill_color.clone()
        );

        let original_area = colored_circle.area();
        let original_perimeter = colored_circle.perimeter();
        let original_description = colored_circle.description();
        let original_radius = colored_circle.get_radius();
        let original_border_color = colored_circle.get_color();
        let original_fill_color = colored_circle.get_fill_color();

        let colored_as_circle: CRc<Circle> = colored_circle.clone().into_super();
        let colored_as_shape: CRc<Shape> = colored_as_circle.into_super();

        let circle_back = colored_as_shape.try_into_subtype::<CRc<Circle>>();
        prop_assert!(
            circle_back.is_some(),
            "Failed to downcast Shape back to Circle for ColoredCircle"
        );

        let circle_back = circle_back.unwrap();
        let colored_back = circle_back.try_into_subtype::<CRc<ColoredCircle>>();
        prop_assert!(
            colored_back.is_some(),
            "Failed to downcast Circle back to ColoredCircle"
        );

        let colored_back = colored_back.unwrap();

        prop_assert!(
            (colored_back.area() - original_area).abs() < epsilon,
            "ColoredCircle area changed after round-trip: original={}, after={}",
            original_area,
            colored_back.area()
        );

        prop_assert!(
            (colored_back.perimeter() - original_perimeter).abs() < epsilon,
            "ColoredCircle perimeter changed after round-trip: original={}, after={}",
            original_perimeter,
            colored_back.perimeter()
        );

        prop_assert!(
            colored_back.description() == original_description,
            "ColoredCircle description changed after round-trip: original='{}', after='{}'",
            original_description,
            colored_back.description()
        );

        prop_assert!(
            (colored_back.get_radius() - original_radius).abs() < epsilon,
            "ColoredCircle radius changed after round-trip: original={}, after={}",
            original_radius,
            colored_back.get_radius()
        );

        prop_assert!(
            colored_back.get_color() == original_border_color,
            "ColoredCircle border color changed after round-trip: original={:?}, after={:?}",
            original_border_color,
            colored_back.get_color()
        );

        prop_assert!(
            colored_back.get_fill_color() == original_fill_color,
            "ColoredCircle fill color changed after round-trip: original={:?}, after={:?}",
            original_fill_color,
            colored_back.get_fill_color()
        );

        // 测试 Ellipse 的多级类型转换往返一致性
        let ellipse = Ellipse::new(semi_major_axis, semi_minor_axis, "test".to_string());

        let original_area = ellipse.area();
        let original_perimeter = ellipse.perimeter();
        let original_description = ellipse.description();
        let original_major = ellipse.get_radius();
        let original_minor = ellipse.get_semi_minor_axis();

        let ellipse_as_circle: CRc<Circle> = ellipse.clone().into_super();
        let ellipse_as_shape: CRc<Shape> = ellipse_as_circle.into_super();

        let circle_back = ellipse_as_shape.try_into_subtype::<CRc<Circle>>();
        prop_assert!(
            circle_back.is_some(),
            "Failed to downcast Shape back to Circle for Ellipse"
        );

        let circle_back = circle_back.unwrap();
        let ellipse_back = circle_back.try_into_subtype::<CRc<Ellipse>>();
        prop_assert!(
            ellipse_back.is_some(),
            "Failed to downcast Circle back to Ellipse"
        );

        let ellipse_back = ellipse_back.unwrap();

        prop_assert!(
            (ellipse_back.area() - original_area).abs() < epsilon,
            "Ellipse area changed after round-trip: original={}, after={}",
            original_area,
            ellipse_back.area()
        );

        prop_assert!(
            (ellipse_back.perimeter() - original_perimeter).abs() < epsilon,
            "Ellipse perimeter changed after round-trip: original={}, after={}",
            original_perimeter,
            ellipse_back.perimeter()
        );

        prop_assert!(
            ellipse_back.description() == original_description,
            "Ellipse description changed after round-trip: original='{}', after='{}'",
            original_description,
            ellipse_back.description()
        );

        prop_assert!(
            (ellipse_back.get_radius() - original_major).abs() < epsilon,
            "Ellipse semi_major_axis changed after round-trip: original={}, after={}",
            original_major,
            ellipse_back.get_radius()
        );

        prop_assert!(
            (ellipse_back.get_semi_minor_axis() - original_minor).abs() < epsilon,
            "Ellipse semi_minor_axis changed after round-trip: original={}, after={}",
            original_minor,
            ellipse_back.get_semi_minor_axis()
        );

        // 测试 Square 的多级类型转换往返一致性
        let square = Square::new(side, "test".to_string());

        let original_area = square.area();
        let original_perimeter = square.perimeter();
        let original_description = square.description();
        let original_side = square.get_width();

        let square_as_rectangle: CRc<Rectangle> = square.clone().into_super();
        let square_as_shape: CRc<Shape> = square_as_rectangle.into_super();

        let rectangle_back = square_as_shape.try_into_subtype::<CRc<Rectangle>>();
        prop_assert!(
            rectangle_back.is_some(),
            "Failed to downcast Shape back to Rectangle for Square"
        );

        let rectangle_back = rectangle_back.unwrap();
        let square_back = rectangle_back.try_into_subtype::<CRc<Square>>();
        prop_assert!(
            square_back.is_some(),
            "Failed to downcast Rectangle back to Square"
        );

        let square_back = square_back.unwrap();

        prop_assert!(
            (square_back.area() - original_area).abs() < epsilon,
            "Square area changed after round-trip: original={}, after={}",
            original_area,
            square_back.area()
        );

        prop_assert!(
            (square_back.perimeter() - original_perimeter).abs() < epsilon,
            "Square perimeter changed after round-trip: original={}, after={}",
            original_perimeter,
            square_back.perimeter()
        );

        prop_assert!(
            square_back.description() == original_description,
            "Square description changed after round-trip: original='{}', after='{}'",
            original_description,
            square_back.description()
        );

        prop_assert!(
            (square_back.get_width() - original_side).abs() < epsilon,
            "Square side changed after round-trip: original={}, after={}",
            original_side,
            square_back.get_width()
        );

        prop_assert!(
            (square_back.get_width() - square_back.get_height()).abs() < epsilon,
            "Square width and height should still be equal after round-trip: width={}, height={}",
            square_back.get_width(),
            square_back.get_height()
        );

        // 测试 RoundedRectangle 的多级类型转换往返一致性
        let max_radius = (width.min(height)) / 2.0;
        prop_assume!(corner_radius <= max_radius);

        let rounded_rect = RoundedRectangle::new(
            width,
            height,
            corner_radius,
            "test".to_string()
        );

        let original_area = rounded_rect.area();
        let original_perimeter = rounded_rect.perimeter();
        let original_description = rounded_rect.description();
        let original_width = rounded_rect.get_width();
        let original_height = rounded_rect.get_height();
        let original_corner_radius = rounded_rect.get_corner_radius();

        let rounded_as_rectangle: CRc<Rectangle> = rounded_rect.clone().into_super();
        let rounded_as_shape: CRc<Shape> = rounded_as_rectangle.into_super();

        let rectangle_back = rounded_as_shape.try_into_subtype::<CRc<Rectangle>>();
        prop_assert!(
            rectangle_back.is_some(),
            "Failed to downcast Shape back to Rectangle for RoundedRectangle"
        );

        let rectangle_back = rectangle_back.unwrap();
        let rounded_back = rectangle_back.try_into_subtype::<CRc<RoundedRectangle>>();
        prop_assert!(
            rounded_back.is_some(),
            "Failed to downcast Rectangle back to RoundedRectangle"
        );

        let rounded_back = rounded_back.unwrap();

        prop_assert!(
            (rounded_back.area() - original_area).abs() < epsilon,
            "RoundedRectangle area changed after round-trip: original={}, after={}",
            original_area,
            rounded_back.area()
        );

        prop_assert!(
            (rounded_back.perimeter() - original_perimeter).abs() < epsilon,
            "RoundedRectangle perimeter changed after round-trip: original={}, after={}",
            original_perimeter,
            rounded_back.perimeter()
        );

        prop_assert!(
            rounded_back.description() == original_description,
            "RoundedRectangle description changed after round-trip: original='{}', after='{}'",
            original_description,
            rounded_back.description()
        );

        prop_assert!(
            (rounded_back.get_width() - original_width).abs() < epsilon,
            "RoundedRectangle width changed after round-trip: original={}, after={}",
            original_width,
            rounded_back.get_width()
        );

        prop_assert!(
            (rounded_back.get_height() - original_height).abs() < epsilon,
            "RoundedRectangle height changed after round-trip: original={}, after={}",
            original_height,
            rounded_back.get_height()
        );

        prop_assert!(
            (rounded_back.get_corner_radius() - original_corner_radius).abs() < epsilon,
            "RoundedRectangle corner_radius changed after round-trip: original={}, after={}",
            original_corner_radius,
            rounded_back.get_corner_radius()
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_inherited_property_transitivity(
        radius in 0.1f64..1000.0f64,
        border_color in "[a-z]{3,10}",
        fill_color in "[a-z]{3,10}",
        semi_major_axis in 0.1f64..1000.0f64,
        semi_minor_axis in 0.1f64..1000.0f64,
        side in 0.1f64..1000.0f64,
        width in 0.1f64..1000.0f64,
        height in 0.1f64..1000.0f64,
        corner_radius in 0.1f64..100.0f64,
        color in "[a-z]{3,10}"
    ) {
        // 测试 ColoredCircle 的颜色属性传递性
        let colored_circle = ColoredCircle::new(
            radius,
            border_color.clone(),
            fill_color.clone()
        );

        let color_direct = colored_circle.get_color();

        let colored_as_circle: CRc<Circle> = colored_circle.clone().into_super();
        let color_via_circle = colored_as_circle.get_color();

        let colored_as_shape: CRc<Shape> = colored_as_circle.clone().into_super();
        let color_via_shape = colored_as_shape.get_color();

        prop_assert!(
            color_direct == color_via_circle,
            "ColoredCircle color mismatch: direct={:?}, via Circle={:?}",
            color_direct,
            color_via_circle
        );

        prop_assert!(
            color_direct == color_via_shape,
            "ColoredCircle color mismatch: direct={:?}, via Shape={:?}",
            color_direct,
            color_via_shape
        );

        prop_assert!(
            *color_direct == Some(border_color.clone()),
            "ColoredCircle color should be {:?}, but got {:?}",
            Some(border_color.clone()),
            color_direct
        );

        // 测试 Ellipse 的颜色属性传递性
        let ellipse = Ellipse::new(semi_major_axis, semi_minor_axis, color.clone());

        let color_direct = ellipse.get_color();

        let ellipse_as_circle: CRc<Circle> = ellipse.clone().into_super();
        let color_via_circle = ellipse_as_circle.get_color();

        let ellipse_as_shape: CRc<Shape> = ellipse_as_circle.clone().into_super();
        let color_via_shape = ellipse_as_shape.get_color();

        prop_assert!(
            color_direct == color_via_circle,
            "Ellipse color mismatch: direct={:?}, via Circle={:?}",
            color_direct,
            color_via_circle
        );

        prop_assert!(
            color_direct == color_via_shape,
            "Ellipse color mismatch: direct={:?}, via Shape={:?}",
            color_direct,
            color_via_shape
        );

        prop_assert!(
            *color_direct == Some(color.clone()),
            "Ellipse color should be {:?}, but got {:?}",
            Some(color.clone()),
            color_direct
        );

        // 测试 Square 的颜色属性传递性
        let square = Square::new(side, color.clone());

        let color_direct = square.get_color();

        let square_as_rectangle: CRc<Rectangle> = square.clone().into_super();
        let color_via_rectangle = square_as_rectangle.get_color();

        let square_as_shape: CRc<Shape> = square_as_rectangle.clone().into_super();
        let color_via_shape = square_as_shape.get_color();

        prop_assert!(
            color_direct == color_via_rectangle,
            "Square color mismatch: direct={:?}, via Rectangle={:?}",
            color_direct,
            color_via_rectangle
        );

        prop_assert!(
            color_direct == color_via_shape,
            "Square color mismatch: direct={:?}, via Shape={:?}",
            color_direct,
            color_via_shape
        );

        prop_assert!(
            *color_direct == Some(color.clone()),
            "Square color should be {:?}, but got {:?}",
            Some(color.clone()),
            color_direct
        );

        // 测试 RoundedRectangle 的颜色属性传递性
        let max_radius = (width.min(height)) / 2.0;
        prop_assume!(corner_radius <= max_radius);

        let rounded_rect = RoundedRectangle::new(
            width,
            height,
            corner_radius,
            color.clone()
        );

        let color_direct = rounded_rect.get_color();

        let rounded_as_rectangle: CRc<Rectangle> = rounded_rect.clone().into_super();
        let color_via_rectangle = rounded_as_rectangle.get_color();

        let rounded_as_shape: CRc<Shape> = rounded_as_rectangle.clone().into_super();
        let color_via_shape = rounded_as_shape.get_color();

        prop_assert!(
            color_direct == color_via_rectangle,
            "RoundedRectangle color mismatch: direct={:?}, via Rectangle={:?}",
            color_direct,
            color_via_rectangle
        );

        prop_assert!(
            color_direct == color_via_shape,
            "RoundedRectangle color mismatch: direct={:?}, via Shape={:?}",
            color_direct,
            color_via_shape
        );

        prop_assert!(
            *color_direct == Some(color.clone()),
            "RoundedRectangle color should be {:?}, but got {:?}",
            Some(color.clone()),
            color_direct
        );
    }
}
