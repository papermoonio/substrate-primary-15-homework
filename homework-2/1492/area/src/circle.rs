use crate::area::Area;

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let c = Circle::new(1.0);
        assert_eq!(c.area(), std::f64::consts::PI);
    }
}