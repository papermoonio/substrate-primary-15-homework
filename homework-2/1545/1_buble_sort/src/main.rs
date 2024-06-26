fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut nums = [5, 1, 3, 8, 6, 2, 4, 9, 7];
    println!("原始数组: {:?}", nums);
    bubble_sort(&mut nums);
    println!("排序后数组: {:?}", nums);

    let mut fnums = [2.3, 1.4, 3.2, 8.9];
    println!("原始数组: {:?}", fnums);
    bubble_sort(&mut fnums);
    println!("排序后数组: {:?}", fnums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        let mut array = [5, 1, 3, 8, 6, 2, 4, 9, 7];
        bubble_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_float() {
        let mut array = [2.3, 1.4, 3.2, 8.9];
        bubble_sort(&mut array);
        assert_eq!(array, [1.4, 2.3, 3.2, 8.9]);
    }
}

