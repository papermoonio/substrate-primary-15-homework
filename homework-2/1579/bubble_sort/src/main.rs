

fn bubble_sort<T: PartialOrd + Copy>(arr: Vec<T>) ->Vec<T> {
    let mut arr = arr;
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n-i-1){
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
    arr
}

fn main() {
    //整数
    let nums = vec![35,13,435,41,9,5];
    println!("排序前: {:?}", nums);
    println!("排序之后的结果：{:?}",bubble_sort(nums));
    //小数
    let floats = vec![34.1,0.431,2.1,455.231,5.0,4.5];
    println!("小数排序前：{:?}",floats);
    println!("排序后的结果：{:?}",bubble_sort(floats));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort_integers() {
        let array = vec![5, 3, 4, 1, 2];
        let result = bubble_sort(array);
        assert_eq!(result,[1,2,3,4,5]);
    }

    #[test]
    fn test_bubble_sort_floats() {
        let array = vec![3.2, 1.5, 4.8, 2.3];
        let result = bubble_sort( array);
        assert_eq!(result, [1.5, 2.3, 3.2, 4.8]);
    }
}