fn sum_u32(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for item in arr {
        match sum.checked_add(*item) {
            Some(new_sum) => sum = new_sum,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    println!("Hello, 1547!");
    println!("-------");
    let arr = [1, 2, 3, 4, 5];
    let sum = sum_u32(&arr);
    match sum {
        Some(n) => println!("Sum : {}", n),
        None => println!("Overflowed"),
    }
    println!("-------");
    let arr = [2000000001, 1294_967_295, 1000000000];
    let sum = sum_u32(&arr);
    match sum {
        Some(n) => println!("Sum : {}", n),
        None => println!("Overflowed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_sum_u32_no_overflow() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&arr), Some(15));
    }

    #[test]
    fn test_sum_u32_large_numbers() {
        let arr = [2000000001, 1294_967_295, 1000000000];
        assert_eq!(sum_u32(&arr), None);
    }
}
