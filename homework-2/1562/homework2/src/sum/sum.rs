// Copyright (c) david Technologies Co.Ltd. 2015-2022. 
// All rights reserved. Licensed under Apache-2.0.


/// 求和程序。
///
/// # Arguments
///
/// * `nums` - 要排序的可变数组，类型为 `&mut [u32]`
///
/// # Examples
///
/// ```
/// use homework2::sum::sum::sum_u32;
/// let mut test_arrs = vec![4, 3, 2, 1];
/// let sum = sum_u32(&mut test_arrs);
/// assert_eq!(sum, Some(10));
/// ```
///
pub fn sum_u32(nums: &[u32])->Option<u32> {
    nums.iter().try_fold(0u32, |acc, &num| acc.checked_add(num))
}

#[cfg(test)]
mod tests {
    use super::sum_u32;
    #[test]
    fn test_sum_u32() {
        let mut test_arrs = vec![4, 3, 2, 1];
        let sum = sum_u32(&mut test_arrs);
        assert_eq!(sum, Some(10));
    }
}