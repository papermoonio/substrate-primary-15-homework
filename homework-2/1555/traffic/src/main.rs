//use light::*;
pub mod light;

use light::*;

fn main() {
    let red = Red{};
    println!("Red ligth time: {}s", red.time());

    let green = Green{};
    println!("Green ligth time: {}s", green.time());

    let yellow = Yellow{};
    println!("Yellow ligth time: {}s", yellow.time());
}
