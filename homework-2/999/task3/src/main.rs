fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut res = 0u32;

    for &num in nums {
        res = res.checked_add(num)?;
    }
    Some(res)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    match sum_u32(&numbers) {
        Some(result) => println!("Sum is: {}", result),
        None => println!("Overflow occurred"),
    }

    let overflow_numbers = vec![1, 2, u32::MAX, 4, 5,];
    match sum_u32(&overflow_numbers) {
        Some(result) => println!("Sum is: {}", result),
        None => println!("Overflow occurred"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32_empty() {
        let nums: Vec<u32> = vec![];
        assert_eq!(sum_u32(&nums), Some(0));
    }

    #[test]
    fn test_sum_u32_single_element() {
        let nums = vec![42];
        assert_eq!(sum_u32(&nums), Some(42));
    }

    #[test]
    fn test_sum_u32_multiple_elements() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&nums), Some(15));
    }

    #[test]
    fn test_sum_u32_with_overflow() {
        let nums = vec![u32::MAX, 1];
        assert_eq!(sum_u32(&nums), None);
    }

    #[test]
    fn test_sum_u32_large_numbers_no_overflow() {
        let nums = vec![u32::MAX - 1, 1];
        assert_eq!(sum_u32(&nums), Some(u32::MAX));
    }

    #[test]
    fn test_sum_u32_all_zeros() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(sum_u32(&nums), Some(0));
    }
}