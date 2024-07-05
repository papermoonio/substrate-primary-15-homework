fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &num in numbers {
        match sum.checked_add(num) {
            Some(s) => sum = s,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    //test
    let numbers: [u32; 5] = [15, 45, 7, 55, 41];
    match sum_u32(&numbers) {
        Some(sum) => println!("和为: {}", sum),
        None => println!("溢出"),
    }
}
