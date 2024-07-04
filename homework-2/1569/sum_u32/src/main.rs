fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    assert_eq!(sum_with_overflow_check(&numbers), Some(15));
}
// 求和函数
fn sum_with_overflow_check(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &num| acc.checked_add(num))
}
// 测试求和函数
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_with_overflow_check() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_with_overflow_check(&numbers), Some(15));

        let large_numbers = vec![u32::MAX, u32::MAX];
        assert_eq!(sum_with_overflow_check(&large_numbers), None);
    }
}
