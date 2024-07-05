fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        let mut a = [3, 6, 9, 0, 11];
        let mut s = [ "bob", "charlie", "alice"];
        super::bubble_sort(&mut a);
        super::bubble_sort(&mut s);

        assert_eq!(a, [0, 3, 6, 9, 11]);
        assert_eq!(s, ["alice", "bob", "charlie"]);
    }
}