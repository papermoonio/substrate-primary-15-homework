mod area;
mod circle;
mod square;
mod triangle;

use area::print_area;
use circle::Circle;
use square::Square;
use triangle::Triangle;

fn main() {
    let circle = Circle::new(1.0);
    let square = Square::new(3.0);
    let triangle = Triangle::new(3.0, 5.0);

    print_area(&circle);
    print_area(&square);
    print_area(&triangle);
}

