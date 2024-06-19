// Define the enum for traffic lights
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Define a trait for duration
trait Duration {
    // Method to return the duration time
    fn time(&self) -> u8;
}

// Implement the Duration trait for TrafficLight enum
impl Duration for TrafficLight {
    fn time(&self) -> u8 {
        // Match each traffic light variant to its respective time duration
        match self {
            TrafficLight::Red => 10,    // Red light lasts for 10 seconds
            TrafficLight::Yellow => 3,  // Yellow light lasts for 3 seconds
            TrafficLight::Green => 15,  // Green light lasts for 15 seconds
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    // Print the duration of each traffic light
    println!("Red light lasts for {} seconds", red.time());
    println!("Yellow light lasts for {} seconds", yellow.time());
    println!("Green light lasts for {} seconds", green.time());
}

// cargo run result:
// Red light lasts for 10 seconds
// Yellow light lasts for 3 seconds
// Green light lasts for 15 seconds

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_light_duration() {
        let red_light = TrafficLight::Red;
        assert_eq!(red_light.time(), 10);
    }

    #[test]
    fn test_yellow_light_duration() {
        let yellow_light = TrafficLight::Yellow;
        assert_eq!(yellow_light.time(), 3);
    }

    #[test]
    fn test_green_light_duration() {
        let green_light = TrafficLight::Green;
        assert_eq!(green_light.time(), 15);
    }
}

// cargo test result:
// running 3 tests
// test tests::test_green_light_duration ... ok
// test tests::test_red_light_duration ... ok
// test tests::test_yellow_light_duration ... ok

// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s