#[allow(dead_code)]
fn print_area<T: Area>(t: &T) -> f64 {
    t.compute()
}
trait Area {
    fn compute(&self) -> f64;
}
#[allow(dead_code)]
enum Shape {
    Circle(Circle),
    Triangle(Triangle),
    Square(Square),
}
#[derive(Debug)]
struct Circle {
    lenght: f64,
}
#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}
#[derive(Debug)]
struct Square {
    lenght: f64,
}

impl Area for Shape {
    fn compute(&self) -> f64 {
        match self {
            Shape::Circle(e) => e.lenght.powi(2) * 2.0 * 3.14,
            Shape::Triangle(e) => e.length * e.height,
            Shape::Square(l) => l.lenght.powi(2),
        }
    }
}

impl Area for Square {
    fn compute(&self) -> f64 {
        self.lenght.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_bound() {
        let ab = Shape::Circle(Circle { lenght: 1.0 });
        assert_eq!(6.28, print_area(&ab));
        let t = Square { lenght: 2.0 };
        assert_eq!(4.0, print_area(&t));
    }
}
