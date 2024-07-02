/*第四题：实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。
*/

trait Shape {
    fn area(&self) -> f64;
}

//圆形
struct Circle {
    radius: f64,
}

//三角形
struct Triangle {
    bottom: f64,
    height: f64,
}

//正方形
struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.bottom * self.height
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_shape_area<T: Shape>(shape: &T) {
    println!("Area : {}", shape.area())
}

fn main() {

    let x = Circle { radius: 5.0 };
    print_shape_area(&x);

    let y = Triangle { bottom: 5.0, height: 5.0 };
    print_shape_area(&y);

    let z = Square { side: 5.0 };
    print_shape_area(&z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area() {
        let c = Circle { radius: 5.0 };
        assert_eq!(c.area(), std::f64::consts::PI * 25.0);
    }

    #[test]
    fn triangle_area() {
        let t = Triangle { bottom: 5.0, height: 5.0 };
        assert_eq!(t.area(), 12.5);
    }

    #[test]
    fn square_area() {
        let s = Square { side: 5.0 };
        assert_eq!(s.area(), 25.0);
    }

}
