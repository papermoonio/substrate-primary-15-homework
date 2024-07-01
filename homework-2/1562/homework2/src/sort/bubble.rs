// Copyright (c) david Technologies Co.Ltd. 2015-2022. 
// All rights reserved. Licensed under Apache-2.0.


/// 冒泡排序算法，对传入的可变数组进行排序。
///
/// # Arguments
///
/// * `arr` - 要排序的可变数组，类型为 `&mut [T]`，其中 `T` 是实现了 `PartialOrd` trait 的类型。
///
/// # Examples
///
/// ```
/// use homework2::sort::bubble::bubble_sort;
/// let mut arr = [64, 34, 25, 12, 22, 11, 90];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
/// ```
///
pub fn bubble_sort(arrs: &mut[i32]){
    let arr_len = arrs.len();
    for i in 00..arr_len {
        let mut swapped = false;
        for j in 0..arr_len - i - 1 {
            if arrs[j] > arrs[j+1] {
                arrs.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
    #[test]
    fn test_bubble_sort() {
        let mut test_arrs = vec![4, 3, 2, 1];
        bubble_sort(&mut test_arrs);
        assert_eq!(test_arrs, vec![1, 2, 3, 4]);
    }
}