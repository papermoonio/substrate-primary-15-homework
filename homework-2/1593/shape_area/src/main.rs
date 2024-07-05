mod shape_area;

#[cfg(test)]
mod test_shape_area;
fn main() {
    println!("Done.");

    let rectangle = shape_area::Rectangle { width: 3.0, height: 2 };
    shape_area::print_area(rectangle);

    let triangle = shape_area::Triangle { base: 3.0, height: 2.0 };
    shape_area::print_area(triangle);

    let circle = shape_area::Circle { radius: 8.0 };
    shape_area::print_area(circle);
}
