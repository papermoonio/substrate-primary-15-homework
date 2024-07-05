use crate::duration::Duration;

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl Duration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        let red = TrafficLight::Red;
        let green = TrafficLight::Green;
        let yellow = TrafficLight::Yellow;

        assert_eq!(red.duration(), 60);
        assert_eq!(green.duration(), 60);
        assert_eq!(yellow.duration(), 10);
    }
}