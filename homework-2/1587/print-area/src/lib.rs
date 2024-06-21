use std::fmt::{Debug, Display};
pub trait Area {
    type AreaTy: Display;
    fn area(&self) -> Self::AreaTy;
}

pub fn area<T: Area + Debug>(shape: T) {
    println!("The area of the shape: {:?} is {:.4}", shape, shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[derive(Debug)]
    struct Rectangle {
        width: usize,
        height: usize,
    }

    impl Area for Rectangle {
        type AreaTy = usize;
        fn area(&self) -> usize {
            self.width * self.height
        }
    }

    #[derive(Debug)]
    struct Circle {
        radius: usize,
    }

    impl Area for Circle {
        type AreaTy = f64;
        fn area(&self) -> f64 {
            PI * (self.radius as f64) * (self.radius as f64)
        }
    }

    #[derive(Debug)]
    struct TriAngle {
        a: usize,
        b: usize,
        c: usize,
    }

    impl Area for TriAngle {
        type AreaTy = f64;
        fn area(&self) -> f64 {
            let s = (self.a + self.b + self.c) as f64 / 2.0;
            (s * (s - self.a as f64) * (s - self.b as f64) * (s - self.c as f64)).sqrt()
        }
    }

    #[test]
    fn it_works() {
        let rectangle = Rectangle {
            width: 10,
            height: 20,
        };
        area(rectangle);

        let circle = Circle { radius: 10 };
        area(circle);

        let triangle = TriAngle { a: 3, b: 3, c: 3 };
        area(triangle);
    }
}
