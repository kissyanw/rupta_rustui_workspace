/// This test suite tests class inheritance-related functionality
mod circle;
mod multilevel;
mod polymorphism;
mod rectangle;
mod shape;
mod triangle;

// Re-export main types for convenience
use circle::Circle;
use circle::colored_circle::ColoredCircle;
use circle::ellipse::Ellipse;
use classes::prelude::*;
use rectangle::Rectangle;
use rectangle::rounded_rectangle::RoundedRectangle;
use rectangle::square::Square;
use shape::Shape;
use triangle::Triangle;

/// Test creating all 7 shape instances
#[test]
fn test_create_all_shapes() {
    println!("\n--- Creating 7 Shape Instances ---");

    // 1. Circle
    let circle = Circle::new(5.0, "red".to_string());
    assert_eq!(circle.get_radius(), 5.0);
    assert_eq!(circle.get_color(), &Some("red".to_string()));

    // 2. Rectangle
    let rectangle = Rectangle::new(4.0, 6.0, "blue".to_string());
    assert_eq!(rectangle.get_width(), 4.0);
    assert_eq!(rectangle.get_height(), 6.0);

    // 3. Triangle
    let triangle = Triangle::new(3.0, 4.0, 5.0, "green".to_string());
    assert_eq!(triangle.get_side_a(), 3.0);
    assert_eq!(triangle.get_side_b(), 4.0);
    assert_eq!(triangle.get_side_c(), 5.0);

    // 4. ColoredCircle (second-level derived class)
    let colored_circle = ColoredCircle::new(7.0, "black".to_string(), "yellow".to_string());
    assert_eq!(colored_circle.get_radius(), 7.0);
    assert_eq!(colored_circle.get_color(), &Some("black".to_string()));
    assert_eq!(colored_circle.get_fill_color(), &Some("yellow".to_string()));

    // 5. Ellipse (second-level derived class)
    let ellipse = Ellipse::new(8.0, 5.0, "purple".to_string());
    assert_eq!(ellipse.get_radius(), 8.0);
    assert_eq!(ellipse.get_semi_minor_axis(), 5.0);

    // 6. Square (second-level derived class)
    let square = Square::new(6.0, "orange".to_string());
    assert_eq!(square.get_width(), 6.0);
    assert_eq!(square.get_height(), 6.0);

    // 7. RoundedRectangle (second-level derived class)
    let rounded_rect = RoundedRectangle::new(10.0, 8.0, 2.0, "pink".to_string());
    assert_eq!(rounded_rect.get_width(), 10.0);
    assert_eq!(rounded_rect.get_height(), 8.0);
    assert_eq!(rounded_rect.get_corner_radius(), 2.0);

    println!("✓ All 7 shapes created successfully");
}

/// Test polymorphic behavior: store all shapes in Vec<CRc<Shape>> and iterate
#[test]
fn test_polymorphism_with_collection() {
    println!("\n--- Testing Polymorphic Behavior ---");

    let circle = Circle::new(5.0, "red".to_string());
    let rectangle = Rectangle::new(4.0, 6.0, "blue".to_string());
    let triangle = Triangle::new(3.0, 4.0, 5.0, "green".to_string());
    let colored_circle = ColoredCircle::new(7.0, "black".to_string(), "yellow".to_string());
    let ellipse = Ellipse::new(8.0, 5.0, "purple".to_string());
    let square = Square::new(6.0, "orange".to_string());
    let rounded_rect = RoundedRectangle::new(10.0, 8.0, 2.0, "pink".to_string());

    // Store all shapes in Vec<CRc<Shape>>
    let shapes: Vec<CRc<Shape>> = vec![
        circle.clone().into_super(),
        rectangle.clone().into_super(),
        triangle.clone().into_super(),
        colored_circle.clone().into_super().into_super(), // ColoredCircle -> Circle -> Shape
        ellipse.clone().into_super().into_super(),        // Ellipse -> Circle -> Shape
        square.clone().into_super().into_super(),         // Square -> Rectangle -> Shape
        rounded_rect.clone().into_super().into_super(),   // RoundedRectangle -> Rectangle -> Shape
    ];

    assert_eq!(shapes.len(), 7);

    // Iterate through the collection, verify each shape can correctly call its own methods
    for (index, shape) in shapes.iter().enumerate() {
        let area = shape.area();
        let perimeter = shape.perimeter();
        let description = shape.description();

        assert!(
            area > 0.0,
            "Shape {} area should be greater than 0",
            index + 1
        );
        assert!(
            perimeter > 0.0,
            "Shape {} perimeter should be greater than 0",
            index + 1
        );
        assert!(
            !description.is_empty(),
            "Shape {} description should not be empty",
            index + 1
        );

        println!(
            "{:<5} {:<40} {:<15.2} {:<15.2}",
            index + 1,
            description,
            area,
            perimeter
        );
    }

    println!("✓ Polymorphic behavior verified successfully!");
}

/// Test upcast: Circle -> Shape
#[test]
fn test_upcast_circle_to_shape() {
    println!("\n--- Circle -> Shape Upcast ---");

    let circle_original = Circle::new(10.0, "cyan".to_string());
    let original_area = circle_original.area();
    let original_desc = circle_original.description();

    let circle_as_shape: CRc<Shape> = circle_original.clone().into_super();

    // Verify polymorphic behavior: methods called through Shape reference should return the same results
    assert_eq!(circle_as_shape.area(), original_area);
    assert_eq!(circle_as_shape.description(), original_desc);

    println!("  Original Circle area: {:.2}", original_area);
    println!(
        "  Area after converting to Shape: {:.2}",
        circle_as_shape.area()
    );
    println!("  ✓ Upcast successful, polymorphic behavior normal");
}

/// Test upcast: Rectangle -> Shape
#[test]
fn test_upcast_rectangle_to_shape() {
    println!("\n--- Rectangle -> Shape Upcast ---");

    let rect_original = Rectangle::new(12.0, 8.0, "magenta".to_string());
    let original_area = rect_original.area();
    let original_desc = rect_original.description();

    let rect_as_shape: CRc<Shape> = rect_original.clone().into_super();

    assert_eq!(rect_as_shape.area(), original_area);
    assert_eq!(rect_as_shape.description(), original_desc);

    println!("  Original Rectangle area: {:.2}", original_area);
    println!(
        "  Area after converting to Shape: {:.2}",
        rect_as_shape.area()
    );
    println!("  ✓ Upcast successful, polymorphic behavior normal");
}

/// Test upcast: Triangle -> Shape
#[test]
fn test_upcast_triangle_to_shape() {
    println!("\n--- Triangle -> Shape Upcast ---");

    let tri_original = Triangle::new(5.0, 12.0, 13.0, "brown".to_string());
    let original_area = tri_original.area();
    let original_desc = tri_original.description();

    let tri_as_shape: CRc<Shape> = tri_original.clone().into_super();

    assert_eq!(tri_as_shape.area(), original_area);
    assert_eq!(tri_as_shape.description(), original_desc);

    println!("  Original Triangle area: {:.2}", original_area);
    println!(
        "  Area after converting to Shape: {:.2}",
        tri_as_shape.area()
    );
    println!("  ✓ Upcast successful, polymorphic behavior normal");
}

/// Test downcast: Shape -> Circle (success)
#[test]
fn test_downcast_shape_to_circle_success() {
    println!("\n--- Shape -> Circle Downcast (Success) ---");

    let circle = Circle::new(10.0, "cyan".to_string());
    let shape: CRc<Shape> = circle.clone().into_super();

    match shape.try_into_subtype::<CRc<Circle>>() {
        Some(circle_back) => {
            assert_eq!(circle_back.get_radius(), 10.0);
            assert_eq!(circle_back.get_color(), &Some("cyan".to_string()));
            println!("  ✓ Successfully downcast Shape to Circle");
            println!("  Radius after conversion: {:.2}", circle_back.get_radius());
        }
        None => {
            panic!("Downcast should succeed");
        }
    }
}

/// Test downcast: Shape -> Rectangle (success)
#[test]
fn test_downcast_shape_to_rectangle_success() {
    println!("\n--- Shape -> Rectangle Downcast (Success) ---");

    let rectangle = Rectangle::new(12.0, 8.0, "magenta".to_string());
    let shape: CRc<Shape> = rectangle.clone().into_super();

    match shape.try_into_subtype::<CRc<Rectangle>>() {
        Some(rect_back) => {
            assert_eq!(rect_back.get_width(), 12.0);
            assert_eq!(rect_back.get_height(), 8.0);
            println!("  ✓ Successfully downcast Shape to Rectangle");
            println!(
                "  Width after conversion: {:.2}, Height: {:.2}",
                rect_back.get_width(),
                rect_back.get_height()
            );
        }
        None => {
            panic!("Downcast should succeed");
        }
    }
}

/// Test downcast: Shape -> Triangle (success)
#[test]
fn test_downcast_shape_to_triangle_success() {
    println!("\n--- Shape -> Triangle Downcast (Success) ---");

    let triangle = Triangle::new(5.0, 12.0, 13.0, "brown".to_string());
    let shape: CRc<Shape> = triangle.clone().into_super();

    match shape.try_into_subtype::<CRc<Triangle>>() {
        Some(tri_back) => {
            assert_eq!(tri_back.get_side_a(), 5.0);
            assert_eq!(tri_back.get_side_b(), 12.0);
            assert_eq!(tri_back.get_side_c(), 13.0);
            println!("  ✓ Successfully downcast Shape to Triangle");
            println!(
                "  Sides after conversion: ({:.2}, {:.2}, {:.2})",
                tri_back.get_side_a(),
                tri_back.get_side_b(),
                tri_back.get_side_c()
            );
        }
        None => {
            panic!("Downcast should succeed");
        }
    }
}

/// Test incorrect downcast: Shape(Circle) -> Rectangle (failure)
#[test]
fn test_downcast_shape_to_wrong_type_failure() {
    println!("\n--- Incorrect Downcast: Shape(Circle) -> Rectangle (Should Fail) ---");

    let circle = Circle::new(10.0, "cyan".to_string());
    let shape: CRc<Shape> = circle.into_super();

    match shape.try_into_subtype::<CRc<Rectangle>>() {
        Some(_) => {
            panic!("Should not succeed: cannot convert Circle to Rectangle");
        }
        None => {
            println!("  ✓ Correctly failed: cannot convert Circle to Rectangle");
            println!("  Type system correctly prevented incorrect type conversion");
        }
    }
}

/// Test multi-level conversion chain: ColoredCircle -> Circle -> Shape -> Circle -> ColoredCircle
#[test]
fn test_multilevel_conversion_colored_circle() {
    println!("\n--- ColoredCircle Multi-level Conversion Chain ---");

    let colored_original = ColoredCircle::new(15.0, "navy".to_string(), "gold".to_string());

    // ColoredCircle -> Circle
    let colored_as_circle: CRc<Circle> = colored_original.clone().into_super();
    assert_eq!(colored_as_circle.get_radius(), 15.0);

    // Circle -> Shape
    let colored_as_shape: CRc<Shape> = colored_as_circle.clone().into_super();
    assert!(colored_as_shape.area() > 0.0);

    // Shape -> Circle
    match colored_as_shape.try_into_subtype::<CRc<Circle>>() {
        Some(circle_back) => {
            assert_eq!(circle_back.get_radius(), 15.0);

            // Circle -> ColoredCircle
            match circle_back.try_into_subtype::<CRc<ColoredCircle>>() {
                Some(colored_back) => {
                    assert_eq!(colored_back.get_radius(), 15.0);
                    assert_eq!(colored_back.get_color(), &Some("navy".to_string()));
                    assert_eq!(colored_back.get_fill_color(), &Some("gold".to_string()));
                    println!(
                        "  ✓ Multi-level conversion chain complete: ColoredCircle -> Circle -> Shape -> Circle -> ColoredCircle"
                    );
                }
                None => {
                    panic!("Conversion back to ColoredCircle should succeed");
                }
            }
        }
        None => {
            panic!("Conversion back to Circle should succeed");
        }
    }
}

/// Test multi-level conversion chain: Square -> Rectangle -> Shape -> Rectangle -> Square
#[test]
fn test_multilevel_conversion_square() {
    println!("\n--- Square Multi-level Conversion Chain ---");

    let square_original = Square::new(20.0, "silver".to_string());

    // Square -> Rectangle
    let square_as_rect: CRc<Rectangle> = square_original.clone().into_super();
    assert_eq!(square_as_rect.get_width(), 20.0);
    assert_eq!(square_as_rect.get_height(), 20.0);

    // Rectangle -> Shape
    let square_as_shape: CRc<Shape> = square_as_rect.clone().into_super();
    assert!(square_as_shape.area() > 0.0);

    // Shape -> Rectangle
    match square_as_shape.try_into_subtype::<CRc<Rectangle>>() {
        Some(rect_back) => {
            assert_eq!(rect_back.get_width(), 20.0);
            assert_eq!(rect_back.get_height(), 20.0);

            // Rectangle -> Square
            match rect_back.try_into_subtype::<CRc<Square>>() {
                Some(square_back) => {
                    assert_eq!(square_back.get_width(), 20.0);
                    assert_eq!(square_back.get_height(), 20.0);
                    assert!((square_back.get_width() - square_back.get_height()).abs() < 1e-10);
                    println!(
                        "  ✓ Multi-level conversion chain complete: Square -> Rectangle -> Shape -> Rectangle -> Square"
                    );
                }
                None => {
                    panic!("Conversion back to Square should succeed");
                }
            }
        }
        None => {
            panic!("Conversion back to Rectangle should succeed");
        }
    }
}

/// Test multi-level conversion chain: Ellipse -> Circle -> Shape -> Circle -> Ellipse
#[test]
fn test_multilevel_conversion_ellipse() {
    println!("\n--- Ellipse Multi-level Conversion Chain ---");

    let ellipse_original = Ellipse::new(18.0, 12.0, "teal".to_string());

    // Ellipse -> Circle
    let ellipse_as_circle: CRc<Circle> = ellipse_original.clone().into_super();
    assert_eq!(ellipse_as_circle.get_radius(), 18.0);

    // Circle -> Shape
    let ellipse_as_shape: CRc<Shape> = ellipse_as_circle.clone().into_super();
    assert!(ellipse_as_shape.area() > 0.0);

    // Shape -> Circle
    match ellipse_as_shape.try_into_subtype::<CRc<Circle>>() {
        Some(circle_back) => {
            assert_eq!(circle_back.get_radius(), 18.0);

            // Circle -> Ellipse
            match circle_back.try_into_subtype::<CRc<Ellipse>>() {
                Some(ellipse_back) => {
                    assert_eq!(ellipse_back.get_radius(), 18.0);
                    assert_eq!(ellipse_back.get_semi_minor_axis(), 12.0);
                    println!(
                        "  ✓ Multi-level conversion chain complete: Ellipse -> Circle -> Shape -> Circle -> Ellipse"
                    );
                }
                None => {
                    panic!("Conversion back to Ellipse should succeed");
                }
            }
        }
        None => {
            panic!("Conversion back to Circle should succeed");
        }
    }
}
