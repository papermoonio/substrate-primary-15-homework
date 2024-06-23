
// 定义交通信号灯的枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个 trait，其中包含一个返回时间的方法
trait LightDuration {
    fn duration(&self) -> u32;
}

// 为 TrafficLight 枚举实现 LightDuration trait
impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}
// 添加单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_green_light_duration() {
        let green = TrafficLight::Green;
        assert_eq!(green.duration(), 45);
    }

    #[test]
    fn test_yellow_light_duration() {
        let yellow = TrafficLight::Yellow;
        assert_eq!(yellow.duration(), 5);
    }

    #[test]
    fn test_red_light_duration() {
        let red = TrafficLight::Red;
        assert_eq!(red.duration(), 60);
    }
}
fn main() {
    // 定义三个信号灯
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    let red = TrafficLight::Red;

    println!("Gre light durTime: {}s", green.duration());
    println!("Yel light durTime: {}s", yellow.duration());
    println!("Red light durTime: {}s", red.duration());
}
