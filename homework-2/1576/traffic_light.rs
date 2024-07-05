pub enum TrafficLight {
    Red,
    Green,
    Yellow
}

impl TrafficLight {
    fn duration(var: TrafficLight) -> usize {
        return match var {
            TrafficLight::Red => 20,
            TrafficLight::Green => 90,
            TrafficLight::Yellow => 6
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_traffic_light() {
        use super::TrafficLight;
        let red = TrafficLight::Red;
        let green = TrafficLight::Green;
        let yellow = TrafficLight::Yellow;
        assert_eq!(TrafficLight::duration(red), 20);
        assert_eq!(TrafficLight::duration(green), 90);
        assert_eq!(TrafficLight::duration(yellow), 6);
    }
}