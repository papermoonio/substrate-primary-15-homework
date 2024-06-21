pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_i32() {
        let mut arr: Vec<i32> = vec![3, 2, 4, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }

    #[test]
    fn it_works_for_something_impls_partialord() {
        #[derive(Debug, PartialOrd, PartialEq)]
        struct Something {
            a: i32,
            b: i32,
        }

        let mut arr: Vec<Something> = vec![
            Something { a: 3, b: 2 },
            Something { a: 2, b: 3 },
            Something { a: 4, b: 1 },
            Something { a: 1, b: 4 },
        ];

        bubble_sort(&mut arr);
        assert_eq!(
            arr,
            vec![
                Something { a: 1, b: 4 },
                Something { a: 2, b: 3 },
                Something { a: 3, b: 2 },
                Something { a: 4, b: 1 }
            ]
        );
    }
}
