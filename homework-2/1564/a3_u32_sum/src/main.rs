fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in numbers {
        if let Some(result) = sum.checked_add(num) {
            sum = result;
        } else {
            return None;
        }
    }
    Some(sum)
}

fn main() {
    // println!("Hello, world!");
    let numbers1 = [1, 2, 3, 4, 5];
    let ret = sum_u32(&numbers1);
    if let Some(val) = ret {
        // 有值的情况
        println!("The value is: {}", val);
    } else {
        // 没有值的情况
        println!("There is None.");
    }

    let numbers1 = [u32::MAX, 1];
    let ret = sum_u32(&numbers1);
    if let Some(val) = ret {
        // 有值的情况
        println!("The value is: {}", val);
    } else {
        // 没有值的情况
        println!("There is None.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32() {
        // Test case 1: Sum within u32 range
        let numbers1 = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&numbers1), Some(15));

        // Test case 2: Sum exceeds u32 range
        let numbers2 = [u32::MAX, 1];
        assert_eq!(sum_u32(&numbers2), None);
    }
}
