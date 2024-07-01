use homework2::sort::bubble::bubble_sort;
use homework2::sort::trait_bubble::bubble_sort2;
use homework2::traffic::light::{Color,TrafficLight};
use homework2::sum::sum::sum_u32;
use homework2::area::circle::Circle;
use homework2::area::square::Square;
use homework2::area::triangle::Triangle;
use homework2::area::Area;

fn main() {
    println!("Starting homework2 sort...");
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Before sort: {:?}", arr);
    bubble_sort(&mut arr);
    println!("After sort: {:?}", arr);

    let mut arr2 = [64, 34, 25, 12, 22, 11, 90];
    println!("Before trait sort: {:?}", arr2);
    bubble_sort2(&mut arr2);
    println!("After trait sort: {:?}", arr2);
    println!("End homework2 sort...");

    println!("Starting homework2 traffic...");
    let red_light = Color::Red;
    let red_duration = red_light.get_duration();
    println!("red traffic duration: {:?}", red_duration);

    let green_light = Color::Green;
    let green_duration = green_light.get_duration();
    println!("green traffic duration: {:?}", green_duration);

    let yellow_light = Color::Yellow;
    let yellow_duration = yellow_light.get_duration();
    println!("yellow traffic duration: {:?}", yellow_duration);

    println!("End homework2 traffic...");


    println!("Starting homework2 sum ...");
    let mut test_arrs = vec![4, 3, 2, 1];
    let sum = sum_u32(&mut test_arrs);
    println!("sum: {:?}", sum);
    println!("End homework2 sum ...");

    println!("Starting homework2 area ...");
    let circle = Circle { radius: 3 };
    let circle_area = circle.area();
    println!("circle radus: {:?} -->area: {:?}", circle.radius, circle_area);


    let square = Square { side_length: 3 };
    let square_area = square.area();
    println!("square side length: {:?} --> area: {:?}", square.side_length, square_area);


    let triangle = Triangle { base: 3, height:10};
    let triangle_area = triangle.area();
    println!("triangle base: {:?}, heigth: {:?} --> area: {:?}", triangle.base, triangle.height, triangle_area);

    println!("End homework2 erea ...");
}
