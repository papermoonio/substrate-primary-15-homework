
fn sum_u32(slice: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in slice {
        if let Some(new_num) = sum.checked_add(num){
            sum = new_num;
        } else {
            return None;
        }
    }
    Some(sum)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    let result1 = sum_u32(&numbers);
    println!("{:?}", result1);
    let numbers1 = vec![u32::MAX, 1];
    let result2 = sum_u32(&numbers1);
    println!("{:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_u32() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7];
        let result1 = sum_u32(&numbers);
        assert_eq!(result1, Some(28));
        let numbers1 = vec![u32::MAX, 1];
        let result2 = sum_u32(&numbers1);
        assert_eq!(result2, None);
    }
}