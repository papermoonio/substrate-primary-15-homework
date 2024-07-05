fn u32_sum(numbers: &[u32]) -> Option<u32> {
    let result = numbers.iter().try_fold(0u32, |acc, &number| {
        acc.checked_add(number)
    });
    result
}

fn main() {
    let numbers = vec![100, 201, 302];
    let sum = u32_sum(&numbers);
    match sum {
        Some(n) => println!("sum : {}", n),
        None => println!("None"),
    }

    let numbers_overflow = vec![u32::MAX, u32::MAX];
    let sum_overflow = u32_sum(&numbers_overflow);
    match sum_overflow {
        Some(n) => println!("sum : {}", n),
        None => println!("None"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_sum_1() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(u32_sum(&numbers), Some(28));
    }

    #[test]
    fn test_sum_u32_list_overflow_case() {
        let numbers = vec![u32::MAX, u32::MAX];
        assert_eq!(u32_sum(&numbers), None);
    }
}
