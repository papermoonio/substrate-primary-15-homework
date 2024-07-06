pub fn sum_numbers(numbers: &[u32]) -> Option<u32> {
    let sum = numbers
        .iter()
        .try_fold(0u32, |acc, &num| acc.checked_add(num));

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers() {
        // 测试正常情况，没有溢出
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_numbers(&numbers), Some(15));

        // 测试溢出情况
        let numbers = vec![u32::MAX, 1];
        assert_eq!(sum_numbers(&numbers), None);
    }
}
