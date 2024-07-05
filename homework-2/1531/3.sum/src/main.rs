fn sum(arr: &[u32]) -> Option<u32> {
    arr.iter().try_fold(0, |acc: u32, &x| acc.checked_add(x))
}

#[cfg(test)]
mod tests {
    use super::sum;

    #[test]
    fn test_sum_with_empty_array() {
        let arr: [u32; 0] = [];
        let result = sum(&arr);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_sum_with_single_element() {
        let arr = [42];
        let result = sum(&arr);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_sum_with_multiple_elements_no_overflow() {
        let arr = [1, 2, 3, 4];
        let result = sum(&arr);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_sum_with_overflow() {
        let arr = [u32::MAX; 2]; // This will cause an overflow
        let result = sum(&arr);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sum_with_mixed_values() {
        let arr = [u32::MAX - 10, 10, 1];
        let result = sum(&arr);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sum_with_values_near_overflow() {
        let arr = [u32::MAX - 1, 1];
        let result = sum(&arr);
        assert_eq!(result, Some(u32::MAX));
    }

    #[test]
    fn test_sum_with_all_zeros() {
        let arr = [0; 10]; // 10 zeros
        let result = sum(&arr);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_sum_with_large_numbers() {
        let arr = [10000000, 20000000, 30000000];
        let result = sum(&arr);
        assert_eq!(result, Some(60000000));
    }
}

fn main() {
    let numbers_normal = vec![1, 2, 3, 4, 5];
    match sum(&numbers_normal) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }

    let numbers_overflow = vec![1, u32::MAX, 2];
    match sum(&numbers_overflow) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }
}
