use std::ops::Mul;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Rectangle<T, K>
where
    T: Mul<Output = T> + Copy + Into<f64>,
    K: Mul<Output = K> + Copy + Into<f64>,
{
    pub width: T,
    pub height: K,
}

impl<T, K> Shape for Rectangle<T, K>
where
    T: Mul<Output = T> + Copy + Into<f64>,
    K: Mul<Output = K> + Copy + Into<f64>,
{
    fn area(&self) -> f64 {
        self.width.into() * self.height.into()
    }
}

pub struct Triangle<T, K>
where
    T: Mul<Output = T> + Copy + Into<f64>,
    K: Mul<Output = K> + Copy + Into<f64>,
{
    pub base: T,
    pub height: K,
}

impl<T, K> Shape for Triangle<T, K>
where
    T: Mul<Output = T> + Copy + Into<f64>,
    K: Mul<Output = K> + Copy + Into<f64>,
{
    fn area(&self) -> f64 {
        (self.base.into() * self.height.into() / 2.0 * 100.0).round() / 100.0
    }
}

pub struct Circle<T>
where
    T: Mul<Output = T> + Copy + Into<f64>,
{
    pub radius: T,
}

impl<T> Shape for Circle<T>
where
    T: Mul<Output = T> + Copy + Into<f64>,
{
    fn area(&self) -> f64 {
        (self.radius.into() * self.radius.into() * std::f64::consts::PI * 100.0).round() / 100.0
    }
}   

pub fn print_area<T>(shape: T)
where
    T: Shape,
{
    println!("Area: {:.2?}", shape.area());
}
