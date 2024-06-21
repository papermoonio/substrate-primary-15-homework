pub fn sum(arr: &[u32]) -> Option<u32> {
    let mut res: u32 = 0;
    for i in arr {
        res = res.checked_add(*i)?;
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = [1, 2, 3, 4];
        assert_eq!(sum(&arr), Some(10));

        let arr = [u32::MAX, 1];
        assert_eq!(sum(&arr), None);
    }
}
