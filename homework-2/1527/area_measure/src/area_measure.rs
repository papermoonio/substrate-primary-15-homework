use rust_decimal::Decimal;

trait Shape{
    fn area(&self);
    fn perimeter(&self);
}

pub struct Circular {
    pub radius: Decimal,
}

pub struct Rectangle {
    pub length: Decimal,
    pub wigth: Decimal,
}


pub struct Square {
    pub length: Decimal,
}
/*
 * length_c 当作底边
 */
pub struct Triangle {
    pub length_a: Decimal,
    pub length_b: Decimal,
    pub length_c: Decimal,
    pub height: Decimal,
}

//圆形
impl Shape for Circular {
    fn area(&self) {
        let pai = Decimal::new(31416, 4);
        let rr: Option<Decimal> = self.radius.checked_mul(self.radius);
        match rr {
            Some(res) => {
                let all_res: Option<Decimal> = res.checked_mul(pai);
                match all_res {
                    Some(cp) => {
                        println!("圆的面积是：{}",cp.trunc_with_scale(4));
                    },
                    None => println!("圆的面积计算失败"),
                }
            },
            None => println!("圆的面积计算异常"),
        }
    }
    fn perimeter(&self) {
        let pai = Decimal::new(31416, 4);
        let perimeter = Decimal::new(200, 2);
        let rr: Option<Decimal> = self.radius.checked_mul(perimeter);
        match rr {
            Some(res) => {
                let all_res: Option<Decimal> = res.checked_mul(pai);
                match all_res {
                    Some(cp) => {
                        println!("圆的周长是：{}",cp.trunc_with_scale(4));
                    },
                    None => println!("圆的周长计算失败"),
                }
            },
            None => println!("圆的周长计算异常"),
        }
    }
}
//长方形
impl Shape for Rectangle {
    fn area(&self) {
        let rec_area: Option<Decimal> = self.length.checked_mul(self.wigth);
        match rec_area {
            Some(res) => {
                println!("长方形的面积是：{}",res.trunc_with_scale(2));
            },
            None => println!("长方形的面积计算异常"),
        }
    }
    fn perimeter(&self) {
        let perimeter = Decimal::new(200, 2);
        let rr: Option<Decimal> = self.length.checked_add(self.wigth);
        match rr {
            Some(res) => {
                let rec_per: Option<Decimal> = res.checked_mul(perimeter);
                match rec_per {
                    Some(cp) => {
                        println!("长方形的周长是：{}",cp.trunc_with_scale(2));
                    },
                    None => println!("长方形的周长计算失败"),
                }
            },
            None => println!("长方形的周长计算异常"),
        }
    }
}
//正方形
impl Shape for Square {
    fn area(&self) {
        let squ_area: Option<Decimal> = self.length.checked_mul(self.length);
        match squ_area {
            Some(res) => {
                println!("正方形的面积是：{}",res.trunc_with_scale(2));
            },
            None => println!("正方形的面积计算异常"),
        }
    }
    fn perimeter(&self) {
        let perimeter = Decimal::new(400, 2);
        let squ_per: Option<Decimal> = self.length.checked_mul(perimeter);
        match squ_per {
            Some(res) => {
                println!("正方形的周长是：{}",res.trunc_with_scale(2));
            },
            None => println!("正方形的周长计算异常"),
        }
    }
}
//三角形
impl Shape for Triangle {
    fn area(&self) {
        let perimeter = Decimal::new(50, 2);
        let tri_area: Option<Decimal> = self.length_c.checked_mul(self.height);
        match tri_area {
            Some(res) => {
                let tri_area_res: Option<Decimal> = res.checked_mul(perimeter);
                match tri_area_res {
                    Some(area_res) =>{
                        println!("三角形的面积是：{}",area_res.trunc_with_scale(2));
                    },
                    None => println!("三角形的面积计算异常"),
                }
            },
            None => println!("三角形的面积计算异常"),
        }
    }
    fn perimeter(&self) {
        let tri_per: Option<Decimal> = self.length_a.checked_add(self.length_b);
        match tri_per {
            Some(res) => {
                let tri_area_res: Option<Decimal> = res.checked_add(self.length_c);
                match tri_area_res {
                    Some(area_res) =>{
                        println!("三角形的周长是：{}",area_res.trunc_with_scale(2));
                    },
                    None => println!("三角形的周长计算异常"),
                }
            },
            None => println!("三角形的周长计算异常"),
        }
    }
}



pub fn print_shap<T:Shape>(shape:T){
    shape.perimeter();
    shape.area();
}

#[cfg(test)]
#[test]
pub fn area_test() {
    let circle: Circular = Circular{ radius: Decimal::new(500, 2)};
    println!("圆得半径是：{}",circle.radius);
    print_shap(circle);
    let rectangle: Rectangle = Rectangle{ length: Decimal::new(1000, 2), wigth: Decimal::new(800, 2)};
    println!("长方形的长：{}，宽：{}",rectangle.length,rectangle.wigth);
    print_shap(rectangle);
    let square: Square = Square{ length: Decimal::new(1000, 2)};
    println!("正方形的边长：{}",square.length);
    print_shap(square);
    let triangle: Triangle = Triangle{ length_a: Decimal::new(300, 2),length_b: Decimal::new(500, 2),length_c: Decimal::new(400, 2),height: Decimal::new(300, 2)};
    println!("三角形的边长：{}、{}、{} 高是：{}",triangle.length_a,triangle.length_b,triangle.length_c,triangle.height);
    print_shap(triangle);
}
