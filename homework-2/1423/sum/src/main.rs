fn main() {
    let arr1:[u32;5]=[1,2,3,5,6];
    let arr2:[u32;5]=[1000000000,3000000000,4000000000,200000,50000000];
    let sum1 =add(&arr1);
    match sum1 {
        Some(result)=>println!("The result is {}",result),
        None=>println!("The result is overflow"),
    }
    let sum2 =add(&arr2);
    match sum2 {
        Some(result)=>println!("The result is {}",result),
        None=>println!("The result is overflow"),
    }
}

fn add(a:&[u32])->Option<u32>{
    let mut sum:u32=0;
    for i in 0..a.len(){
       let(result,ifoverflow)=sum.overflowing_add(a[i]);
       if ifoverflow{
          return None;
       }else{
        sum=result;
       }
    }
    return Some(sum);
}

#[cfg(test)]
#[test]
fn test_add1(){
    let a:[u32;5]=[1,2,3,5,6];
    let b:[u32;5]=[1000000000,3000000000,4000000000,200000,50000000];
    let sum1 =add(&a);
    let sum2=add(&b);
    assert_eq!(sum1,Some(17));
    assert_eq!(sum2,None);
}

