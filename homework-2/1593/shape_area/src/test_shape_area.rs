use crate::shape_area::{Rectangle, Triangle, Circle};
use crate::shape_area::Shape;

#[test]
fn test_rectangle_with_integer() {
    let rectangle = Rectangle { width: 3, height: 2 };
    assert_eq!(rectangle.area(), 6.0);
}

#[test]
fn test_rectangle_with_float() {
    let rectangle = Rectangle { width: 3.0, height: 2.0 };
    assert_eq!(rectangle.area(), 6.0);
}

#[test]
fn test_triangle_with_integer() {
    let triangle = Triangle { base: 3, height: 2 };
    assert_eq!(triangle.area(), 3.0);
}

#[test]
fn test_triangle_with_float() {
    let triangle = Triangle { base: 3.0, height: 2.0 };
    assert_eq!(triangle.area(), 3.0);
}

#[test]
fn test_circle_with_integer() {
    let circle = Circle { radius: 5 };
    assert_eq!(circle.area(), 78.54);
}

#[test]
fn test_circle_with_float() {
    let circle = Circle { radius: 5.0 };
    assert_eq!(circle.area(), 78.54);
}