fn main() {
    println!("Homework_2 - task_1:");

    let test_1=[23,33,12,67,18,88];
    println!("Orginal array: {:?}",test_1 );
    let mut i = 0;
    while i <= 5 {
        println!("#{}: {}",i, test_1[i]);
        i += 1;
    }

    //println!("Orginal array: {:?}",test_1 );
    //sort(test_1);
}

// fn sort(arr:vec![i32]){
//     println!("Orginal array: {:?}",arr );
// }