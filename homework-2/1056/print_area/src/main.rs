trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let p = (self.a + self.b + self.c) * 0.5;
        let z = p * (p - self.a) * (p - self.b) * (p -self.c);
        z.sqrt()
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
    println!("Area is: {:.2}", shape.area());
}

fn main() {
    let circle = Circle { radius: 1.0 };
    print_area(circle);
    let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0};
    print_area(triangle);
    let square = Square { side:5.0 };
    print_area(square);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area(){
        let circle = Circle { radius: 1.0 };
        assert_eq!(circle.area(), 3.14);
        let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0};
        assert_eq!(triangle.area(), 6.00);
        let square = Square { side: 5.0 };
        assert_eq!(square.area(), 25.00);
    }
}

