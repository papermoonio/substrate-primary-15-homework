// 定义交通信号灯枚举
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义 trait
trait Duration {
    fn duration(&self) -> u32;
}

// 为 TrafficLight 实现 Duration trait
impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,    // 红灯持续 30 秒
            TrafficLight::Yellow => 5,  // 黄灯持续 5 秒
            TrafficLight::Green => 45,  // 绿灯持续 45 秒
        }
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 30);
    }

    #[test]
    fn test_yellow_light_duration() {
        assert_eq!(TrafficLight::Yellow.duration(), 5);
    }

    #[test]
    fn test_green_light_duration() {
        assert_eq!(TrafficLight::Green.duration(), 45);
    }

    #[test]
    fn test_all_lights_different_durations() {
        let red = TrafficLight::Red;
        let yellow = TrafficLight::Yellow;
        let green = TrafficLight::Green;

        assert_ne!(red.duration(), yellow.duration());
        assert_ne!(yellow.duration(), green.duration());
        assert_ne!(red.duration(), green.duration());
    }

    #[test]
    fn test_light_equality() {
        assert_eq!(TrafficLight::Red, TrafficLight::Red);
        assert_ne!(TrafficLight::Red, TrafficLight::Green);
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("红灯持续时间: {} 秒", red.duration());
    println!("黄灯持续时间: {} 秒", yellow.duration());
    println!("绿灯持续时间: {} 秒", green.duration());
}