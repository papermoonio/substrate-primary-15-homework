use task1::{bubble_sort_i32, bubble_sort_partial_ord};

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort_i32(&mut arr);
    println!("Sorted array: {:?}", arr);

    let mut int_arr = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort_partial_ord(&mut int_arr);
    println!("Sorted int array: {:?}", int_arr);

    let mut float_arr = [64.0, 34.5, 25.1, 12.6, 22.9, 11.2, 90.8];
    bubble_sort_partial_ord(&mut float_arr);
    println!("Sorted float array: {:?}", float_arr);

    let mut char_arr = ['z', 'a', 'b', 'y', 'x', 'w', 'v'];
    bubble_sort_partial_ord(&mut char_arr);
    println!("Sorted char array: {:?}", char_arr);
}
