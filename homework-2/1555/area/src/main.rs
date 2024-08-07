pub mod shape;

#[warn(unused_imports)]
use shape::*;

fn main() {
    println!("Homework_2 - task_4:");

    let shape_01 = Shape::new(String::from("circle"),vec![1.0]);
    //shape_01.display();
    println!("Shape is {}, area is {}",shape_01.shape, shape_01.area());

    let shape_02 = Shape::new("rectangle".to_string(),vec![8.0,9.0]);
    println!("Shape is {}, area is {}",shape_02.shape, shape_02.area());
}


//Unit test, run by `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let expected=3.141592653589793;
        let sp = Shape::new(String::from("circle"),vec![1.0]);
        assert_eq!(sp.area(), expected);
    }

    #[test]
    fn test_rectangle() {
        let expected=48.0;
        let sp = Shape::new(String::from("rectangle"),vec![8.0,6.0]);
        assert_eq!(sp.area(), expected);
    }
}