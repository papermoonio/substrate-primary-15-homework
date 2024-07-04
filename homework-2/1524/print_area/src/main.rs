fn main() {
    println!("Hello, world!");
    let triangle = Triangle {
        bottom: 4.0,
        height: 2.0,
    };
    let area = print_area(triangle);
    println!("Triangle Area: {:?}", area);

    let rectangle = Rectangle {
        width: 4.0,
        length: 2.0,
    };
    let rectangle_area = print_area(rectangle);
    println!("rectangle Area: {:?}", rectangle_area);

    let square = Rectangle {
        width: 4.0,
        length: 4.0,
    };
    let square_area = print_area(square);
    println!("square Area: {:?}", square_area);

    let circle = Circle { radius: 4.0 };
    let circle_area = print_area(circle);
    println!("circle Area: {:?}", circle_area);
    // Hello, world!
    // Triangle Area: 4.0
    // rectangle Area: 8.0
    // square Area: 16.0
    // circle Area: 50.26548245743669
}

trait Shape {
    fn compute_area(&self) -> f64;
}

struct Triangle {
    bottom: f64,
    height: f64,
}

impl Shape for Triangle {
    fn compute_area(&self) -> f64 {
        0.5 * self.bottom * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn compute_area(&self) -> f64 {
        const PI: f64 = std::f64::consts::PI;
        PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    length: f64,
}

impl Shape for Rectangle {
    fn compute_area(&self) -> f64 {
        self.length * self.width
    }
}

fn print_area<T: Shape>(shape: T) -> f64 {
    shape.compute_area()
}
