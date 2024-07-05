fn sum(arr: &[u32]) -> Option<u32> {
    let mut result: u32 = 0;
    for i in 0..arr.len() {
        match result.checked_add(arr[i]) {
            Some(num) => result = num,
            _ => return None
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sum() {
        let a = [1, 3, 5];
        let b =[2147483647, 2147483646, 2147483645, 2147483644];
        assert_eq!(super::sum(&a), Some(9));
        assert_eq!(super::sum(&b), None);
    }
}