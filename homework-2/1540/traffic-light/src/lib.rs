pub enum TrafficLight {
    Red(u8),
    Green(u8),
    Yellow(u8),
}

impl TrafficLight {
    pub fn light_last(self) -> u8 {
        match self {
            TrafficLight::Red(red_light_last) => red_light_last,
            TrafficLight::Green(green_light_last) => green_light_last,
            TrafficLight::Yellow(yellow_light_last) => yellow_light_last,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let red = TrafficLight::Red(30);
        let green = TrafficLight::Green(25);
        let yellow = TrafficLight::Yellow(3);

        let red_light_last = red.light_last();
        println!("red_light_last:{}", red_light_last);
        let green_light_last = green.light_last();
        println!("green_light_last:{}", green_light_last);
        let yellow_light_last = yellow.light_last();
        println!("yellow_light_last:{}", yellow_light_last);
    }
}
