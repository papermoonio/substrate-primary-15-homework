use std::{borrow::Borrow, os::unix::raw::pid_t};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}




trait FloatCompatible:Copy + std::ops::Mul<f64, Output = Self> + Into<f64> {}
impl<T: Copy + std::ops::Mul<f64, Output = Self> + Into<f64>> FloatCompatible for T  {}

pub trait Compute {
     fn area_compute(&self)->Option<f64>;
}


#[derive(Debug)]
struct Circles<U: FloatCompatible >{
    radius: U,
}
impl<U:FloatCompatible> Circles<U>{
    fn new(radius: U)-> Self{
        Circles { radius }
    }
}

impl<U: FloatCompatible> Compute for Circles<U >{
     fn area_compute(&self)->Option<f64> {
        let pi = 3.14;
        let redius :f64 = self.radius.into();
        Some(redius * redius * pi)
    }
}

#[derive(Default,Debug)]
struct Squares<U: FloatCompatible>{
    length:U,
    width:U,
}
impl <U:FloatCompatible> Squares<U> {
    fn new(length:U,width:U) -> Self {
        Squares { length, width }
    }
}
impl<U:FloatCompatible> Compute for Squares<U> {
    fn area_compute(&self)->Option<f64> {
        let length:f64 = self.length.into();
        let width:f64 = self.width.into();
        Some(length * width)
    }
    
}
#[derive(Default,Debug)]
struct Triangles{
    base:f64,
    height:f64,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circles = Circles::new(32.0);
        println!("{}",circles.area_compute().unwrap());
        let squares = Squares::new(55.0, 23.2);
        println!("{}",squares.area_compute().unwrap());

        
    }
}
