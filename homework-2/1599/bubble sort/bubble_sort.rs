fn main(){
    //test
    let mut arr:Vec<i32>=vec![1,5,8,9,7,4,6];
    bubble_sort_advance(&mut arr);
    for i in arr.iter(){
        println!("{} ",i);
    }
}
//basic requirement
fn bubble_sort_basic(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
//advance requirement
fn bubble_sort_advance<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}