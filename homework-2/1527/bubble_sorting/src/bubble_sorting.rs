use std::fmt::Debug;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

fn build_rand_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let arr_len = rng.gen_range(5..15);
    println!("数组长度为：{}", arr_len);
    let res_arr: Vec<i32> = (0..arr_len).map(|_|rng.gen_range(0..1000)).collect();
    return res_arr;
}

fn asc(mut original: Vec<i32>) {
    for i in 0..original.len()  {
        for j in i+1..original.len() {
            if original[i]>original[j] {
                let swap = original[i];
                original[i] = original[j];
                original[j] = swap;
            }
        }
    }
    println!("升序数组：{:?} ",original);
}

fn desc(mut original: Vec<i32>) {
    for i in 0..original.len()  {
        for j in i+1..original.len() {
            if original[i]<original[j] {
                let swap = original[i];
                original[i] = original[j];
                original[j] = swap;
            }
        }
    }
    println!("降序数组：{:?} ",original);
}

//泛型 指定泛型生成 随机数组
// fn build_rand_array_generic_paradigm<T: PartialOrd>() -> Vec<T> {
//     // 创建线程随机数生成器
//     let mut rng = rand::thread_rng();
//     let arr_len = rng.gen_range(5..15);
//     println!("数组长度为：{}", arr_len);
//     let mut rng_gp = rand::thread_rng();
//     let result_arr = (0..arr_len).map(|_| rng_gp.sample(Standard)).collect();

//     return result_arr;
// }

fn asc_paradigm<T:PartialOrd + Debug + Copy>(mut original: Vec<T>) {
    for i in 0..original.len()  {
        for j in i+1..original.len() {
            if original[i]>original[j] {
                let swap = original[i];
                original[i] = original[j];
                original[j] = swap;
            }
        }
    }
    println!("升序数组：{:?} ",original);
}

fn desc_paradigm<T:PartialOrd + Debug + Copy>(mut original: Vec<T>) {
    for i in 0..original.len()  {
        for j in i+1..original.len() {
            if original[i]<original[j] {
                let swap = original[i];
                original[i] = original[j];
                original[j] = swap;
            }
        }
    }
    println!("升序数组：{:?} ",original);
}

pub fn bubble_sorting_i32() {
    let original_array: Vec<i32> = build_rand_array();
    println!("原始数组：{:?} ",original_array);

    asc(original_array.clone());
    desc(original_array.clone());
}

#[cfg(test)]
#[test]
pub fn bubble_sorting_generic_paradigm() {
    let i32_original_array: Vec<i32> = [1112,232,45,6,78,2,12,1,55,5245,55,544,446,483,897].to_vec();
    println!("i32_原始数组：{:?} ",i32_original_array);
    asc_paradigm(i32_original_array.clone());
    desc_paradigm(i32_original_array.clone());

    let i8_original_array: Vec<i8> = [11,111,45,6,78,2,12,1,55,108,55,14,25,78].to_vec();
    println!("i8_原始数组：{:?} ",i8_original_array);
    asc_paradigm(i8_original_array.clone());
    desc_paradigm(i8_original_array.clone());

    let f32_original_array: Vec<f32> = [11.00,564.14,45.25,6.65,78.478,2.6,12.47,1.0,55.01,3154.11,55.21,55.11,848.4,483.5,4567.77].to_vec();
    println!("f32_原始数组：{:?} ",f32_original_array);
    asc_paradigm(f32_original_array.clone());
    desc_paradigm(f32_original_array.clone());
}

pub fn bubble_sorting_generic_by_main() {
    let i32_original_array: Vec<i32> = [1112,232,45,6,78,2,12,1,55,5245,55,544,446,483,897].to_vec();
    println!("i32_原始数组：{:?} ",i32_original_array);
    asc_paradigm(i32_original_array.clone());
    desc_paradigm(i32_original_array.clone());

    let i8_original_array: Vec<i8> = [11,111,45,6,78,2,12,1,55,108,55,14,25,78].to_vec();
    println!("i8_原始数组：{:?} ",i8_original_array);
    asc_paradigm(i8_original_array.clone());
    desc_paradigm(i8_original_array.clone());

    let f32_original_array: Vec<f32> = [11.00,564.14,45.25,6.65,78.478,2.6,12.47,1.0,55.01,3154.11,55.21,55.11,848.4,483.5,4567.77].to_vec();
    println!("f32_原始数组：{:?} ",f32_original_array);
    asc_paradigm(f32_original_array.clone());
    desc_paradigm(f32_original_array.clone());
}