fn main() {
    println!("Hello, world!");
    let mut arr = vec![10, 11, 5, 4, 8, 7, 6, 9];
    let mut arr0 = vec![10, 11, 5, 4, 8, 7, 6, 9];
    let mut arr1 = vec![10.0, 11.0, 5.0, 4.0, 8.8, 7.7, 6.5, 9.2];
    let mut arr2 = vec!["a", "f", "c", "b", "g", "m", "d", "e"];
    let mut arr3 = vec!['a', 'f', 'c', 'b', 'g', 'm', 'd', 'e'];
    bubble_sort_u32(&mut arr);
    bubble_sort(&mut arr0);
    bubble_sort(&mut arr1);
    bubble_sort(&mut arr2);
    bubble_sort(&mut arr3);
    println!("bubble_sort_u32");
    println!("arr: {:?}", arr);
    println!("bubble_sort");
    println!("arr0: {:?}", arr0);
    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("arr3: {:?}", arr3);

    // result:
    // Hello, world!
    // bubble_sort_u32
    // arr: [4, 5, 6, 7, 8, 9, 10, 11]
    // bubble_sort
    // arr0: [4, 5, 6, 7, 8, 9, 10, 11]
    // arr1: [4.0, 5.0, 6.5, 7.7, 8.8, 9.2, 10.0, 11.0]
    // arr2: ["a", "b", "c", "d", "e", "f", "g", "m"]
    // arr3: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'm']
}

#[warn(dead_code)]
fn bubble_sort_u32(arr: &mut Vec<u32>) {
    // 首先讲数组长度计算出来 以免多次计算
    let len = arr.len();
    // 第一层循环：从第2位开始到数组的最后一位（更多的是表示循环多少轮）
    for i in 1..len {
        // 第二层循环：从第1位开始到数组的已经排好的那一位（即第一层循环的轮数）
        for j in 0..len - i {
            // 比较前后两数大小 前者大于后者则将两者进行交换
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort<T>(arr: &mut Vec<T>)
where
    T: PartialOrd,
{
    // 首先讲数组长度计算出来 以免多次计算
    let len = arr.len();
    // 第一层循环：从第2位开始到数组的最后一位（更多的是表示循环多少轮）
    for i in 1..len {
        // 第二层循环：从第1位开始到数组的已经排好的那一位（即第一层循环的轮数）
        for j in 0..len - i {
            // 比较前后两数大小 前者大于后者则将两者进行交换
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
