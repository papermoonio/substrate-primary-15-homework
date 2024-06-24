mod sort;

use sort::{bubble_sort, bubble_sort_generic};

fn main() {
    // 测试非泛型冒泡排序
    let mut array = [3, 2, 1, 5, 4, 6, 11, 0, 7, 17];
    bubble_sort(&mut array);
    println!("非泛型冒泡排序结果：{:?}", array);

    println!("================================================");

    // 测试泛型冒泡排序
    // 准备一个Vec<String>
    let mut list: Vec<String> = vec![];
    list.push("We".to_string());
    list.push("use".to_string());
    list.push("generics".to_string());
    list.push("to".to_string());
    list.push("create".to_string());
    list.push("definitions".to_string());
    list.push("for".to_string());
    list.push("items".to_string());
    list.push("like".to_string());
    list.push("function".to_string());
    list.push("signatures".to_string());
    list.push("or".to_string());
    list.push("structs".to_string());
    list.push("which".to_string());
    list.push("we".to_string());
    list.push("can".to_string());
    list.push("then".to_string());
    list.push("use".to_string());
    list.push("with".to_string());
    list.push("many".to_string());
    list.push("different".to_string());
    list.push("concrete".to_string());
    list.push("data".to_string());
    list.push("types".to_string());

    bubble_sort_generic(&mut list);
    println!("泛型冒泡排序结果：\n{:?}", list.join(" "));

}
