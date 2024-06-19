// Define a trait Shape that defines a method to compute the area of the shape
trait Shape {
    fn area(&self) -> f64;
}

// Implement the Circle struct and implement the Shape trait for it
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Implement the Triangle struct and implement the Shape trait for it
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// Implement the Square struct and implement the Shape trait for it
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// Generic function print_area that accepts any type T that implements the Shape trait
fn print_area<T: Shape>(shape: T) {
    println!("The area of the shape is: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 4.0, height: 3.0 };
    let square = Square { side: 6.0 };

    print_area(circle);  
    print_area(triangle);  
    print_area(square);   
}

// cargo run result:
// The area of the shape is: 78.53981633974483
// The area of the shape is: 6
// The area of the shape is: 36

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_circle_area() {
        // Test the area method of the Circle struct
        let circle = Circle { radius: 5.0 };
        // Verify if the computed area of the circle is correct
        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
    }

    #[test]
    fn test_triangle_area() {
        // Test the area method of the Triangle struct
        let triangle = Triangle { base: 4.0, height: 3.0 };
        // Verify if the computed area of the triangle is correct
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_square_area() {
        // Test the area method of the Square struct
        let square = Square { side: 6.0 };
        // Verify if the computed area of the square is correct
        assert_eq!(square.area(), 36.0);
    }

    #[test]
    fn test_print_area() {
        // Capture printed output
        let mut cmd = Command::cargo_bin("ShapeArea_4").unwrap();
        cmd.assert().stdout(predicate::str::contains("The area of the shape is:"));
    }
}

// Cargo test result:
// running 4 tests
// test tests::test_circle_area ... ok
// test tests::test_triangle_area ... ok
// test tests::test_square_area ... ok
// test tests::test_print_area ... ok
// test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
