
fn main() {
    println!("Hello, world!");
    let numbers_normal = vec![1, 2, 3];
    match sum_new(&numbers_normal) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }

    let numbers_overflow = vec![1, u32::MAX, 2];
    match sum_new(&numbers_overflow) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow!"),
    }
}



fn sum_new(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0_u32;
    for &number in numbers {
        match sum.checked_add(number) {
            Some(tmp_sum) => sum = tmp_sum,
            None => return None,
        }
    }
    Some(sum)
}
