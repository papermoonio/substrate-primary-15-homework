fn main() {
    assert_eq!(TrafficLight::Red.duration(), 30);
    assert_eq!(TrafficLight::Yellow.duration(), 5);
    assert_eq!(TrafficLight::Green.duration(), 45);
}
// 定义枚举类型
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义 Duration trait
trait Duration {
    fn duration(&self) -> u32;
}

//实现 Duration trait
impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

// 测试TrafficLight
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 30);
        assert_eq!(TrafficLight::Yellow.duration(), 5);
        assert_eq!(TrafficLight::Green.duration(), 45);
    }
}
