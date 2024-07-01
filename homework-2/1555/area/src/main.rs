pub mod shape;

#[warn(unused_imports)]
use shape::*;

fn main() {
    println!("Homework_2 - task_4:");

    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(4.0, 6.0);

    println!("Area of the circle: {}", circle.area());
    println!("Area of the rectangle: {}", rectangle.area());

    let int_circle = Circle::new(5);  // Using integer radius
    let int_rectangle = Rectangle::new(4, 6);  // Using integer dimensions

    println!("Area of the integer circle: {}", int_circle.area());
    println!("Area of the integer rectangle: {}", int_rectangle.area());
}


//Unit test, run by `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        
    }

    #[test]
    fn test_rectangle() {
        
    }
}