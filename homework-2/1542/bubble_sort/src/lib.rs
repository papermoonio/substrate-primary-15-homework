use core::fmt;
use std::fmt::write;



struct Vecdisplay<'a,T:'a>(&'a[T]);

impl <'a,T:fmt::Display+'a> fmt::Display for Vecdisplay<'a,T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for item in self.0  {
            if !first {
                write!(f,",{}",item)?;
            }else {
                write!(f,"{}",item)?;
            }
            first = false;
        }
        Ok(())
    }
}


pub fn bubble_sort_paord<T:PartialOrd+Copy>(vec: &mut Vec<T>) -> &Vec<T>{
    for i in 0..vec.len(){
        for j in 0..vec.len()-1  {
            if vec[j]>(vec[j+1]){
                vec.swap(j, j+1);
                
            }
        }
    }
    vec
    
}


#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn bubble_test(){
        let mut  vec = vec!["1.2","1","23","353","45","2034"];
        let mut  vec = vec![34,1,35235,45,258,97];
        let mut  vec = vec![34.2,1.0,3523.5,45.56,258.3,0.02];
        let paord = bubble_sort_paord(&mut vec);
        println!("{}",Vecdisplay(paord));
        
    }
}
