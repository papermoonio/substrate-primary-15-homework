u32 求和函数及溢出检查

/// 计算 u32 类型整数集合的和，溢出时返回 None
fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty_slice() {
        assert_eq!(sum_u32(&[]), Some(0));
    }

    #[test]
    fn test_sum_single_element() {
        assert_eq!(sum_u32(&[42]), Some(42));
    }

    #[test]
    fn test_sum_multiple_elements() {
        assert_eq!(sum_u32(&[1, 2, 3, 4, 5]), Some(15));
    }

    #[test]
    fn test_sum_large_numbers() {
        assert_eq!(sum_u32(&[u32::MAX - 10, 5, 5]), Some(u32::MAX));
    }

    #[test]
    fn test_sum_overflow() {
        assert_eq!(sum_u32(&[u32::MAX, 1]), None);
    }

    #[test]
    fn test_sum_multiple_overflows() {
        assert_eq!(sum_u32(&[u32::MAX, u32::MAX, u32::MAX]), None);
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred"),
    }

    let large_numbers = vec![u32::MAX, 1];
    match sum_u32(&large_numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred"),
    }
}