pub mod light;
use light::*;

fn main() {
    println!("\n\nHomework-2 #task_2: Traffice light");

    let red = Red{};
    println!("Red ligth time: {}s", red.time());

    let green = Green{};
    println!("Green ligth time: {}s", green.time());

    let yellow = Yellow{};
    println!("Yellow ligth time: {}s", yellow.time());
}


//Unit test, run by `cargo test`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_last_time() {
        let expected = 60;
        let red = Red{};
        assert_eq!(red.time(), expected);
    }

    #[test]
    fn test_green_last_time() {
        let expected = 50;
        let green = Green{};
        assert_eq!(green.time(), expected);
    }

    #[test]
    fn test_yellow_last_time() {
        let expected = 3;
        let yellow = Yellow{};
        assert_eq!(yellow.time(), expected);
    }
}