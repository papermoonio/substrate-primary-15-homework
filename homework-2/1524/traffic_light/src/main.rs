fn main() {
    println!("Hello, world!");

    let red = TrafficLight::Red;
    let red_time = red.returnTime();
    println!("red_time is {}", red_time);

    let yellow = TrafficLight::Yellow;
    let yellow_time = yellow.returnTime();
    println!("yellow_time is {}", yellow_time);

    let green = TrafficLight::Green;
    let green_time = green.returnTime();
    println!("green_time is {}", green_time);
}

trait ReturnTime {
    fn returnTime(&self) -> u32;
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl ReturnTime for TrafficLight {
    fn returnTime(&self) -> u32 {
        match *self {
            TrafficLight::Red => 90,
            TrafficLight::Yellow => 15,
            TrafficLight::Green => 30,
        }
    }
}
