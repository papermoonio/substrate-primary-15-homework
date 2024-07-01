pub trait Shape<T> {
    fn area(&self) -> T;
}

pub struct Circle<T> {
    radius: T,
}

impl<T: Into<f64> + Clone > Circle<T> {
    pub fn new(radius: T) -> Self {
        Circle { radius }
    }
}

impl<T: Into<f64> + Clone > Shape<f64> for Circle<T> {
    fn area(&self) -> f64 {
        let radius: f64 = self.radius.clone().into();
        std::f64::consts::PI * radius * radius
    }
}

pub struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: Into<f64> + Clone > Rectangle<T> {
    pub fn new(width: T, height: T) -> Self {
        Rectangle { width, height }
    }
}

impl<T: Into<f64> + Clone > Shape<f64> for Rectangle<T> {
    fn area(&self) -> f64 {
        let width: f64 = self.width.clone().into();
        let height: f64 = self.height.clone().into();
        width * height
    }
}