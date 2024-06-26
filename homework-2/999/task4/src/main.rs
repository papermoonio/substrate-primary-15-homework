trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    high: f64,
}

struct Square {
    side: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 *  self.base * self.high
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Area>(shape: &T) {
    println!("the area is: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 3.0, high: 4.0 };
    let square = Square { side: 2.0 };

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
        assert!((circle.area() - 28.274333882308138).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 3.0, high: 4.0 };
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 2.0 };
        assert_eq!(square.area(), 4.0);
    }

    #[test]
    fn test_large_circle_area() {
        let circle = Circle { radius: 1e6 };
        assert!((circle.area() - std::f64::consts::PI * 1e12).abs() < 1e-5);
    }

    #[test]
    fn test_large_triangle_area() {
        let triangle = Triangle { base: 1e6, high: 2e6 };
        assert_eq!(triangle.area(), 1e6 * 1e6);
    }

    #[test]
    fn test_large_square_area() {
        let square = Square { side: 1e6 };
        assert_eq!(square.area(), 1e12);
    }
}