// Copyright (c) david Technologies Co.Ltd. 2015-2022. 
// All rights reserved. Licensed under Apache-2.0.

use crate::area::area::{Area};
pub struct Square <T>{
    // 边长
    pub side_length: T,
}

/// 计算正方形的面积
///
/// # Arguments
/// # Examples
///
/// ```
/// use homework2::area::Area;
/// use homework2::area::square::Square;
/// let square = Square { side_length: 3 };
/// let square_area = square.area();
/// ```
///
impl <T> Area<T> for Square <T>
where T: Into<f64> + Copy, {
    fn area(&self) ->f64 {
        let side_length = self.side_length.into();
        side_length * side_length
    }
}

#[cfg(test)]
mod tests {
    use crate::area::square::*;
    #[test]
    fn test_square_area() {
        let square = Square { side_length: 3 };
        assert_eq!(square.area(), 9.0);
    }
}