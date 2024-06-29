enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match *self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("Red light duration: {} seconds", red_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_duration() {
        let red_light = TrafficLight::Red;
        let yellow_light = TrafficLight::Yellow;
        let green_light = TrafficLight::Green;

        assert_eq!(red_light.duration(), 30);
        assert_eq!(yellow_light.duration(), 5);
        assert_eq!(green_light.duration(), 45);
    }
}
