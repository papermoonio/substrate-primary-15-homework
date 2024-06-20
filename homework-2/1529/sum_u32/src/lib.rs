pub fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum = 0u32;

    for &num in nums {
        match sum.checked_add(num) {
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
    fn test_normal_sum() {
        let nums = vec![1,2,3,4,5];
        assert_eq!(sum_u32(&nums), Some(15));
    }

    #[test]
    fn test_empty_slice() {
        let nums: Vec<u32> = vec![];
        assert_eq!(sum_u32(&nums), Some(0))
    }

    #[test]
    fn test_overflow() {
        let nums = vec![u32::MAX, 1];
        assert_eq!(sum_u32(&nums), None);
    }
}