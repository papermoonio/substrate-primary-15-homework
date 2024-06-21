enum TrafficLight {
    RED,
    YELLOW,
    GREEN,
}

trait TrafficLightDuration {
    // 定义灯持续时间（单位：秒）
    fn duration(&self) -> u32;
}

impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::RED => 60,
            TrafficLight::YELLOW => 5,
            TrafficLight::GREEN => 50,
        }
    }
}

fn main() {
    println!("Hello, 1547!");
    let red = TrafficLight::RED;
    let yellow = TrafficLight::YELLOW;
    let green = TrafficLight::GREEN;
    println!("red light duration : {} ", red.duration());
    println!("yellow light duration : {} ", yellow.duration());
    println!("green light duration : {} ", green.duration());
}
