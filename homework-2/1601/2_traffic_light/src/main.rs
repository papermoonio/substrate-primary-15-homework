enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn time(&self) -> u8;
}

impl Duration for TrafficLight {
    fn time(&self) -> u8 {
        // setup traffic light time duration
        match self {
            TrafficLight::Red => 20,    
            TrafficLight::Yellow => 2,  
            TrafficLight::Green => 50, 
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("My id is 1601");

    println!("Red light has {} seconds", red.time());
    println!("Yellow light has {} seconds", yellow.time());
    println!("Green light has {} seconds", green.time());
}

// main results:
/*
Red light has 20 seconds
Yellow light has 2 seconds
Green light has 50 seconds
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_light() {
        let red_light = TrafficLight::Red;
        assert_eq!(red_light.time(), 20);
    }

    #[test]
    fn test_yellow_light() {
        let yellow_light = TrafficLight::Yellow;
        assert_eq!(yellow_light.time(), 2);
    }

    #[test]
    fn test_green_light() {
        let green_light = TrafficLight::Green;
        assert_eq!(green_light.time(), 50);
    }
}

// test results:
/*
running 3 tests
test tests::test_green_light ... ok
test tests::test_yellow_light ... ok
test tests::test_red_light ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/