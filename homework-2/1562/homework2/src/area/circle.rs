// Copyright (c) david Technologies Co.Ltd. 2015-2022. 
// All rights reserved. Licensed under Apache-2.0.

use crate::area::area::{Area};
use std;

pub struct Circle <T>{
    // 半径
    radius: T,
}

/// 计算圆面积
///
/// # Arguments
/// # Examples
///
/// ```
/// ```
///
impl <T> Area<T> for Circle <T>
where T: Into<f64> + Copy, {
    fn area(&self) ->f64 {
        let radius_f64 = self.radius.into();
        std::f64::consts::PI * radius_f64 * radius_f64
    }
}

#[cfg(test)]
mod tests {
    use crate::area::circle::*;
    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 3 };
        assert_eq!(circle.area(), std::f64::consts::PI * 9.0);
    }
}