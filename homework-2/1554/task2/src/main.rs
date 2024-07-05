// 定义交通信号灯枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 为TrafficLight枚举实现time方法
impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,    // 红灯持续60秒
            TrafficLight::Yellow => 5,  // 黄灯持续5秒
            TrafficLight::Green => 30,  // 绿灯持续30秒
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_green_30_seconds() {
        // 创建一个绿灯实例
        let light = TrafficLight::Green;
        let green_time = light.time();
        assert_eq!(green_time, 30);
    }

    #[test]
    fn test_red_60_seconds() {
        // 创建一个绿灯实例
        let light = TrafficLight::Red;
        let red_time = light.time();
        assert_eq!(red_time, 60);
    }

    #[test]
    fn test_yeallow_5_seconds() {
        // 创建一个绿灯实例
        let light = TrafficLight::Yellow;
        let yeallow_time = light.time();
        assert_eq!(yeallow_time, 5);
    }

}



fn main() {
    // 创建一个绿灯实例
    let light = TrafficLight::Green;
    
    // 使用模式匹配来处理不同的交通信号灯
    match light {
        TrafficLight::Red => println!("STOP for {} seconds.", light.time()),
        TrafficLight::Yellow => println!("CAUTION for {} seconds.", light.time()),
        TrafficLight::Green => println!("GO for {} seconds.", light.time()),
    }
}