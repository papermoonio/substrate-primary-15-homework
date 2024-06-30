fn main() {
    println!("\n\nHomework-2 #task_1: Bubble Sort Algorithm");
    println!("冒泡算法，Rust支持中文输出。\n\n");
    
    //1.双循环的写法
    println!("双循环的写法：要先知道原理，才方便理解");
    let mut test_1:Vec<i32>=vec![23,33,12,67,18,88,31];
    let mut test_2:Vec<i32>=vec![1223,22334,1332,2567,1228,8118,3122,997,3382];
    sort_vector(&mut test_1);
    sort_vector(&mut test_2);

    //2.递归的写法
    println!("递归的写法：看代码，能理解咋回事");
    let mut self_1:Vec<i32>=vec![21,34,12,66,13,17,31];
    let mut self_2:Vec<i32>=vec![2223,7334,7332,5537,1224,7118,112,9917,3389];
    sort_self(&mut self_1,0);
    sort_self(&mut self_2,0);
}

fn sort_self(vec: &mut Vec<i32>,done:usize){
    if done==0 {
        println!("\n****** Start of sorting ******" );
        println!("Orginal array: {:?}",vec );
        println!("-------------------" );
    }else{
        if done==vec.len()-1 {
            println!("-------------------" );
            println!("Done: {:?}",vec );
            println!("****** End of sorting ******\n" );
            return ();
        }
    }

    let mut i = 0;
    let mut tmp:i32;
    let end = vec.len()-1-done;     
    while i < end {
        if vec[i+1] < vec[i] {
            tmp=vec[i];
            vec[i]=vec[i+1];
            vec[i+1]=tmp;
        }
        i += 1;
    }
    println!("Round[{}]: {:?}",done,vec );
    return sort_self(vec, done+1);
}

fn sort_vector(vec: &mut Vec<i32>) {
    println!("\n****** Start of sorting ******" );
    println!("Orginal array: {:?}",vec );
    println!("-------------------" );

    let mut i = 0;
    while i < vec.len()-1 {

        let mut j = 0;
        let mut tmp:i32;
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
    println!("****** End of sorting ******\n" );
}

//Unit test, run by `cargo test`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_01() {
        let expected = vec![12, 18, 23, 31, 33, 67, 88];
        let mut test_1:Vec<i32>=vec![23,33,12,67,18,88,31];
        sort_vector(&mut test_1);
        assert_eq!(test_1, expected);
    }

    #[test]
    fn test_sort_02() {
        let expected = vec![12, 18, 23, 31, 33, 67, 88];
        let mut test_1:Vec<i32>=vec![23,33,12,67,18,88,31];
        sort_self(&mut test_1,0);
        assert_eq!(test_1, expected);
    }
}