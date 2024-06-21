// defined Area circle
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// defined tri
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// defined Square
struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// print circle area
fn print_area<T: Area>(shape: T) {
    println!("The area of the shape is: {:.2}", shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle {
            base: 4.0,
            height: 3.0,
        };
        assert_eq!(triangle.area(), 0.5 * 4.0 * 3.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 6.0 };
        assert_eq!(square.area(), 6.0 * 6.0);
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 4.0,
        height: 3.0,
    };
    let square = Square { side: 6.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}
