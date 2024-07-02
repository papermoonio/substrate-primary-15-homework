/*第二题：为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同*/

enum TrafficLight{
    Red,
    Yellow,
    Green,
}

trait Duration{
    fn time(&self) -> u32;
}

impl Duration for TrafficLight{
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 99,
            TrafficLight::Yellow => 2,
            TrafficLight::Green => 60,
        }
    }
}

fn main() {

    let red = TrafficLight::Red;

    let yellow = TrafficLight::Yellow;

    let green = TrafficLight::Green;

    println!("红灯持续 {} 秒", red.time());
    println!("黄灯持续 {} 秒", yellow.time());
    println!("绿灯持续 {} 秒", green.time());
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn red_light_duration() {
        let light = TrafficLight::Red;
        assert_eq!(light.time(), 99);
    }

    #[test]
    fn yellow_light_duration() {
        let light = TrafficLight::Yellow;
        assert_eq!(light.time(), 2);
    }

    #[test]
    fn green_light_duration() {
        let light = TrafficLight::Green;
        assert_eq!(light.time(), 60);
    }
}
