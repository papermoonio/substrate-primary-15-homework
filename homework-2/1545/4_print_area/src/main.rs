trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        const PI: f64 = std::f64::consts::PI;
        PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.
    }
}

struct Rect {
    width: f64,
    height: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
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

fn print_area<T: Area>(shape: T) {
    println!("Area of the shape is: {:.2}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2. };
    print_area(circle);

    let triangle = Triangle { base: 2.5, height: 3.3 };
    print_area(triangle);

    let rect = Rect { width: 2.5, height: 1.5 };
    print_area(rect);

    let square = Square { side: 9.5 };
    print_area(square);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle { radius: 2. };
        assert_eq!(circle.area(), std::f64::consts::PI * 2. * 2.);
    }

    #[test]
    fn test_triangle() {
        let triangle = Triangle { base: 2.5, height: 3.3 };
        assert_eq!(triangle.area(), 2.5 * 3.3 / 2.);
    }

    #[test]
    fn test_rect() {
        let rect = Rect { width: 2.5, height: 1.5 };
        assert_eq!(rect.area(), 2.5 * 1.5);
    }

    #[test]
    fn test_square() {
        let square = Square { side: 9.5 };
        assert_eq!(square.area(), 9.5 * 9.5);
    }
}

