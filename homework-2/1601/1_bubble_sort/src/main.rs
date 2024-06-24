fn bubble_sort_qestion_one<T: PartialOrd>(arr: &mut [T]) {
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
    println!("My id is 1601");
    let mut int_array = [0, -23, 5044, 33, 29, 292];
    println!("Before: {:?}", int_array);
    bubble_sort_qestion_one(&mut int_array);
    println!("After: {:?}", int_array);

    let mut float_array = [0.01, -23.02, 5044.0, 33.0333, 33.033201, 292.00];
    println!("Before: {:?}", float_array);
    bubble_sort_qestion_one(&mut float_array);
    println!("After: {:?}", float_array);

    let mut str_array = ["martin", "yeung", "rust", "substrate", "martinyeung"];
    println!("Unsorted char array: {:?}", str_array);
    bubble_sort_qestion_one(&mut str_array);
    println!("Sorted char array: {:?}", str_array);

    let mut char_array = ['s', 'u', 'b', 's', 't'];
    println!("Before: {:?}", char_array);
    bubble_sort_qestion_one(&mut char_array);
    println!("After: {:?}", char_array);
}

// main results:
/*
Before: [0, -23, 5044, 33, 29, 292]
After: [-23, 0, 29, 33, 292, 5044]
Before: [0.01, -23.02, 5044.0, 33.0333, 33.033201, 292.0]
After: [-23.02, 0.01, 33.033201, 33.0333, 292.0, 5044.0]
Unsorted char array: ["martin", "yeung", "rust", "substrate", "martinyeung"]
Sorted char array: ["martin", "martinyeung", "rust", "substrate", "yeung"]
Before: ['s', 'u', 'b', 's', 't']
After: ['b', 's', 's', 't', 'u']
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_int_array() {
        let mut array = [0, -23, 5044, 33, 29, 292];
        bubble_sort_qestion_one(&mut array);
        assert_eq!(array, [-23, 0, 29, 33, 292, 5044]);
    }

    #[test]
    fn test_bubble_sort_float_array() {
        let mut array = [0.01, -23.02, 5044.0, 33.0333, 33.033201, 292.0];
        bubble_sort_qestion_one(&mut array);
        assert_eq!(array, [-23.02, 0.01, 33.033201, 33.0333, 292.0, 5044.0]);
    }

    #[test]
    fn test_bubble_sort_str_array() {
        let mut array = ["martin", "yeung", "rust", "substrate", "martinyeung"];
        bubble_sort_qestion_one(&mut array);
        assert_eq!(array, ["martin", "martinyeung", "rust", "substrate", "yeung"]);
    }

    #[test]
    fn test_bubble_sort_char_array() {
        let mut array = ['s', 'u', 'b', 's', 't'];
        bubble_sort_qestion_one(&mut array);
        assert_eq!(array, ['b', 's', 's', 't', 'u']);
    }
}

// Test results:
/*
running 4 tests
test tests::test_bubble_sort_char_array ... ok
test tests::test_bubble_sort_float_array ... ok
test tests::test_bubble_sort_str_array ... ok
test tests::test_bubble_sort_int_array ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/