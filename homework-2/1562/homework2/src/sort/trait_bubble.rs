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
/// use homework2::sort::trait_bubble::bubble_sort2;
/// let mut arr2 = [64, 34, 25, 12, 22, 11, 90];
/// bubble_sort2(&mut arr2);
/// assert_eq!(arr2, [11, 12, 22, 25, 34, 64, 90]);
/// ```
///
pub fn bubble_sort2<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    
    for i in 0..n {
        let mut swapped = false;
        
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
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
    use super::bubble_sort2;
    #[test]
    fn test_bubble_sort2() {
        let mut test_arrs2 = vec![4, 3, 2, 1];
        bubble_sort2(&mut test_arrs2);
        assert_eq!(test_arrs2, vec![1, 2, 3, 4]);
    }
}