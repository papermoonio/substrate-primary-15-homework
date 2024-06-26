pub trait Shape {
    fn desc(&self) -> String;
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn desc(&self) -> String {
        format!("原型：半径 {}", self.radius)
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Shape for Triangle {
    fn desc(&self) -> String {
        format!("三角形：底 {} 高 {}", self.base, self.height)
    }
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn desc(&self) -> String {
        format!("正方形：边 {}", self.side)
    }
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area() {
        let circle = Circle { radius: 2.0 };
        let want_area = std::f64::consts::PI * 2.0 * 2.0;
        assert!((circle.area() - want_area) < 1e-6)
    }

    #[test]
    fn triangle_area() {
        let circle = Triangle { base: 10.0, height: 5.0 };
        let want_area = 0.5 * 10.0 * 5.0;
        assert!((circle.area() - want_area) < 1e-6)
    }

    #[test]
    fn square_area() {
        let square = Square { side: 4.0 };
        let want_area = 4.0 * 4.0;
        assert!((square.area() - want_area) < 1e-6)
    }
}