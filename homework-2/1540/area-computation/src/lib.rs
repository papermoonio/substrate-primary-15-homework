use std::f32::consts::PI;

pub trait CanComputeArea {
    fn area(&self) -> f32;
}

pub fn area<T: CanComputeArea>(shape: T) -> f32 {
    shape.area()
}

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl CanComputeArea for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

pub struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Triangle {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
}

impl CanComputeArea for Triangle {
    fn area(&self) -> f32 {
        let p = (self.a + self.b + self.c) / 2.0;
        let tmp = p * (p - self.a) * (p - self.b) * (p - self.c);
        tmp.sqrt()
    }
}

pub struct Rectangle {
    a: f32,
    b: f32,
}

impl Rectangle {
    pub fn new(a: f32, b: f32) -> Self {
        Self { a, b }
    }
}

impl CanComputeArea for Rectangle {
    fn area(&self) -> f32 {
        self.a * self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_should_works() {
        let circle = Circle::new(2.0);
        println!("circle area:{}", area(circle));

        let triangle = Triangle::new(3.0, 4.0, 5.0);
        println!("triangle area:{}", area(triangle));

        let rectangle = Rectangle::new(5.0, 10.0);
        println!("rectangle area:{}", area(rectangle));
    }
}
