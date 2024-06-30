fn main() {
    println!("Hello, area-shape!");
    let circle = Circle { radius: 10.0 };
    let triangle = Triangle { base: 2.0, height: 25.0 };
    let square = Square { side: 10.0 };

    println!("This circle's area is: {}", circle.area());
    println!("This triangle's area is: {}", triangle.area());
    println!("This square's area is: {}", square.area());
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


