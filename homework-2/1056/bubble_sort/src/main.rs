fn bubble_sort_basic(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
 }

fn bubble_sort_generic<T:PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut a = [4, 3, 11, 33, 22];
    println!("Unsorted array:{:?}", a);
    bubble_sort_basic(&mut a);
    println!("Sorted array:{:?}", a);

    let mut b = [4, 3, 11, 33, 22];
    println!("Unsorted array:{:?}", b);
    bubble_sort_generic(&mut b);
    println!("Sorted array:{:?}", b);

    let mut c = [4.1, 3.5, 11.3, 33.3, 22.5];
    println!("Unsorted array:{:?}", c);
    bubble_sort_generic(&mut c);
    println!("Sorted array:{:?}", c);

    let mut d = ["cc", "aa", "dd", "bb", "ee"];
    println!("Unsorted array:{:?}", d);
    bubble_sort_generic(&mut d);
    println!("Sorted array:{:?}", d);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_basic() {
        let mut a = [4, 3, 11, 33, 22];
        bubble_sort_basic(&mut a);
        assert_eq!(a, [3, 4, 11, 22, 33]);
    }

    #[test]
    fn test_bubble_sort_generic() {
        let mut a = [4, 3, 11, 33, 22];
        bubble_sort_generic(&mut a);
        assert_eq!(a, [3, 4, 11, 22, 33]);

        let mut b = [4.1, 3.5, 11.3, 33.3, 22.5];
        bubble_sort_generic(&mut b);
        assert_eq!(b, [3.5, 4.1, 11.3, 22.5, 33.3]);

        let mut c = ["cc", "aa", "dd", "bb", "ee"];
        bubble_sort_generic(&mut c);
        assert_eq!(c, ["aa", "bb", "cc", "dd", "ee"]);
    }

}
