fn main() {
    println!("Hello, world!");
    let mut arr: Vec<i32> = vec![2, 13, 5, 11, 3];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![2, 3, 5, 11, 13]);
}

pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
