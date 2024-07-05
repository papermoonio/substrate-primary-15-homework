
enum TrafficLight {
    Red,
    Green,
    Yellow,
}



trait WaitTime {
    fn wait_time(&self) -> i32;
}

impl WaitTime for TrafficLight {
    fn wait_time(&self) -> i32{
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 10,
        }
    }
}

fn main() {
    let a = TrafficLight::Green;
    println!("value a: is {}", a.wait_time());
    let b = TrafficLight::Red;
    println!("value a: is {}", b.wait_time());
    let c = TrafficLight::Yellow;
    println!("value a: is {}", c.wait_time());
}
