// Copyright (c) david Technologies Co.Ltd. 2015-2022. 
// All rights reserved. Licensed under Apache-2.0.

use crate::area::area::{Area};

pub struct Triangle <T>{
    // 底
    pub base: T,
    // 高
    pub height: T,
}

/// 计算正方形的面积
///
/// # Arguments
/// # Examples
///
/// ```
/// use homework2::area::Area;
/// use homework2::area::triangle::Triangle;
/// let triangle = Triangle { base: 3, height:10};
/// let triangle_area = triangle.area();
/// ```
///
impl <T> Area<T> for Triangle <T>
where T: Into<f64> + Copy, {
    fn area(&self) ->f64 {
        let base = self.base.into();
        let heigth = self.height.into();
        0.5 * base * heigth
    }
}

#[cfg(test)]
mod tests {
    use crate::area::triangle::*;
    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 3, height:10};
        assert_eq!(triangle.area(), 15.0);
    }
}