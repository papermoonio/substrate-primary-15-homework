use duration::Duration;
use rand::Rng;
use traffic_light::TrafficLight;

mod duration;
mod traffic_light;


fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("Red light duration: {}", red.duration());
    println!("Green light duration: {}", green.duration());
    println!("Yellow light duration: {}", yellow.duration());

    println!("=======================================================");

    let rand_light = rand_traffic_light();
    match rand_light {
        TrafficLight::Red => println!("Red light duration: {}", rand_light.duration()),
        TrafficLight::Green => println!("Green light duration: {}", rand_light.duration()),
        TrafficLight::Yellow => println!("Yellow light duration: {}", rand_light.duration()),
    }
}

fn rand_traffic_light() -> TrafficLight {
    let rand = rand::thread_rng().gen_range(0..3);

    match rand {
        0 => TrafficLight::Red,
        1 => TrafficLight::Yellow,
        _ => TrafficLight::Green,
    }
}
