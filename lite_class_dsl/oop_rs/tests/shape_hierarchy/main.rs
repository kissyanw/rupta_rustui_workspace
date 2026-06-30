/// This test suite tests class inheritance-related functionality
mod circle;
mod multilevel;
mod polymorphism;
mod rectangle;
mod shape;
mod triangle;

use oop_rs::prelude::*;

// Re-export main types for convenience
pub use circle::colored_circle::{ColoredCircle, IColoredCircle};
pub use circle::ellipse::{Ellipse, IEllipse};
pub use circle::{Circle, ICircle};
pub use rectangle::rounded_rectangle::{RoundedRectangle, IRoundedRectangle};
pub use rectangle::square::{Square, ISquare};
pub use rectangle::{Rectangle, IRectangle};
pub use shape::{Shape, IShape};
pub use triangle::{Triangle, ITriangle};

/// Test creating all 7 shape instances
#[test]
fn test_create_all_shapes() {
    println!("\n--- Creating 7 Shape Instances ---");

    // 1. Circle
    let circle = Circle::new(5.0, "red".to_string());
    assert_eq!(circle.get().radius(), 5.0);
    assert_eq!(circle.get().color(), &Some("red".to_string()));

    // 2. Rectangle
    let rectangle = Rectangle::new(4.0, 6.0, "blue".to_string());
    assert_eq!(rectangle.get().width(), 4.0);
    assert_eq!(rectangle.get().height(), 6.0);

    // 3. Triangle
    let triangle = Triangle::new(3.0, 4.0, 5.0, "green".to_string());
    assert_eq!(triangle.get().side_a(), 3.0);
    assert_eq!(triangle.get().side_b(), 4.0);
    assert_eq!(triangle.get().side_c(), 5.0);

    // 4. ColoredCircle (second-level derived class)
    let colored_circle = ColoredCircle::new(7.0, "black".to_string(), "yellow".to_string());
    assert_eq!(colored_circle.get().radius(), 7.0);
    assert_eq!(colored_circle.get().color(), &Some("black".to_string()));
    assert_eq!(colored_circle.get().fill_color(), &Some("yellow".to_string()));

    // 5. Ellipse (second-level derived class)
    let ellipse = Ellipse::new(8.0, 5.0, "purple".to_string());
    assert_eq!(ellipse.get().radius(), 8.0);
    assert_eq!(ellipse.get().semi_minor_axis(), 5.0);

    // 6. Square (second-level derived class)
    let square = Square::new(6.0, "orange".to_string());
    assert_eq!(square.get().width(), 6.0);
    assert_eq!(square.get().height(), 6.0);

    // 7. RoundedRectangle (second-level derived class)
    let rounded_rect = RoundedRectangle::new(10.0, 8.0, 2.0, "pink".to_string());
    assert_eq!(rounded_rect.get().width(), 10.0);
    assert_eq!(rounded_rect.get().height(), 8.0);
    assert_eq!(rounded_rect.get().corner_radius(), 2.0);

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
        circle.clone() as CRc<Shape>,
        rectangle.clone() as CRc<Shape>,
        triangle.clone() as CRc<Shape>,
        colored_circle.clone() as CRc<Circle> as CRc<Shape>,
        ellipse.clone() as CRc<Circle> as CRc<Shape>,
        square.clone() as CRc<Rectangle> as CRc<Shape>,
        rounded_rect.clone() as CRc<Rectangle> as CRc<Shape>,
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

    let circle_as_shape: CRc<Shape> = circle_original.clone() as CRc<Shape>;

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

    let rect_as_shape: CRc<Shape> = rect_original.clone() as CRc<Shape>;

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

    let tri_as_shape: CRc<Shape> = tri_original.clone() as CRc<Shape>;

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
    let shape: CRc<Shape> = circle.clone() as CRc<Shape>;

    match shape.downcast_rc::<Circle>() {
        Ok(circle_back) => {
            assert_eq!(circle_back.get().radius(), 10.0);
            assert_eq!(circle_back.get().color(), &Some("cyan".to_string()));
            println!("  ✓ Successfully downcast Shape to Circle");
            println!("  Radius after conversion: {:.2}", circle_back.get().radius());
        }
        Err(_) => {
            panic!("Downcast should succeed");
        }
    }
}

/// Test downcast: Shape -> Rectangle (success)
#[test]
fn test_downcast_shape_to_rectangle_success() {
    println!("\n--- Shape -> Rectangle Downcast (Success) ---");

    let rectangle = Rectangle::new(12.0, 8.0, "magenta".to_string());
    let shape: CRc<Shape> = rectangle.clone() as CRc<Shape>;

    match shape.downcast_rc::<Rectangle>() {
        Ok(rect_back) => {
            assert_eq!(rect_back.get().width(), 12.0);
            assert_eq!(rect_back.get().height(), 8.0);
            println!("  ✓ Successfully downcast Shape to Rectangle");
            println!(
                "  Width after conversion: {:.2}, Height: {:.2}",
                rect_back.get().width(),
                rect_back.get().height()
            );
        }
        Err(_) => {
            panic!("Downcast should succeed");
        }
    }
}

/// Test downcast: Shape -> Triangle (success)
#[test]
fn test_downcast_shape_to_triangle_success() {
    println!("\n--- Shape -> Triangle Downcast (Success) ---");

    let triangle = Triangle::new(5.0, 12.0, 13.0, "brown".to_string());
    let shape: CRc<Shape> = triangle.clone() as CRc<Shape>;

    match shape.downcast_rc::<Triangle>() {
        Ok(tri_back) => {
            assert_eq!(tri_back.get().side_a(), 5.0);
            assert_eq!(tri_back.get().side_b(), 12.0);
            assert_eq!(tri_back.get().side_c(), 13.0);
            println!("  ✓ Successfully downcast Shape to Triangle");
            println!(
                "  Sides after conversion: ({:.2}, {:.2}, {:.2})",
                tri_back.get().side_a(),
                tri_back.get().side_b(),
                tri_back.get().side_c()
            );
        }
        Err(_) => {
            panic!("Downcast should succeed");
        }
    }
}

/// Test incorrect downcast: Shape(Circle) -> Rectangle (failure)
#[test]
fn test_downcast_shape_to_wrong_type_failure() {
    println!("\n--- Incorrect Downcast: Shape(Circle) -> Rectangle (Should Fail) ---");

    let circle = Circle::new(10.0, "cyan".to_string());
    let shape: CRc<Shape> = circle as CRc<Shape>;

    match shape.downcast_rc::<Rectangle>() {
        Ok(_) => {
            panic!("Should not succeed: cannot convert Circle to Rectangle");
        }
        Err(_) => {
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
    let colored_as_circle: CRc<Circle> = colored_original.clone() as CRc<Circle>;
    assert_eq!(colored_as_circle.get().radius(), 15.0);

    // Circle -> Shape
    let colored_as_shape: CRc<Shape> = colored_as_circle.clone() as CRc<Shape>;
    assert!(colored_as_shape.area() > 0.0);

    // Shape -> Circle
    match colored_as_shape.downcast_rc::<Circle>() {
        Ok(circle_back) => {
            assert_eq!(circle_back.get().radius(), 15.0);

            // Circle -> ColoredCircle
            match circle_back.downcast_rc::<ColoredCircle>() {
                Ok(colored_back) => {
                    assert_eq!(colored_back.get().radius(), 15.0);
                    assert_eq!(colored_back.get().color(), &Some("navy".to_string()));
                    assert_eq!(colored_back.get().fill_color(), &Some("gold".to_string()));
                    println!(
                        "  ✓ Multi-level conversion chain complete: ColoredCircle -> Circle -> Shape -> Circle -> ColoredCircle"
                    );
                }
                Err(_) => {
                    panic!("Conversion back to ColoredCircle should succeed");
                }
            }
        }
        Err(_) => {
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
    let square_as_rect: CRc<Rectangle> = square_original.clone() as CRc<Rectangle>;
    assert_eq!(square_as_rect.get().width(), 20.0);
    assert_eq!(square_as_rect.get().height(), 20.0);

    // Rectangle -> Shape
    let square_as_shape: CRc<Shape> = square_as_rect.clone() as CRc<Shape>;
    assert!(square_as_shape.area() > 0.0);

    // Shape -> Rectangle
    match square_as_shape.downcast_rc::<Rectangle>() {
        Ok(rect_back) => {
            assert_eq!(rect_back.get().width(), 20.0);
            assert_eq!(rect_back.get().height(), 20.0);

            // Rectangle -> Square
            match rect_back.downcast_rc::<Square>() {
                Ok(square_back) => {
                    assert_eq!(square_back.get().width(), 20.0);
                    assert_eq!(square_back.get().height(), 20.0);
                    assert!((square_back.get().width() - square_back.get().height()).abs() < 1e-10);
                    println!(
                        "  ✓ Multi-level conversion chain complete: Square -> Rectangle -> Shape -> Rectangle -> Square"
                    );
                }
                Err(_) => {
                    panic!("Conversion back to Square should succeed");
                }
            }
        }
        Err(_) => {
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
    let ellipse_as_circle: CRc<Circle> = ellipse_original.clone() as CRc<Circle>;
    assert_eq!(ellipse_as_circle.get().radius(), 18.0);

    // Circle -> Shape
    let ellipse_as_shape: CRc<Shape> = ellipse_as_circle.clone() as CRc<Shape>;
    assert!(ellipse_as_shape.area() > 0.0);

    // Shape -> Circle
    match ellipse_as_shape.downcast_rc::<Circle>() {
        Ok(circle_back) => {
            assert_eq!(circle_back.get().radius(), 18.0);

            // Circle -> Ellipse
            match circle_back.downcast_rc::<Ellipse>() {
                Ok(ellipse_back) => {
                    assert_eq!(ellipse_back.get().radius(), 18.0);
                    assert_eq!(ellipse_back.get().semi_minor_axis(), 12.0);
                    println!(
                        "  ✓ Multi-level conversion chain complete: Ellipse -> Circle -> Shape -> Circle -> Ellipse"
                    );
                }
                Err(_) => {
                    panic!("Conversion back to Ellipse should succeed");
                }
            }
        }
        Err(_) => {
            panic!("Conversion back to Circle should succeed");
        }
    }
}
