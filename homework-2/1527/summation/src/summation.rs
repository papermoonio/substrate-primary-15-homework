use rand::Rng;

pub fn build_rand_array(param: bool) -> Vec<u32> {
    if !param {
        let mut rng = rand::thread_rng();
        let arr_len: u32 = rng.gen_range(1..15);
        println!("数组长度为：{}", arr_len);
        let res_arr: Vec<u32> = (0..arr_len).map(|_|rng.gen_range(0..100000000)).collect();  
        println!("数组为：{:?}", res_arr);
        return res_arr;
    } else {
        println!("数组长度为：{}", 5);
        let res_arr: Vec<u32> = [1000000000,2000000000,1000000000,1000000000,294967296].to_vec();
        println!("数组为：{:?}", res_arr);
        return res_arr;
    }
}

pub fn sum_all(list:&[u32]) -> Option<u32>{
    let res = list.iter().try_fold(0, |a:u32, &b| a.checked_add(b));
    return res;
}


#[cfg(test)]
#[test]
fn sum_test() {
    //未溢出
    print!("未溢出  ");
    let arr = &build_rand_array(false);
    let res = sum_all(arr);
    match res {
        Some(res) => println!("Sum is :{}", res), 
        None => println!("Sum overflows u32"),
    }
    //溢出 4,294,967,295
    print!("溢出  ");
    let arr1 = &build_rand_array(true);
    let res1 = sum_all(arr1);
    match res1 {
        Some(res1) => println!("Sum is :{}", res1), 
        None => println!("Sum overflows u32"),
    }
}
