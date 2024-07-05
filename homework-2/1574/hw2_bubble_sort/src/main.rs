fn main() {
    let mut arr = [8, 99, 10, 67, 59, 66, 18, 41];
    bubble_sort(&mut arr);
    println!("shorted arr: {:?}", arr);
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in  0..n - 1 -i {
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
    fn test_bubble_sort() {
        let mut arr = [8, 99, 10, 67, 59, 66, 18, 41];
        bubble_sort(&mut arr);
        assert_eq!(arr, [8, 10, 18, 41, 59, 66, 67, 99]);
        let mut arr_float = [5.9, 5.1, 87.1, 22.2, 67.33];
        bubble_sort(&mut arr_float);
        assert_eq!(arr_float, [5.1, 5.9, 22.2, 67.33, 87.1]);
    }
}
