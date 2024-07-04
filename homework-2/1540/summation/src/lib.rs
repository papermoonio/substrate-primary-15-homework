pub fn summarize(target: &[u32]) -> Option<u32> {
    // let mut result = 0u64;
    // for element in target.iter() {
    //     result += *element as u64;
    // }
    // 
    // println!("result:     {}", result);
    // println!("u32::MAX:   {}", u32::MAX);
    // 
    // return if result > u32::MAX as u64 {
    //     None
    // } else {
    //     Some(result as u32)
    // };
    let result: u64 = target.iter().map(|&x| u64::from(x)).sum();

    println!("result:     {}", result);
    println!("u32::MAX:   {}", u32::MAX);

    u32::try_from(result).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn summarize_should_works() -> Result<(), String> {
        let mut rng = thread_rng();

        let target = (0..10)
            .map(|_| Rng::gen_range(&mut rng, 0..u32::MAX / 5))
            .collect::<Vec<u32>>();

        let target = target.as_slice();

        let result = summarize(target);
        match result {
            Some(_) => Ok(()),
            None => Err("结果溢出，返回为None".into()),
        }
    }

    #[test]
    pub fn test() {
        let rounds = 10;
        let mut normal = 0;
        let mut error = 0;
        (0..rounds).for_each(|_| {
            let result = summarize_should_works();
            match result {
                Ok(_) => normal += 1,
                Err(_) => error += 1,
            }
        });
        println!("======================================");

        println!("总共计算{}轮次", rounds);
        println!("正常输出{}轮", normal);
        println!("溢出{}轮", error);
    }
}
