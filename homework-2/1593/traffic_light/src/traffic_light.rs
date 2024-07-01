pub enum TrafficLight {
    Red,
    Yellow,
    Green
}

pub trait TrafficLightTime {
    fn duration(&self) -> u32;
}

impl TrafficLightTime for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 90
        }
    }
}