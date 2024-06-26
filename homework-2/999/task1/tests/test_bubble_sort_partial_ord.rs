use task1::bubble_sort_partial_ord;

#[test]
fn test_bubble_sort_integers() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    bubble_sort_partial_ord(&mut arr);
    assert_eq!(arr, [1, 2, 5, 5, 6, 9]);
}

#[test]
fn test_bubble_sort_floats() {
    let mut arr = [3.3, 1.1, 2.2];
    bubble_sort_partial_ord(&mut arr);
    assert_eq!(arr, [1.1, 2.2, 3.3]);
}

#[test]
fn test_bubble_sort_chars() {
    let mut arr = ['b', 'a', 'd', 'c'];
    bubble_sort_partial_ord(&mut arr);
    assert_eq!(arr, ['a', 'b', 'c', 'd']);
}

#[test]
fn test_bubble_sort_empty() {
    let mut arr: [i32; 0] = [];
    bubble_sort_partial_ord(&mut arr);
    assert_eq!(arr, []);
}

#[test]
fn test_bubble_sort_single_element() {
    let mut arr = [1];
    bubble_sort_partial_ord(&mut arr);
    assert_eq!(arr, [1]);
}

#[test]
fn test_bubble_sort_already_sorted() {
    let mut arr = [1, 2, 3, 4, 5];
    bubble_sort_partial_ord(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn test_bubble_sort_reverse_order() {
    let mut arr = [5, 4, 3, 2, 1];
    bubble_sort_partial_ord(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}