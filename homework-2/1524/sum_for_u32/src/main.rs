fn main() {
    println!("Hello, world!");
    let list: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let list2: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 4294967295];
    let sum = sum_u32(list);
    println!("sum: {:?}", sum);
    let sum1 = sum_u32(list2);
    println!("sum1: {:?}", sum1);

    // Hello, world!
    // sum: Some(91)
    // sum1: None
}

fn sum_u32(list: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in 0..list.len() {
        sum = sum.checked_add(list[i])?;
    }

    Some(sum)
}
