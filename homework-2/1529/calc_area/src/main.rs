use calc_area::{Shape, Circle, Square, Triangle};

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let square = Square { side: 2.0 };

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        Box::new(triangle),
        Box::new(square),
    ];

    for shape in shapes {
        println!("{} - 面积：{}", shape.desc(), shape.area());
    } 
}
