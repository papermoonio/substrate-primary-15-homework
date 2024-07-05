trait Area {
    fn area(&self) -> f32;
    fn figure_type(&self) -> String;
}


struct Cycle {
    r: f32,
}

struct Triangle {
    h: f32,
    w: f32,
}

struct Rectangle {
    h: f32,
    w: f32,
}

impl Area for Cycle {
    fn area(&self) -> f32 {
        self.r * self.r * std::f32::consts::PI
    }

    fn figure_type(&self) -> String {
        String::from("Cycle")
    }
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        self.h * self.w / 2.0
    }

    fn figure_type(&self) -> String {
        String::from("Triangle")
    }
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.h * self.w 
    }

    fn figure_type(&self) -> String {
        if self.h == self.w {
            String::from("square")
        } else {
            String::from("Rectangle")
        }
    }
}


fn print_area(figure: impl Area) {
    println!("this figure type is: {}", figure.figure_type() );
    println!("area is {}", figure.area());
}


fn main() {
    let c = Cycle { r: 2.0 };
    print_area(c);
    let r = Rectangle { h: 10.0, w: 2.0 };
    print_area(r);
    let s = Rectangle { h: 5.0, w: 5.0 };
    print_area(s);
    let t = Triangle { h: 10.0, w: 2.0 };
    print_area(t);
}
