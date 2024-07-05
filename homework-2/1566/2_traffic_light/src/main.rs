enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Time {
    fn time(&self) -> u8;
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 15,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("red light time: {}", red.time());
    println!("green light time: {}", green.time());
    println!("yellow light time: {}", yellow.time());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_time() {
        let red_light = TrafficLight::Red;
        let yellow_light = TrafficLight::Yellow;
        let green_light = TrafficLight::Green;

        assert_eq!(red_light.time(), 10);
        assert_eq!(yellow_light.time(), 3);
        assert_eq!(green_light.time(), 15);
    }
}
