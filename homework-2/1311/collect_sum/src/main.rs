/*第三题：实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None*/

fn collect_sum(slice: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in slice {
        if let Some(num1) = sum.checked_add(num){
            sum = num1;
        } else {
            return None;
        }
    }
    Some(sum)
}

fn main() {
    let nums = [11, 43, 66, 77, 99];
    let sum = collect_sum(&nums);
    match sum {
        Some(n) => println!("Sum : {}", n),
        None => println!("overflow!"),
    }

    let nums1 = [u32::MAX, 10];
    let sum1 = collect_sum(&nums1);
    match sum1 {
        Some(n) => println!("Sum : {}", n),
        None => println!("overflow!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_collect_sum() {
        let nums = [11, 43, 66, 77, 99];
        assert_eq!(sum_u32(&nums), Some(296));
    }

    #[test]
    fn test_collect_sum1() {
        let nums = [u32::MAX, 10];
        assert_eq!(sum_u32(&nums), None);
    }
}
