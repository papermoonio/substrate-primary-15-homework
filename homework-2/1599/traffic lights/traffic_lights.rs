// 定义一个交通信号灯的枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个trait，包含一个返回时间的方法
trait LightDuration {
    fn time(&self) -> u32;
}

// 为TrafficLight枚举实现LightDuration trait
impl LightDuration for TrafficLight {
    fn time(&self) -> u32 {
        match *self {
            TrafficLight::Red => 60,    // 红灯持续60秒
            TrafficLight::Yellow => 10, // 黄灯持续10秒
            TrafficLight::Green => 30,  // 绿灯持续30秒
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("红灯: {}秒", red.time());
    println!("黄灯: {}秒", yellow.time());
    println!("绿灯: {}秒", green.time());
}
