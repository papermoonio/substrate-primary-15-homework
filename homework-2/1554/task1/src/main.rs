
// basic fn
pub fn bubble_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let size = arr.len();
    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;

        // 最后i个元素已经排好了顺序
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] { 
                arr.swap(j - 1, j);
                swapped = true;
            }
        }

        // 如果当前循环没有发生元素交换，说明数组已经有序
        if !swapped {
            break;
        }
    }
} 

// 进阶版 泛型的使用
pub fn bubble_sort2<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let size = arr.len();
    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;

        // 最后i个元素已经排好了顺序
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] { 
                arr.swap(j - 1, j);
                swapped = true;
            }
        }

        // 如果当前循环没有发生元素交换，说明数组已经有序
        if !swapped {
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        bubble_sort2(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}


fn main() {
    println!("Hello, world!");
}

