// 定义一个泛型函数，接收实现了 Display 和 AreaCalculator trait 的类型参数
fn print_area<T: AreaCalculator>(shape: T) {
    println!("The area of the shape is: {}", shape.calculate_area());
}

// 定义一个 AreaCalculator trait，包含计算面积的方法
trait AreaCalculator {
    fn calculate_area(&self) -> f64;
}

// 实现 AreaCalculator trait 的 Circle 结构体
struct Circle {
    radius: f64,
}

impl AreaCalculator for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 实现 AreaCalculator trait 的 Triangle 结构体
struct Triangle {
    base: f64,
    height: f64,
}

impl AreaCalculator for Triangle {
    fn calculate_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现 AreaCalculator trait 的 Square 结构体
struct Square {
    side: f64,
}

impl AreaCalculator for Square {
    fn calculate_area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    print_area(circle);

    let triangle = Triangle {
        base: 4.0,
        height: 3.0,
    };
    print_area(triangle);

    let square = Square { side: 6.0 };
    print_area(square);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_area_circle() {
        let circle = Circle { radius: 5.0 };
        let expected_area = std::f64::consts::PI * 5.0 * 5.0;

        assert_eq!(circle.calculate_area(), expected_area);
    }

    #[test]
    fn test_print_area_triangle() {
        let triangle = Triangle {
            base: 4.0,
            height: 3.0,
        };
        let expected_area = 0.5 * 4.0 * 3.0;
        assert_eq!(triangle.calculate_area(), expected_area);
    }

    #[test]
    fn test_print_area_square() {
        let square = Square { side: 6.0 };
        let expected_area = 6.0 * 6.0;
        assert_eq!(square.calculate_area(), expected_area);
    }
}
