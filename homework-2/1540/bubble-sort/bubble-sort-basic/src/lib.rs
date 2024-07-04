pub fn bubble_sort_basic(mut to_be_sorted: Vec<i32>) -> Vec<i32> {
    for i in (0..to_be_sorted.len()).rev() {
        for j in 0..i {
            if to_be_sorted[j] > to_be_sorted[i] {
                let a = to_be_sorted[j];
                to_be_sorted[j] = to_be_sorted[i];
                to_be_sorted[i] = a;
            }
        }
    }
    return to_be_sorted;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, thread_rng};

    #[test]
    pub fn bubble_sort_basic_should_work() {
        let mut rng = thread_rng();
        let to_be_sorted = (0..10).map(|_| {
            Rng::gen_range(&mut rng, 0..100)
        }).collect::<Vec<i32>>();

        println!("before sort:{:#?}", to_be_sorted);
        let after_sorted = bubble_sort_basic(to_be_sorted);
        println!("after sorted:{:#?}", after_sorted);
    }
}
