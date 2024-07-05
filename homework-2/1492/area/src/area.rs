pub trait Area {
    // 求图形面积
    fn area(&self) -> f64;
}

/// 打印图形面积
pub fn print_area<T: Area>(shape: &T) {
    println!("The area is {}", shape.area());
}

#[cfg(test)]
mod tests {
    use crate::{circle::Circle, square::Square, triangle::Triangle};

    use super::*;

    #[test]
    fn test_print_area() {
        let circle = Circle::new(1.0);
        let square = Square::new(3.0);
        let triangle = Triangle::new(3.0, 4.0);

        print_area(&circle);
        print_area(&square);
        print_area(&triangle);
    }
}