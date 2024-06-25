
trait TrafficLightDuration{
    fn time(&self) -> u32;
}

enum TrafficLight{
    Red,
    Yellow,
    Green,
}

impl TrafficLightDuration for TrafficLight{
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 40,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("红灯是 {} 秒", red.time());
    println!("黄灯是 {} 秒", yellow.time());
    println!("绿灯是 {} 秒", green.time());
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_red_light_duration(){
        assert_eq!(TrafficLight::Red.time(),60);
        assert_eq!(TrafficLight::Yellow.time(),3);
        assert_eq!(TrafficLight::Green.time(),40);
    }
}