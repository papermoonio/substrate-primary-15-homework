fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut list = vec![1, 11, 2, 22, 55, 5, 7, 88];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut list = vec![1, 11, 2, 22, 55, 5, 7, 88];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 5, 7, 11, 22, 55, 88]);
    }
}
