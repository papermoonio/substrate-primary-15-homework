fn main() {
    println!("Homework_2 - task_1:");

    let mut test_1:Vec<i32>=vec![23,33,12,67,18,88,31];
    sort_vector(&mut test_1)
}

fn sort_vector(vec: &mut Vec<i32>) {
    println!("****** Start of sorting ******" );
    println!("Orginal array: {:?}",vec );
    println!("-------------------" );
    let mut i = 0;
    let mut tmp:i32;
    while i < vec.len()-1 {
        //println!("Index: {}",i);
        let mut j = 0;
        while j < vec.len()-i-1 {
            if vec[j+1] < vec[j] {
                tmp=vec[j];
                vec[j]=vec[j+1];
                vec[j+1]=tmp;
            }
            j += 1;
        }
        println!("Round[{}]: {:?}",i,vec);
        i += 1;
    }

    println!("-------------------" );
    println!("Done: {:?}",vec );
    println!("****** End of sorting ******" );
}