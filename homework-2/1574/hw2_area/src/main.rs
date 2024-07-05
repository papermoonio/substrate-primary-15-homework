fn main() {
    let circle = Circle { radius: 1.0 };
    let triangle = Triangle { base: 5.0, height: 20.0 };
    let square = Square { side: 10.0 };

    println!("Circle's area is: {}", circle.area());
    println!("Triangle's area is: {}", triangle.area());
    println!("Square's area is: {}", square.area());
}

trait Shape {
    fn area(&self) -> f64;
}
struct Triangle {
    base: f64,
    height: f64,
}
struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_area() {
        let circle = Circle { radius: 3.0 };
        let triangle = Triangle { base: 5.0, height: 9.0 };
        let square = Square { side: 10.0 };
        println!("Circle's area is: {}", circle.area());
        println!("Triangle's area is: {}", triangle.area());
        println!("Square's area is: {}", square.area());
    }
}
