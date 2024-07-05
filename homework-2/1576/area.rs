use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64
}

pub struct Square {
    side: f64
}

pub struct Circle {
    radius: f64
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * (self.radius.powi(2))
    }
}

fn calculate_area<T: Shape>(shape: T) -> f64 {
    shape.area()
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::area::{calculate_area, Circle, Rectangle, Square};

    #[test]
    fn test_area() {
        let rectangle = Rectangle{ width: 4.0, height: 2.0 };
        let square = Square { side: 2.0 };
        let circle = Circle { radius: 1.0 };

        assert_eq!(calculate_area(rectangle), 8.0);
        assert_eq!(calculate_area(square), 4.0);
        assert_eq!(calculate_area(circle), PI);
    }
}