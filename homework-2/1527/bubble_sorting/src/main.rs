mod bubble_sorting;

fn main() {
    println!("这是i32-固定类型：");
    bubble_sorting::bubble_sorting_i32();
    println!("这是泛型：");
    bubble_sorting::bubble_sorting_generic_by_main();
}
