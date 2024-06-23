use crate::area::Area;

pub struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle {
    pub fn new(base: f64, height: f64) -> Self {
        Self {
            base,
            height,
        }
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area() {
        let t = Triangle::new(3.0, 4.0);
        assert_eq!(t.area(), 6.0);
    }
}

