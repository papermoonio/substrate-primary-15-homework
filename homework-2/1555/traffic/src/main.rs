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
