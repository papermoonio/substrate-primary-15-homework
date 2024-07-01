use crate::array_sum::array_sum;

#[test]
fn normal_pass_test() {
    let a = [1, 2, 3, 4, 5];
    assert_eq!(array_sum(&a), Some(15));
}

#[test]
fn overflow_test() {
    let a = [u32::MAX,1];
    assert_eq!(array_sum(&a), None);
}