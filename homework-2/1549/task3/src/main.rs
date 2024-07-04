fn main() {
}
fn sum_checked(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &number in numbers {
        match sum.checked_add(number) {
            Some(result) => sum = result,
            None => return None,
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_sum_u32() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_checked(&arr), Some(15));
    }

    #[test]
    fn test_sum_u32_overflow() {
        let arr = [u32::MAX,1];
        assert_eq!(sum_checked(&arr), None);
    }
}