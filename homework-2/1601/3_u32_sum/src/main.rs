fn u32_sum(numbers: &[u32]) -> Option<u32> {
    let result = numbers.iter().try_fold(0u32, |acc, &number| {
        acc.checked_add(number)
    });
    result
}

fn main() {
    println!("My id is 1601");
    let numbers = vec![2000, 0, 20, 4];
    let sum = u32_sum(&numbers);
    match sum {
        Some(n) => println!("u32 sum : {}", n),
        None => println!("None"),
    }

    let numbers_overflow = vec![u32::MAX, u32::MAX];
    let sum_overflow = u32_sum(&numbers_overflow);
    match sum_overflow {
        Some(n) => println!("u32 sum : {}", n),
        None => println!("None"),
    }
}

// main results:
/*
u32 sum : 2024
None
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_sum_1() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(u32_sum(&numbers), Some(15));
    }

    #[test]
    fn test_u32_sum_2() {
        let numbers: Vec<u32> = vec![];
        assert_eq!(u32_sum(&numbers), Some(0));
    }

    #[test]
    fn test_sum_u32_list_overflow_case() {
        let numbers = vec![u32::MAX, u32::MAX];
        assert_eq!(u32_sum(&numbers), None);
    }
}

// test results:
/*
running 3 tests
test tests::test_sum_u32_list_overflow_case ... ok
test tests::test_u32_sum_2 ... ok
test tests::test_u32_sum_1 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/