trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14159265358979323846264338327950288 * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_graphic_area<T: Area>(graphic: T) {
    println!("The area of the graphic is: {}", graphic.area());
}

fn main() {
    let circle = Circle { radius: 10.0 };
    let triangle = Triangle {
        base: 6.0,
        height: 10.0,
    };
    let square = Square { side: 5.0 };
    print_graphic_area(circle);
    print_graphic_area(triangle);
    print_graphic_area(square);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 10.0 };
        assert_eq!(circle.area(), 314.1592653589793);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle {
            base: 6.0,
            height: 10.0,
        };
        assert_eq!(triangle.area(), 30.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 5.0 };
        assert_eq!(square.area(), 25.0);
    }
}
