use std::{arch::x86_64, iter};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn sum_u32(list:&[u32]) ->Option<u32>{
    let mut sum:u32=0 ;
    for &item in list {
        // sum = sum.saturating_add(item);
        // sum = sum.checked_add(item).unwrap();
        // sum = sum.checked_add(item).unwrap_or(0);
        sum = sum.checked_add(item)?;
    }

    Some(sum)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        let list = [12u32,34,1,24,54,4294967295];
        // println!("sum is{}",sum_u32(&list).unwrap());
        
        match sum_u32(&list) {
            Some(resutl) =>println!("sum is{}",resutl),
            None => print!("overflow"),
        }
    }
}
