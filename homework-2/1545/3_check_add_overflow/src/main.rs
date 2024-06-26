fn sum_with_overflow_check(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0_u32;
    for &number in numbers {
        match sum.checked_add(number) {
            Some(tmp_sum) => sum = tmp_sum,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    let numbers_normal = vec![1, 2, 3];
    match sum_with_overflow_check(&numbers_normal) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }

    let numbers_overflow = vec![1, u32::MAX, 2];
    match sum_with_overflow_check(&numbers_overflow) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let numbers = vec![1, 2, 3];
        assert_eq!(sum_with_overflow_check(&numbers), Some(6));
    }

    #[test]
    fn test_overflow() {
        let numbers = vec![1, u32::MAX, 2];
        assert_eq!(sum_with_overflow_check(&numbers), None);
    }

    #[test]
    fn test_empty() {
        let numbers: Vec<u32> = vec![];
        assert_eq!(sum_with_overflow_check(&numbers), Some(0));
    }
}

