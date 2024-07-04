pub fn bubble_sort_boost<T: PartialOrd>(mut to_be_sorted: Vec<T>) -> Vec<T> {
    for i in (0..to_be_sorted.len()).rev() {
        for j in 0..i {
            if to_be_sorted[j] > to_be_sorted[i] {
                to_be_sorted.swap(j, i);
            }
        }
    }
    to_be_sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    pub fn bubble_sort_boost_should_work() {
        let mut rng = thread_rng();
        let to_be_sorted = (0..10)
            .map(|_| Rng::gen_range(&mut rng, 1..100))
            .collect::<Vec<i32>>();

        println!("before sort:{:#?}", to_be_sorted);
        let after_sorted = bubble_sort_boost(to_be_sorted);
        println!("after sorted:{:#?}", after_sorted);
    }
}
