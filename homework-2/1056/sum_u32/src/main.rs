fn sum_u32(nums:&[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    let len = nums.len();
    for i in 0..len {
        sum = sum.checked_add(nums[i])?;
    }
    Some(sum)
}

fn main() {
    let nums = &[10, 7, 8, 6, 3];
    match sum_u32(nums) {
        Some(result) => println!("Sum:{}", result),
        None => println!("Overflow!"),
    }

    let big_nums = &[4294967295, 4294967295, 4294967295, 4294967295, 4294967295];
    match sum_u32(big_nums) {
        Some(result) => println!("Sum:{}", result),
        None => println!("Overflow!"),
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_sum_u32() {
        let nums = &[10, 7, 8, 6, 3];
        assert_eq!(sum_u32(nums), Some(34));

        let big_nums = &[4294967295, 4294967295, 4294967295, 4294967295, 4294967295];
        assert_eq!(sum_u32(big_nums), None);
    }
}