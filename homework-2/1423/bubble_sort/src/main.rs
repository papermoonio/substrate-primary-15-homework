fn main() {
    let mut arr1=[1,2,6,4,9,3];
    let mut arr2=['a','z','d','k','h'];
    let mut arr3=[1.2,4.67,2.9,7.8,3.5];
    sort1(&mut arr1);
    print!("arr1 is:");
    for i in arr1{
        print!("{i} ");
    }
    println!();
    sort2(&mut arr2);
    print!("arr2 is:");
    for i in arr2{
        print!("{i} ");
    }
    println!();
    sort2(&mut arr3);
    print!("arr3 is:");
    for i in arr3{
        print!("{i} ");
    }
}
// basic part
fn sort1(a: &mut[i32]){
  for i in 0..a.len(){
    for j in 0..a.len()-1-i{
        if a[j]>a[j+1]{
            a.swap(j,j+1);
        }
  }
}
}

// raising part
fn sort2<T:PartialOrd>(a: &mut[T]){
    for i in 0..a.len(){
        for j in 0..a.len()-1-i{
            if a[j]>a[j+1]{
                a.swap(j,j+1);
            }
      }
}
}
#[cfg(test)]
mod tests{
    use crate::{sort1, sort2};

#[test]
fn test_sort1() {
    let mut a1=[1,2,6,4,9,3];
    sort1(&mut a1);
    assert_eq!(a1,[1,2,3,4,6,9]);
}
#[test]
fn test_sort2() {
     let mut a2 =['a','z','d','k','h'];
     sort2(&mut a2);
     assert_eq!(a2,['a','d','h','k','z']);

     let mut a3 =[1.2,4.67,2.9,7.8,3.5];
     sort2(&mut a3);
     assert_eq!(a3,[1.2,2.9,3.5,4.67,7.8]);
 }
}