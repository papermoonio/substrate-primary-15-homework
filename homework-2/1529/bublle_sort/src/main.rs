use bublle_sort::bubble_sort;

fn main() {
    let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
