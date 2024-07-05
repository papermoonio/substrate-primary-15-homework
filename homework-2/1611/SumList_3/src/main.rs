fn sum_u32_list(numbers: &[u32]) -> Option<u32> {
    // Use try_fold method to compute the sum of the integer collection
    let result = numbers.iter().try_fold(0u32, |acc, &x| {
        // Use checked_add method for safe addition operation and check for overflow
        acc.checked_add(x)
    });

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = sum_u32_list(&numbers);
    println!("Sum: {:?}", sum);

    let numbers_overflow = vec![u32::MAX, u32::MAX];
    let sum_overflow = sum_u32_list(&numbers_overflow);
    println!("Sum (with overflow): {:?}", sum_overflow);
}

// Cargo run result:
// Sum: Some(15)
// Sum (with overflow): None

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32_list_normal_case() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_u32_list(&numbers), Some(15));
    }

    #[test]
    fn test_sum_u32_list_empty_case() {
        let numbers: Vec<u32> = vec![];
        assert_eq!(sum_u32_list(&numbers), Some(0));
    }

    #[test]
    fn test_sum_u32_list_overflow_case() {
        let numbers = vec![u32::MAX, u32::MAX];
        assert_eq!(sum_u32_list(&numbers), None);
    }
}

// Cargo test result:
// running 3 tests
// test tests::test_sum_u32_list_empty_case ... ok
// test tests::test_sum_u32_list_normal_case ... ok
// test tests::test_sum_u32_list_overflow_case ... ok
// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s