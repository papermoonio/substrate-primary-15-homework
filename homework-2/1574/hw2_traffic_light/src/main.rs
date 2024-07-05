enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 10,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("Red light duration:{}", red.duration());
    println!("Green light duration:{}", green.duration());
    println!("Yellow light duration:{}", yellow.duration());
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_red_light_duration() {
        let red = TrafficLight::Red;
        assert_eq!(red.duration(), 60);
    }

    #[test]
    fn test_green_light_duration() {
        let green = TrafficLight::Green;
        assert_eq!(green.duration(), 60);
    }

    #[test]
    fn test_yellow_light_duration() {
        let yellow = TrafficLight::Yellow;
        assert_eq!(yellow.duration(), 10);
    }
}
