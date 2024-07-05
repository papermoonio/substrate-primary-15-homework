enum TrafficSignal {
    Red(u32),
    Green(u32),
    Yellow(u32),
}

trait Timer {
    fn time(&self) -> u32;
    fn set_time(&mut self, duration: u32);
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

fn main() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let red = TrafficSignal::Red(30);
        assert_eq!(red.time(), 30);
    }

    #[test]
    fn test_set_time() {
        let mut yellow = TrafficSignal::Yellow(5);
        assert_eq!(yellow.time(), 5);
        yellow.set_time(3);
        assert_eq!(yellow.time(), 3);
    }
}

