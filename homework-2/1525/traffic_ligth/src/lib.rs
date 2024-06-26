use std::thread::sleep;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub enum Ligtn{
    Red,
    Green,
}

pub trait Durntion {
    fn durntion(ligth: &Ligtn);
}


pub struct TrafficLight;
impl TrafficLight {
    fn new() ->Self{
        TrafficLight
    }
}

impl Durntion for TrafficLight{
    fn durntion(ligth: &Ligtn) {
        match ligth {
            Ligtn::Green => {
                println!("green ligth is run");
                sleep(std::time::Duration::from_secs(10))
                
            },
            _ =>{
                println!("red ligth is run");
                sleep(std::time::Duration::from_secs(5))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traffic_ligtn() {
        let red = Ligtn::Red;
        TrafficLight::durntion(&red);
        let green = Ligtn::Green;
        TrafficLight::durntion(&green);
        
    }
}
