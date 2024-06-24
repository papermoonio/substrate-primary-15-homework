use crate::area::Area;

pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Square {
        Square { side }
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_area() {
        let s = Square::new(3.0);
        assert_eq!(s.area(), 9.0);
    }
}