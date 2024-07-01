pub struct Shape<T> {
    pub shape: String,
    pub more: Vec<T>,
}

impl <T: Into<f64> + std::fmt::Debug + std::clone::Clone> Shape<T> where Vec<f64>: From<Vec<T>>{
    pub fn new(shape: String, more: Vec<T>) -> Self {
        Shape { shape, more }
    }

    pub fn area(&self) -> f64{
        let m:Vec<f64> = <Vec<T> as Clone>::clone(&self.more).into();
        match self.shape.as_str() {
            "circle" => {
                let r=m[0];
                std::f64::consts::PI*r*r
            },
            "rectangle"=>{
                let w=m[0];
                let h=m[1];
                w * h
            },
            _ => {
                0.0
            },
        }
    }
}

// pub struct Shape<T> {
//     pub shape: String,
//     pub more: Vec<T>,
// }

// impl <T: Into<f64> + std::fmt::Debug + std::clone::Clone> Shape<T> where Vec<f64>: From<Vec<T>>{
//     pub fn new(shape: String, more: Vec<T>) -> Self {
//         Shape { shape, more }
//     }

    // pub fn display(&self) {
    //     println!("Shape: {}, more: {:?}", self.shape, self.more);
    // }

//     pub fn area(&self) -> f64{
//         let m:Vec<f64> = <Vec<T> as Clone>::clone(&self.more).into();
//         match self.shape.as_str() {
//             "circle" => {
//                 let r=m[0];
//                 std::f64::consts::PI*r*r
//             },
//             "rectangle"=>{
//                 let w=m[0];
//                 let h=m[1];
//                 w * h
//             },
//             _ => {
//                 0.0
//             },
//         }
//     }
// }