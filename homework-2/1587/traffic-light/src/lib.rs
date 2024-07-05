#[allow(dead_code)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait Time {
    fn time(&self) -> u32;
}

impl Time for TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 15,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(TrafficLight::Red.time(), 10);
        assert_eq!(TrafficLight::Yellow.time(), 3);
        assert_eq!(TrafficLight::Green.time(), 15);
    }
}
