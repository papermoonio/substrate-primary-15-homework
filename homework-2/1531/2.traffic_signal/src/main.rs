enum TrafficSignal {
    Red(u32),
    Yellow(u32),
    Green(u32),
}

trait Time {
    fn get_time(&self) -> u32;
    fn set_time(&mut self, all_time: u32);
    fn add_time(&mut self, ex_time: u32);
    fn sleep_time(&mut self, sleep_time: u32);
}

impl Time for TrafficSignal {
    fn get_time(&self) -> u32 {
        match *self {
            TrafficSignal::Green(time) => time,
            TrafficSignal::Red(time) => time,
            TrafficSignal::Yellow(time) => time,
        }
    }
    fn set_time(&mut self, all_time: u32) {
        match self {
            TrafficSignal::Green(time) => *time = all_time,
            TrafficSignal::Red(time) => *time = all_time,
            TrafficSignal::Yellow(time) => *time = all_time,
        }
    }

    fn add_time(&mut self, ex_time: u32) {
        match self {
            TrafficSignal::Green(time) => *time = *time + ex_time,
            TrafficSignal::Red(time) => *time = *time + ex_time,
            TrafficSignal::Yellow(time) => *time = *time + ex_time,
        }
    }

    fn sleep_time(&mut self, sleep_time: u32) {
        match self {
            TrafficSignal::Green(time) => *time = time.saturating_sub(sleep_time),
            TrafficSignal::Red(time) => *time = time.saturating_sub(sleep_time),
            TrafficSignal::Yellow(time) => *time = time.saturating_sub(sleep_time),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{Time, TrafficSignal};

    #[test]
    fn test_get_time() {
        let mut signal = TrafficSignal::Green(5);
        assert_eq!(signal.get_time(), 5);

        let mut signal = TrafficSignal::Red(10);
        assert_eq!(signal.get_time(), 10);

        let mut signal = TrafficSignal::Yellow(3);
        assert_eq!(signal.get_time(), 3);
    }

    #[test]
    fn test_set_time() {
        let mut signal = TrafficSignal::Green(0);
        signal.set_time(10);
        assert_eq!(signal.get_time(), 10);

        let mut signal = TrafficSignal::Red(0);
        signal.set_time(20);
        assert_eq!(signal.get_time(), 20);

        let mut signal = TrafficSignal::Yellow(0);
        signal.set_time(5);
        assert_eq!(signal.get_time(), 5);
    }

    #[test]
    fn test_add_time() {
        let mut signal = TrafficSignal::Green(5);
        signal.add_time(5);
        assert_eq!(signal.get_time(), 10);

        let mut signal = TrafficSignal::Red(10);
        signal.add_time(10);
        assert_eq!(signal.get_time(), 20);

        let mut signal = TrafficSignal::Yellow(3);
        signal.add_time(7);
        assert_eq!(signal.get_time(), 10);
    }

    #[test]
    fn test_sleep_time() {
        let mut signal = TrafficSignal::Green(10);
        signal.sleep_time(5);
        assert_eq!(signal.get_time(), 5);

        let mut signal = TrafficSignal::Red(20);
        signal.sleep_time(15);
        assert_eq!(signal.get_time(), 5);

        let mut signal = TrafficSignal::Yellow(3);
        signal.sleep_time(3);
        assert_eq!(signal.get_time(), 0);
    }

    #[test]
    fn test_sleep_time_underflow() {
        // 测试时间减去大于当前时间的值，确保使用saturating_sub正确处理下溢
        let mut signal = TrafficSignal::Green(5);
        signal.sleep_time(10);
        assert_eq!(signal.get_time(), 0);

        let mut signal = TrafficSignal::Red(1);
        signal.sleep_time(2);
        assert_eq!(signal.get_time(), 0);

        let mut signal = TrafficSignal::Yellow(0);
        signal.sleep_time(1);
        assert_eq!(signal.get_time(), 0);
    }
}

fn main() {
    let mut red = TrafficSignal::Red(30);
    let green = TrafficSignal::Green(30);
    let yellow = TrafficSignal::Yellow(3);
    println!("Red signal time: {}", red.get_time());
    println!("Green signal time: {}", green.get_time());
    println!("Yellow signal time: {}", yellow.get_time());

    red.set_time(60);
    println!("Red signal time: {}", red.get_time());
    println!("Green signal time: {}", green.get_time());
    println!("Yellow signal time: {}", yellow.get_time());

    red.add_time(20);
    println!("Red signal time: {}", red.get_time());
    println!("Green signal time: {}", green.get_time());
    println!("Yellow signal time: {}", yellow.get_time());

    red.sleep_time(90);
    println!("Red signal time: {}", red.get_time());
    println!("Green signal time: {}", green.get_time());
    println!("Yellow signal time: {}", yellow.get_time());
}
