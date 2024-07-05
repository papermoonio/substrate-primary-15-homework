fn main() {
    let circle = Circle { radius: 7.0 };
    let triangle = Triangle { base: 9.0, height: 3.0 };
    let square = Square { side_length: 5.0 };
    print_area(circle);
    print_area(triangle);
    print_area(square);
}
// 定义一个 trait 来计算面积
trait Area {
    fn area(&self) -> f64;
}

// 实现圆形的 Area trait
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 实现三角形的 Area trait
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现正方形的 Area trait
struct Square {
    side_length: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
}

// 打印图形面积的函数
fn print_area<T: Area>(shape: T) {
    println!("The area of the shape is: {:.2}", shape.area());
}

// 测试打印图形面积函数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_area() {
        let circle = Circle { radius: 7.0 };
        let triangle = Triangle { base: 3.0, height: 9.0 };
        let square = Square { side_length: 2.0 };
        print_area(circle);
        print_area(triangle);
        print_area(square);
    }
}
