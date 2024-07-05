fn sum(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0_u32;
    for &number in numbers {
        match sum.checked_add(number) {
            Some(tmp_sum) => sum = tmp_sum,
            None => return None,
        }
    }
    Some(sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let numbers = vec![1, 1, 1];
        assert_eq!(sum(&numbers), Some(3));
    }

    #[test]
    fn test_overflow() {
        let numbers = vec![1, u32::MAX, 2];
        assert_eq!(sum(&numbers), None);
    }


}

fn main() {
    println!("Hello, world!");
}
