// Copyright (c) david Technologies Co.Ltd. 2015-2022. 
// All rights reserved. Licensed under Apache-2.0.

use std::time::Duration;

pub enum Color {
    Red,
    Green,
    Yellow,
}

pub trait TrafficLight {
    fn get_duration(&self)->Duration;
}

/// 求交通信号灯持续时间程序。
///
/// # Examples
///
/// ```
/// use homework2::traffic::light::{Color,TrafficLight};
/// use std::time::Duration;
/// let red_light = Color::Red;
/// let red_duration = red_light.get_duration();
/// assert_eq!(red_duration, Duration::from_secs(30));
/// ```
///
impl TrafficLight for Color {
    fn get_duration(&self)->Duration {
        match self {
            Color::Red => return Duration::from_secs(30),
            Color::Green => return Duration::from_secs(45),
            Color::Yellow => return Duration::from_secs(3),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Color,TrafficLight};
    use std::time::Duration;
    #[test]
    fn test_traffic_light() {
        let red_light = Color::Red;
        let red_duration = red_light.get_duration();
        assert_eq!(red_duration, Duration::from_secs(30));

        let green_light = Color::Green;
        let green_duration = green_light.get_duration();
        assert_eq!(green_duration, Duration::from_secs(45));
        
        let yellow_light = Color::Yellow;
        let yellow_duration = yellow_light.get_duration();
        assert_eq!(yellow_duration, Duration::from_secs(3));
    }
}