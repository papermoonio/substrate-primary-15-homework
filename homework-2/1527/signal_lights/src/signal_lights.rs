trait SignalLightsShow{
    fn light_shows();
}

pub enum LightColor {
    RED,
    GREEN,
    ORANGE,
}

pub struct RedTimes;
pub struct GreenTimes;
pub struct OrangeTimes;

impl SignalLightsShow for RedTimes{
    fn light_shows() {
        println!("红灯亮起：{}秒",90);
    }
}

impl SignalLightsShow for GreenTimes{
    fn light_shows() {
        println!("绿灯亮起：{}秒",60);
    }
}

impl SignalLightsShow for OrangeTimes{
    fn light_shows() {
        println!("黄灯亮起：{}秒",3);
    }
}


pub fn next_signal(mut light: LightColor) {
    for i in 0..10  {
        println!("第{}次亮灯",i+1);
        match light {
            LightColor::RED => {RedTimes::light_shows();light = LightColor::ORANGE;},
            LightColor::ORANGE => {OrangeTimes::light_shows();light = LightColor::GREEN;},
            LightColor::GREEN => {GreenTimes::light_shows();light = LightColor::RED;}, 
        }
    }
}

#[cfg(test)]

#[test]
fn light_test() {
    next_signal(LightColor::RED);
}


