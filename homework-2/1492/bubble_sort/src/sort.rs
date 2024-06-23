/// 非泛型冒泡排序
pub fn bubble_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

/// 泛型冒泡排序
pub fn bubble_sort_generic<T: PartialOrd>(list: &mut [T]) {
    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut array = [3, 2, 1, 5, 4, 6, 11, 0, 7, 17];
        bubble_sort(&mut array);
        assert_eq!(array, [0, 1, 2, 3, 4, 5, 6, 7, 11, 17]);
    }

    #[test]
    fn test_bubble_sort_generic() {
        let mut list: Vec<String> = vec![];
        list.push("one".to_string());
        list.push("two".to_string());
        list.push("three".to_string());
        list.push("four".to_string());
        list.push("five".to_string());

        bubble_sort_generic(&mut list);

        assert_eq!(list.join(" "), "five four one three two");
    }
}