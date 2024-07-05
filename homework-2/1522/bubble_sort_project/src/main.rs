// basic sort logic
fn bubble_sort_basic(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
}

//PartialOrd sort fun
fn bubble_sort_generic<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_basic() {
        let mut nums = [5, 2, 9, 1, 5, 6];
        bubble_sort_basic(&mut nums);
        assert_eq!(nums, [1, 2, 5, 5, 6, 9]);
    }
    #[test]
    fn test_bubble_sort_i32() {
        let mut nums = [5, 2, 9, 1, 5, 6];
        bubble_sort_generic(&mut nums);
        assert_eq!(nums, [1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_bubble_sort_f64() {
        let mut nums = [3.1, 2.2, 5.5, 1.0, 4.7];
        bubble_sort_generic(&mut nums);
        assert_eq!(nums, [1.0, 2.2, 3.1, 4.7, 5.5]);
    }

    #[test]
    fn test_bubble_sort_string() {
        let mut strs = ["apple", "orange", "banana", "grape"];
        bubble_sort_generic(&mut strs);
        assert_eq!(strs, ["apple", "banana", "grape", "orange"]);
    }
}
fn main() {
    // basic sort logic
    let mut nums_b = [5, 123, 3, 21, 30];
    println!("before Sort{:?}", nums_b);
    bubble_sort_basic(&mut nums_b);
    println!("after Sort{:?}", nums_b);

    //PartialOrd sort logic
    let mut nums_g = [5, 20, 9, 1, 5, 6];
    println!("before Sorted i32 array: {:?}", nums_g);
    bubble_sort_generic(&mut nums_g);
    println!("after Sorted i32 array: {:?}", nums_g);

    let mut floats = [3.1, 2.2, 5.5, 1.0, 4.7];
    println!("before Sorted f64 array: {:?}", floats);
    bubble_sort_generic(&mut floats);
    println!("after Sorted f64 array: {:?}", floats);

    let mut strs = ["car", "bike", "computer", "cup"];
    println!("before Sorted string array: {:?}", strs);
    bubble_sort_generic(&mut strs);
    println!("after Sorted string array: {:?}", strs);
}
