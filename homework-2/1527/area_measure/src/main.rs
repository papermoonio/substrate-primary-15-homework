use rust_decimal::Decimal;

mod area_measure;


fn main() {
    test();
}

fn test() {
    let circle: area_measure::Circular = area_measure::Circular{ radius: Decimal::new(500, 2)};
    println!("圆得半径是：{}",circle.radius);
    area_measure::print_shap(circle);
    let rectangle: area_measure::Rectangle = area_measure::Rectangle{ length: Decimal::new(1000, 2), wigth: Decimal::new(800, 2)};
    println!("长方形的长：{}，宽：{}",rectangle.length,rectangle.wigth);
    area_measure::print_shap(rectangle);
    let square: area_measure::Square = area_measure::Square{ length: Decimal::new(1000, 2)};
    println!("正方形的边长：{}",square.length);
    area_measure::print_shap(square);
    let triangle: area_measure::Triangle = area_measure::Triangle{ length_a: Decimal::new(300, 2),length_b: Decimal::new(500, 2),length_c: Decimal::new(400, 2),height: Decimal::new(300, 2)};
    println!("三角形的边长：{}、{}、{} 高是：{}",triangle.length_a,triangle.length_b,triangle.length_c,triangle.height);
    area_measure::print_shap(triangle);
}