
trait Area{
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle{
    radius: f64,
}

impl Area for Circle{
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

#[derive(Debug)]
struct Triangle{
    a: f64,
    b: f64,
    c: f64,
}

impl Area for Triangle{
    fn area(&self) -> f64 {
        self.a * self.b * 0.5
    }
}

#[derive(Debug)]
struct Rectangle{
    width: f64,
    height: f64,
}

impl Area for Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn calculate_area<T: Area>(shape: &T) {
    println!("计算的面积是：{}",shape.area())
}

fn main() {
    let circle = Circle{radius: 5.0};
    let triangle = Triangle{a: 3.0, b: 4.0, c: 5.0};
    let rectangle = Rectangle{width: 10.0, height: 20.0};
    calculate_area(&circle);
    calculate_area(&triangle);
    calculate_area(&rectangle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.area(), std::f64::consts::PI * 5.0 * 5.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0 };
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle { width: 10.0, height: 20.0 };
        assert_eq!(rectangle.area(), 200.0);
    }
}