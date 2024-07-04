fn sum_u32(arr: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &i in arr {
        match sum.checked_add(i) {
            Some(v) => sum = v,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    match sum_u32(&arr) {
        Some(v) => println!("Sum of array: {}", v),
        None => println!("Overflow!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_u32() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&arr), Some(15));

        let arr = [u32::MAX, 1];
        assert_eq!(sum_u32(&arr), None);
    }
}
