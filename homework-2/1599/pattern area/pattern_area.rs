// 定义一个可以计算面积的trait
pub trait HasArea {
    fn area(&self) -> f64;
}

// 定义圆形结构体
pub struct Circle {
    radius: f64,
}

// 为圆形实现HasArea trait
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义正方形结构体
pub struct Square {
    side: f64,
}

// 为正方形实现HasArea trait
impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 定义打印面积的函数，接收一个实现了HasArea trait的类型
pub fn print_area<T: HasArea>(shape: T) {
    println!("面积为: {}", shape.area());
}

fn main() {
    //test
    let circle = Circle { radius: 1.0 };
    let square = Square { side: 2.0 };

    print_area(circle);
    print_area(square);
}
