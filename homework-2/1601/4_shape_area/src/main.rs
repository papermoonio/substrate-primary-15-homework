trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("This shape's area is: {}", shape.area());
}

fn main() {
    println!("My id is 1601");
    let circle = Circle { radius: 9.9 };
    let triangle = Triangle { base: 1.0, height: 15.0 };
    let square = Square { side: 8.0 };

    print_area(circle);  
    print_area(triangle);  
    print_area(square);   
}

// main results:
/*
This shape's area is: 307.90749597833565
This shape's area is: 7.5
This shape's area is: 64
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 6.0 };
        assert_eq!(circle.area(), std::f64::consts::PI * 36.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 5.0, height: 4.0 };
        assert_eq!(triangle.area(), 10.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 8.0 };
        assert_eq!(square.area(), 64.0);
    }

    #[test]
    fn test_print_area() {
        let mut cmd = Command::cargo_bin("shape_area").unwrap();
        cmd.assert().stdout(predicate::str::contains("This shape's area is:"));
    }
}

// test results:
/*
running 4 tests
test tests::test_circle_area ... ok
test tests::test_square_area ... ok
test tests::test_triangle_area ... ok
test tests::test_print_area ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
*/