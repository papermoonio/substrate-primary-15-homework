use crate::bubble_sort;
#[test]
fn test_in_i32_list() {
    let mut arr = [-4, 2, -1, 3];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, [-4, -1, 2, 3]);
}

#[test]
fn test_in_f64_list() {
    let mut arr = [-4.5, 2.6, -1.1, 3.7];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, [-4.5, -1.1, 2.6, 3.7]);
}

#[test]
fn test_in_string_list() {
    let mut arr = ["ask","tear","rain","book"];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, ["ask","book","rain","tear"]);
}

#[test]
fn test_in_char_list() {
    let mut arr = ['k','u','r','t'];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, ['k','r','t','u']);
}