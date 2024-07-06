//基础版
fn bubble_sort_base(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

//提高版
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    //基础版
    println!("基础版sort");
    let mut arr = [5, 3, 8, 4, 2];
    println!("Before Sort: {:?}", arr);
    bubble_sort_base(&mut arr);
    println!("After Sort: {:?}", arr);

    //提高版
    println!("提高版sort");
    let mut arr = [5, 3, 8, 4, 2];
    println!("Before Sort: {:?}", arr);
    bubble_sort(&mut arr);
    println!("After Sort: {:?}", arr);

    let mut arr_float = [1.1, 3.3, 2.2, 4.4, 5.5];
    println!("Before Sort: {:?}", arr);
    bubble_sort(&mut arr_float);
    println!("After Sort: {:?}", arr);

    let mut arr_str = ["apple", "banana", "cherry", "date"];
    println!("Before Sort: {:?}", arr);
    bubble_sort(&mut arr_str);
    println!("After Sort: {:?}", arr);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_base_i32() {
        let mut arr = [5, 3, 8, 4, 2];
        bubble_sort_base(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_bubble_sort_i32() {
        let mut arr = [5, 3, 8, 4, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }
    
    #[test]
    fn test_bubble_sort_f32() {
        let mut arr = [1.1, 3.3, 2.2, 4.4, 5.5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    fn test_bubble_sort_str() {
        let mut arr = ["apple", "banana", "cherry", "date"];
        bubble_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "cherry", "date"]);
    }
}
