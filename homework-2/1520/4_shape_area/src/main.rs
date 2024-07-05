use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("The area is: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 4.0, height: 5.0 };
    let square = Square { side: 2.5 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 3.0 };
        assert!((circle.area() - 28.274).abs() < 1e-3);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 4.0, height: 5.0 };
        assert_eq!(triangle.area(), 10.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 2.5 };
        assert_eq!(square.area(), 6.25);
    }
}
