
// Basic: Fixed Type (i32) Array Sorting
// fn bubble_sort(arr: &mut [i32]) {
//     let len = arr.len();
//     for i in 0..len {
//         for j in 0..len - 1 - i {
//             if arr[j] > arr[j + 1] {
//                 arr.swap(j, j + 1);
//             }
//         }
//     }
// }

// fn main() {
//     let mut array = [76, 32, 25, 58, 15, 2, 22, 4, 99];
//     println!("Original array: {:?}", array);
//     bubble_sort(&mut array);
//     println!("Sorted array: {:?}", array);
// }

// Basic result:
// Original array: [76, 32, 25, 58, 15, 2, 22, 4, 99]
// Sorted array: [2, 4, 15, 22, 25, 32, 58, 76, 99]

// ============================================================

// Advanced: Using Generics and PartialOrd for Sorting Any Type
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
    let mut int_array = [76, 32, 25, 58, 15, 2, 22, 4, 99];
    println!("Original array: {:?}", int_array);
    bubble_sort(&mut int_array);
    println!("Sorted array: {:?}", int_array);

    let mut float_array = [76.0, 32.5, 25.1, 58.0, 15.4, 2.8, 22.4, 4.9, 99.2];
    println!("Original array: {:?}", float_array);
    bubble_sort(&mut float_array);
    println!("Sorted array: {:?}", float_array);

    let mut char_array = ['d', 'a', 'c', 'b', 'e'];
    println!("Original array: {:?}", char_array);
    bubble_sort(&mut char_array);
    println!("Sorted array: {:?}", char_array);
}

// Advanced result:
// Original array: [76, 32, 25, 58, 15, 2, 22, 4, 99]
// Sorted array: [2, 4, 15, 22, 25, 32, 58, 76, 99]
// Original array: [76.0, 32.5, 25.1, 58.0, 15.4, 2.8, 22.4, 4.9, 99.2]
// Sorted array: [2.8, 4.9, 15.4, 22.4, 25.1, 32.5, 58.0, 76.0, 99.2]
// Original array: ['d', 'a', 'c', 'b', 'e']
// Sorted array: ['a', 'b', 'c', 'd', 'e']

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_integers() {
        let mut array = [5, 3, 4, 1, 2];
        bubble_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_floats() {
        let mut array = [3.2, 1.5, 4.8, 2.3];
        bubble_sort(&mut array);
        assert_eq!(array, [1.5, 2.3, 3.2, 4.8]);
    }

    #[test]
    fn test_bubble_sort_chars() {
        let mut array = ['z', 'a', 'q', 'b'];
        bubble_sort(&mut array);
        assert_eq!(array, ['a', 'b', 'q', 'z']);
    }
}

// Test result:
// running 3 tests
// test tests::test_bubble_sort_chars ... ok
// test tests::test_bubble_sort_floats ... ok
// test tests::test_bubble_sort_integers ... ok

// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
