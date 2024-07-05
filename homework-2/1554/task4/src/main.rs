trait Shape {
    fn area(&self) -> f64;
}

// 圆形
#[derive(Debug, Copy, Clone)]
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

// 长方形
#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 三角形
#[derive(Debug, Copy, Clone)]
struct Triangle {
    base: f64,
    height: f64,
}
impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}


fn get_area<T: Shape>(shape: T) -> f64 {
    println!("the area is: {:.2}", shape.area());
    shape.area()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle { radius: 2. };
        println!("shape is : {:?}", circle);
        assert_eq!(circle.area(), 3.14 * 2. * 2.);
    }

    #[test]
    fn test_triangle() {
        let triangle = Triangle { base: 2.5, height: 3.3 };
        println!("shape is : {:?}", triangle);
        assert_eq!(triangle.area(), 2.5 * 3.3 / 2.);
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle { width: 2.5, height: 1.5 };
        println!("shape is : {:?}", rect);
        assert_eq!(rect.area(), 2.5 * 1.5);
    }

    #[test]
    fn test_square() {
        let square = Rectangle { width: 2.0, height: 2.0 };
        println!("shape is : {:?}", square);
        assert_eq!(square.area(), 2.0 * 2.0);
    }
}


fn main() {
    println!("Hello, world!");
}
