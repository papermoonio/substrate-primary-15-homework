mod summation;

fn main() {
    sum_test();
}


fn sum_test() {
    //未溢出
    print!("未溢出  ");
    let arr = &summation::build_rand_array(false);
    let res = summation::sum_all(arr);
    match res {
        Some(res) => println!("Sum is :{}", res), 
        None => println!("Sum overflows u32"),
    }
    //溢出 4,294,967,295
    print!("溢出  ");
    let arr1 = &summation::build_rand_array(true);
    let res1 = summation::sum_all(arr1);
    match res1 {
        Some(res1) => println!("Sum is :{}", res1), 
        None => println!("Sum overflows u32"),
    }
}