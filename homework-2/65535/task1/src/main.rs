fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", arr);
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);


    let mut arr_float = [3.1, 2.1, 5.1];
    println!("Unsorted array: {:?}", arr_float);
    bubble_sort(&mut arr_float);
    println!("Sorted array: {:?}", arr_float);

    let mut arr_str = ["test", "aa", "bb", "cc"];
    println!("Unsorted array: {:?}", arr_str);
    bubble_sort(&mut arr_str);
    println!("Sorted array: {:?}", arr_str);
}
//
// fn bubble_sort(arr: &mut [i32]) {
//     let n = arr.len();
//     for i in 0..n {
//         for j in 0..n - i - 1 {
//             if arr[j] > arr[j + 1] {
//                 arr.swap(j, j + 1);
//             }
//         }
//     }
// }

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
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
        let mut array = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut array);

        assert_eq!(array, [11, 12, 22, 25, 34, 64, 90]);
    }
}
