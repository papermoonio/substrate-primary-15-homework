fn main() {
    // i32固定类型冒泡排序
    let mut arr = [34, 23, 66, 77, 3, 1, 44];
    println!("i32固定类型排序前数组：{:?}",arr);
    bubble_sort(&mut arr);
    println!("i32固定类型排序后数组：{:?}",arr);
    
    // 泛型冒泡排序算法-float类型
    let mut float_arr = [14.0, 23.2, 55.5, 22.87, 22.88, 11.3, 90.6];
    println!("泛型冒泡排序前数组：{:?}", float_arr);
    bubble_sort_generic(&mut float_arr);
    println!("泛型冒泡排序后数组{:?}", float_arr);

}

// i32固定类型冒泡排序算法
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
// 泛型冒泡排序算法
fn bubble_sort_generic<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 测试冒泡排序
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // 固定类型冒泡函数测试
    fn test_bubble_sort_i32() {
        let mut arr = [34, 23, 66, 77, 3, 1, 44];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 3, 23, 34, 44, 66, 77]);
    }
    #[test]
    // 泛型冒泡函数测试
    fn test_bubble_sort_generic(){
        let mut float_arr = [14.0, 23.2, 55.5, 22.87, 22.88, 11.3, 90.6];
        bubble_sort_generic(&mut float_arr);
        assert_eq!(float_arr,[11.3, 14.0, 22.87, 22.88, 23.2, 55.5, 90.6]);
    }
}
