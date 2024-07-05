fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut n = arr.len();
    let mut swapped = true; // 用于标记是否发生了交换

    while swapped {
        swapped = false; // 每轮都重置标记
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort_empty() {
        let mut vec: Vec<i32> = Vec::new();
        bubble_sort(&mut vec);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_bubble_sort_single_element() {
        let mut vec = vec![1];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_bubble_sort_already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_reversed() {
        let mut vec = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_random() {
        let mut vec = vec![9, -3, 5, 2, 6, 8, 1, 3];
        bubble_sort(&mut vec);
        let mut expected = vec![1, -3, 2, 3, 5, 6, 8, 9];
        expected.sort(); // 使用Vec自带的sort方法进行排序
        assert_eq!(vec, expected);
    }

    // 测试排序字符串
    #[test]
    fn test_bubble_sort_strings() {
        let mut vec = vec!["orange", "apple", "banana", "grape"];
        bubble_sort(&mut vec);
        let mut expected = vec!["apple", "banana", "grape", "orange"];
        assert_eq!(vec, expected);
    }
}

fn main() {
    let mut nums = [9, -3, 5, 2, 6, 8, 1, 3];
    println!("原始数组: {:?}", nums);
    bubble_sort(&mut nums);
    println!("排序后数组: {:?}", nums);

    let mut fnums = [2.3, -1.4, 3.2, 8.9];
    println!("原始数组: {:?}", fnums);
    bubble_sort(&mut fnums);
    println!("排序后数组: {:?}", fnums);
}
