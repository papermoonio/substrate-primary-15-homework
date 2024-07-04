use std::f64::consts::PI;

// 定义 Area trait
trait Area {
    fn area(&self) -> f64;
}

// 定义不同的形状
struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Square {
    side: f64,
}

// 为不同形状实现 Area trait
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 泛型函数，打印任何实现了 Area trait 的类型的面积
fn print_area<T: Area>(shape: &T) {
    println!("The area is: {:.2}", shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.area(), 78.53981633974483);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 4.0, height: 3.0 };
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 5.0 };
        assert_eq!(square.area(), 25.0);
    }

    #[test]
    fn test_print_area() {
        let circle = Circle { radius: 5.0 };
        let triangle = Triangle { base: 4.0, height: 3.0 };
        let square = Square { side: 5.0 };

        // 使用 assert_eq! 来测试 print_area 函数
        // 由于 print_area 函数打印到标准输出，我们不能直接测试其输出
        // 但我们可以测试它是否能正确调用不同类型的 area 方法
        assert_eq!(circle.area(), 78.53981633974483);
        assert_eq!(triangle.area(), 6.0);
        assert_eq!(square.area(), 25.0);

        // 调用 print_area 函数来确保它能编译和运行
        print_area(&circle);
        print_area(&triangle);
        print_area(&square);
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 4.0, height: 3.0 };
    let square = Square { side: 5.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}