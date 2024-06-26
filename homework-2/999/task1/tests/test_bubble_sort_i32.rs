use task1::bubble_sort_i32;


#[test]
fn test_bubble_sort_i32() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    bubble_sort_i32(&mut arr);
    assert_eq!(arr, [1, 2, 5, 5, 6, 9]);
}