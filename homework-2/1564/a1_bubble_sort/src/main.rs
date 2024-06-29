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
    let mut arr = [5, 2, 9, 1, 3];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);

    let mut arr2 = ["banana", "apple", "cherry", "date"];
    bubble_sort(&mut arr2);
    println!("Sorted array: {:?}", arr2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        // 测试整数数组排序
        let mut arr1 = [5, 2, 9, 1, 3];
        bubble_sort(&mut arr1);
        assert_eq!(arr1, [1, 2, 3, 5, 9]);

        // 测试字符串数组排序
        let mut arr2 = ["banana", "apple", "cherry", "date"];
        bubble_sort(&mut arr2);
        assert_eq!(arr2, ["apple", "banana", "cherry", "date"]);

        // 测试空数组排序
        let mut arr3: [i32; 0] = [];
        bubble_sort(&mut arr3);
        assert_eq!(arr3, []);

        // 测试已排序数组排序
        let mut arr4 = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr4);
        assert_eq!(arr4, [1, 2, 3, 4, 5]);

        // 测试逆序数组排序
        let mut arr5 = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr5);
        assert_eq!(arr5, [1, 2, 3, 4, 5]);
    }
}
