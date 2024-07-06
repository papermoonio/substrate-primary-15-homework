fn sum_u32(nums: &[u32]) -> Option<u32> {
    nums.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    match sum_u32(&nums) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32_normal() {
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&nums), Some(15));
    }

    #[test]
    fn test_sum_u32_empty() {
        let nums: [u32; 0] = [];
        assert_eq!(sum_u32(&nums), Some(0));
    }

    #[test]
    fn test_sum_u32_overflow() {
        let nums = [u32::MAX, 1];
        assert_eq!(sum_u32(&nums), None);
    }
}


