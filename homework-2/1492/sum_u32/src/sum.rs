/// 不使用 checked_add 方法求和
pub fn sum_v1(group: &[u32]) -> Option<u32> {
    let max = u32::MAX as u64;

    if group.is_empty() {
        return None;
    }

    let mut sum = 0_u64;
    for v in group.iter() {
        sum += *v as u64;
        if sum > max {
            return None;
        }
    }

    Some(sum as u32)
}

/// 使用 checked_add 方法求和
pub fn sum_v2(group: &[u32]) -> Option<u32> {
    if group.is_empty() {
        return None;
    }
    let mut result = Some(0_u32);

    for g in group.iter() {
        result = result.and_then(|v| v.checked_add(*g));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_v1() {
        let mut vector: Vec<u32> = (0..1000).collect();
        let mut sum = sum_v1(&vector);
        match sum {
            Some(v) => assert_eq!(v, 499500),
            None => panic!("sum_v1: overflow"), // 本例如果执行到None分支，说明程序错误，而非溢出
        }

        vector = vec![u32::MAX, 1];
        sum = sum_v1(&vector);
        match sum {
            Some(v) => panic!("sum_v1: {}", v),
            None => println!("sum_v1: overflow"),
        }
    }

    #[test]
    fn test_sum_v2() {
        let mut vector: Vec<u32> = (0..1000).collect();
        let mut sum = sum_v2(&vector);
        match sum {
            Some(v) => assert_eq!(v, 499500),
            None => panic!("sum_v2: overflow"), // 本例如果执行到None分支，说明程序错误，而非溢出
        }

        vector = vec![u32::MAX, 1];
        sum = sum_v2(&vector);
        match sum {
            Some(v) => panic!("sum_v2: {}", v),
            None => println!("sum_v2: overflow"),
        }
    }
}