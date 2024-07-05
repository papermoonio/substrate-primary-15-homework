pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait Duration {
    fn duration(&self) -> u8;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_red_light_duration() {
        let red_light = TrafficLight::Red;
        assert_eq!(red_light.duration(), 60);
    }

    #[test]
    fn test_yellow_light_duration() {
        let yellow_light = TrafficLight::Yellow;
        assert_eq!(yellow_light.duration(), 5);
    }

    #[test]
    fn test_green_light_duration() {
        let green_light = TrafficLight::Green;
        assert_eq!(green_light.duration(), 45);
    }
}