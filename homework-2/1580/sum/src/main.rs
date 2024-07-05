

fn sum(a: &[u32]) -> Option<u32> {
    let mut result:u32 = 0;
    for i in a {
        match result.checked_add(*i) {
            Some(t) => result = t,
            None => return  None,
        }
    } 
    Some(result)
}

fn main() {
    let a = vec![25, 96 ,1 ,26, 54, 74, 32, 1002, 65];
    println!("vector a's sum result: {}", sum(&a[0..a.len()]).unwrap());
    let b = vec![u32::MAX, 1];
    print!("vector b's sum result is: ");
    match sum(&b[0..b.len()]) {
        Some(num) => println!("{}", num),
        None => println!("is addition overflow"),
    }
}
