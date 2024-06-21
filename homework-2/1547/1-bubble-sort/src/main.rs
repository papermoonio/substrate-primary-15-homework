// fn bubble_sort_i32(arr: &mut [i32]) {
//     let len = arr.len();
//     for i in 0..len {
//         for j in 0..len - 1 - i {
//             if (arr[j] > arr[j + 1]) {
//                 arr.swap(j, j + 1);
//             }
//         }
//     }
// }

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if (arr[j] > arr[j + 1]) {
                arr.swap(j, j + 1);
            }
        }
    }
}


fn main() {
    println!("Hello, 1547!");
    let mut arr = [19, 34, 8, 13, 67, 1547, 67];
    println!("Unsorted : {:?}", arr);
    // bubble_sort_i32(&mut arr);
    bubble_sort(&mut arr);
    println!("Sorted : {:?}", arr);

    let mut float_arr = [19.3, 34.1, 8.4, 13.2, 67.1, 1547.0, 67.2];
    println!("Unsorted float array: {:?}", float_arr);
    bubble_sort(&mut float_arr);
    println!("Sorted float array: {:?}", float_arr);

    let mut char_array = ['b', 'c', 'd', 'a', 'f', 'e'];
    println!("Unsorted char array: {:?}", char_array);
    bubble_sort(&mut char_array);
    println!("Sorted char array: {:?}", char_array);

    let mut str1 = ["hello", "1547", "world"];
    println!("Unsorted char array: {:?}", str1);
    bubble_sort(&mut str1);
    println!("Sorted char array: {:?}", str1);
}
