// 基础版本: 对i32类型数组进行排序
fn bubble_sort_i32(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 进阶版本: 使用泛型和PartialOrd trait实现对任意可比较类型的排序
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_i32() {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort_i32(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_bubble_sort_i32_empty() {
        let mut arr: [i32; 0] = [];
        bubble_sort_i32(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_bubble_sort_i32_already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort_i32(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_generic_i32() {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_bubble_sort_generic_char() {
        let mut arr = ['d', 'a', 'c', 'b', 'e'];
        bubble_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_bubble_sort_generic_string() {
        let mut arr = vec!["hello", "world", "rust", "programming"];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec!["hello", "programming", "rust", "world"]);
    }

    #[test]
    fn test_bubble_sort_generic_empty() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_bubble_sort_generic_already_sorted() {
        let mut arr = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }
}

fn main() {
    // 主函数中的测试代码保持不变
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    println!("原始数组: {:?}", numbers);
    bubble_sort_i32(&mut numbers);
    println!("排序后的数组: {:?}", numbers);

    let mut chars = ['d', 'a', 'c', 'b', 'e'];
    println!("原始字符数组: {:?}", chars);
    bubble_sort(&mut chars);
    println!("排序后的字符数组: {:?}", chars);
}