pub fn sum_u32_slice(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for &num in nums {
        // if let Some(result) = sum.checked_add(num) {
        //     sum = result;
        // } else {
        //     return None; // 溢出返回 None
        // }
        sum = sum.checked_add(num)?;
    }

    Some(sum) // 返回求和结果 隐式
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32_list() {
        let numbers = &[1, 2, 3, 4, 5];
        assert_eq!(sum_u32_slice(numbers), Some(15));

        let big_numbers = &[4294967295, 4294967295];
        assert_eq!(sum_u32_slice(big_numbers), None);
    }
}
fn main() {
    // 测试用例
    let numbers = &[1, 2, 3, 4, 5];
    match sum_u32_slice(numbers) {
        Some(result) => println!("Sum: {}", result),
        None => println!("Overflow occurred!"),
    }

    let big_numbers = &[4294967295, 4294967295];
    match sum_u32_slice(big_numbers) {
        Some(result) => println!("Sum: {}", result),
        None => println!("Overflow occurred!"),
    }
}
