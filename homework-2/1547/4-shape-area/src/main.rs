trait Shape {
    fn calculate_area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    bottom: f64,
    height: f64,
}

impl Shape for Triangle {
    fn calculate_area(&self) -> f64 {
        0.5 * self.bottom * self.height
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn calculate_area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("Area : {}", shape.calculate_area())
}

fn main() {
    println!("Hello, 1547!");

    let c = Circle { radius: 3.0 };
    print_area(&c);

    let t = Triangle { bottom: 3.0, height: 5.0 };
    print_area(&t);

    let s = Square { side: 2.0 };
    print_area(&s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let c = Circle { radius: 3.0 };
        assert_eq!(c.calculate_area(), std::f64::consts::PI * 9.0);
    }

    #[test]
    fn test_triangle_area() {
        let t = Triangle { bottom: 3.0, height: 5.0 };
        assert_eq!(t.calculate_area(), 7.5);
    }

    #[test]
    fn test_square_area() {
        let s = Square { side: 2.0 };
        assert_eq!(s.calculate_area(), 4.0);
    }

    #[test]
    fn test_print_area() {
        let circle = Circle { radius: 3.0 };
        let triangle = Triangle { bottom: 3.0, height: 5.0 };
        let square = Square { side: 2.0 };

        print_area(&circle);  // Expect: "The area is: <area>"
        print_area(&triangle); // Expect: "The area is: <area>"
        print_area(&square);  // Expect: "The area is: <area>"
    }
}
