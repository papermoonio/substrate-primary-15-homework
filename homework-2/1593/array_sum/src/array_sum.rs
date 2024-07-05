pub fn array_sum(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in arr {
        sum = sum.checked_add(*i)?;
    }
    Some(sum)
}
