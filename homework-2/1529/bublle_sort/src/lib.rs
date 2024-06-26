pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: [i32; 0] = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_sorted_array() {
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_array() {
        let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }
    
    #[test]
    fn test_string_array() {
        let mut arr = ["banana", "apple", "pear", "grape", "orange"];
        bubble_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "grape", "orange", "pear"]);
    }

    #[test]
    fn test_float_array() {
        let mut arr = [3.14, 2.71, 1.41, 1.73, 0.58];
        bubble_sort(&mut arr);
        assert_eq!(arr, [0.58, 1.41, 1.73, 2.71, 3.14]);
    }
}
