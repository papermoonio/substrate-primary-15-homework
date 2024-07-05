
fn bubble_sort1<T: PartialOrd + Copy + Clone>(mut vec: Vec<T>) -> Vec<T> {
    println!("strat");
    for i in (0..(vec.len() - 1)).rev() {
        for j in 0..vec.len() - i - 1 {
            if vec[j + 1] < vec[j] {
                let tmp = vec[j + 1];
                vec[j + 1] = vec[j];
                vec[j] = tmp;
            }
        }
    }
    vec
}

fn main() {
    let a = vec![25, 96 ,1 ,26, 54, 74, 32, 1002, 65];
    println!("sort result: {:?}", bubble_sort1(a));
}
