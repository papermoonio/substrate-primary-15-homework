/*第一题：使用Rust语言写一个冒泡排序的算法
基础要求：固定类型（比如i32）的数组排序
提高部分：能够使用范型和PartialOrd实现对任意类型的排序*/

fn bubble_sort_i32(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len.saturating_sub(1) {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut nums = [33, 12, 66, 78, 99, 145, 666];
    println!("原数组: {:?}", nums);
    bubble_sort_i32(&mut nums);
    println!("新数组: {:?}", nums);

    let mut float_arr = [3.3, 1.2, 6.6, 78.9, 9.9, 14.5, 6.66];
    println!("原数组: {:?}", float_arr);
    bubble_sort(&mut float_arr);
    println!("新数组: {:?}", float_arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testi32() {
        let mut nums = [33, 12, 66, 78, 99, 145, 666];
        bubble_sort(&mut nums);
        assert_eq!(nums, [12, 33, 66, 78, 99, 145, 666]);
    }

    #[test]
    fn test_float() {
        let mut float_arr = [3.3, 1.2, 6.6, 78.9, 9.9, 14.5, 6.66];
        bubble_sort(&mut float_arr);
        assert_eq!(float_arr, [1.2, 3.3, 6.6, 6.66, 9.9, 14.5, 78.9]);
    }
}