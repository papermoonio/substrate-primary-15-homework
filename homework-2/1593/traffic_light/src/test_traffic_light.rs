use crate::traffic_light::TrafficLight;
use crate::traffic_light::TrafficLightTime;

#[test]
fn red_light_works(){
    let light = TrafficLight::Red;
    assert_eq!(light.duration(), 60);
}

#[test]
fn yellow_light_works(){
    let light = TrafficLight::Yellow;
    assert_eq!(light.duration(), 10);
}

#[test]
fn green_light_works(){
    let light = TrafficLight::Green;
    assert_eq!(light.duration(), 90);
}