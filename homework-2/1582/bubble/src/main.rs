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
    println!("Hello, world!");
    let mut int_array = [0, 1, -1, 2, 3, 1];
    bubble_sort_qestion_one(&mut int_array);

    let mut float_array = [0.01, -1.0, 1.0, 2.0, 3.0, 1.0];
    bubble_sort_qestion_one(&mut float_array);

    let mut str_array = ["my", "name", "is", "mark", "substrate"];
    bubble_sort_qestion_one(&mut str_array);

    let mut char_array = ['s', 'u', 'b', 's', 't','r','a','t','e'];
    bubble_sort_qestion_one(&mut char_array);
}
