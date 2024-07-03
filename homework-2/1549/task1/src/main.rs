fn main() {
}
// i32冒泡排序
// fn bubble_sort(arr: &mut [i32]){
//     let n = arr.len();
//     for i in 0..n {
//         for j in 0..n - i -1 {
//             if arr[j] > arr[j+1]{
//                 arr.swap(j,j+1);
//             }
//         }
//     }
// }

//泛型冒泡排序
fn bubble_sort<T: PartialOrd>(arr: &mut[T]){
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i -1 {
            if arr[j] > arr[j+1]{
                arr.swap(j,j+1);
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        let mut arr = [1,2,1,3];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2,3]);
    }

    #[test]
    fn test_float() {
        let mut arr = [1.1,2.2,3.1,2.1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1.1,2.1,2.2,3.1]);
    }

    #[test]
    fn test_strings() {
        let mut arr = ["banana", "apple", "cherry"];
        bubble_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "cherry"]);
    }
}