//为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

enum TrafficSignal {
    Red(u32),
    Green(u32),
    Yellow(u32),
}

trait Timer {
    fn time(&self) -> u32;
    fn set_time(&mut self, duration: u32);
}


fn main() {
    println!("Hello, world!");
    let mut red = TrafficSignal::Red(30);
    let green = TrafficSignal::Green(30);
    let yellow = TrafficSignal::Yellow(3);
    println!("Red signal time: {}", red.time());
    println!("Green signal time: {}", green.time());
    println!("Yellow signal time: {}", yellow.time());

    red.set_time(60);
    println!("Red signal time: {}", red.time());
    println!("Green signal time: {}", green.time());
    println!("Yellow signal time: {}", yellow.time());
}

impl Timer for TrafficSignal {
    fn time(&self) -> u32 {
        match self {
            TrafficSignal::Red(time) => *time,
            TrafficSignal::Green(time) => *time,
            TrafficSignal::Yellow(time) => *time,
        }
    }

    fn set_time(&mut self, duration: u32) {
        match self {
            TrafficSignal::Red(time) => *time = duration,
            TrafficSignal::Green(time) => *time = duration,
            TrafficSignal::Yellow(time) => *time = duration,
        }
    }

}