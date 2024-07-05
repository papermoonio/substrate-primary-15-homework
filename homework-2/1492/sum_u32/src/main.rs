mod sum;

use sum::{sum_v1, sum_v2};

fn main() {
    // 测试sum_v1
    println!("测试 sum_v1(0..1000)");
    let mut vector: Vec<u32> = (0..1000).collect();
    let mut sum = sum_v1(&vector);
    match sum {
        Some(v) => println!("sum_v1: {}", v),
        None => println!("sum_v1: overflow"),
    }
    println!("测试 sum_v1(u32::Max, 1)");
    vector = vec![u32::MAX, 1];
    sum = sum_v1(&vector);
    match sum {
        Some(v) => println!("sum_v1: {}", v),
        None => println!("sum_v1: overflow"),
    }

    println!("sum_v1 测试结束");

    println!("---------------------------------");

    println!("测试 sum_v2(0..1000)");
    vector = (0..1000).collect();
    sum = sum_v2(&vector);
    match sum {
        Some(v) => println!("sum_v2: {}", v),
        None => println!("sum_v2: overflow"),
    }

    println!("测试 sum_v2(u32::Max, 1)");
    vector = vec![u32::MAX, 1];
    sum = sum_v2(&vector);
    match sum {
        Some(v) => println!("sum_v2: {}", v),
        None => println!("sum_v2: overflow"),
    }
    println!("sum_v2 测试结束");

}
