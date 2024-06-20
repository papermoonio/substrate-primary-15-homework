use sum_u32::sum_u32;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred"),
    }
    
    let large_numbers = vec![u32::MAX, 1];
    
    match sum_u32(&large_numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred"),
    }
}
