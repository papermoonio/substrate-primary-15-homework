trait Area {
    fn area(&self) -> std::result::Result<f64, String>;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> Result<f64, String> {
        if self.radius < 0. {
            return Err("radius < 0".to_string());
        }
        const PI: f64 = std::f64::consts::PI;
        Ok(PI * self.radius * self.radius)
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Area for Triangle {
    fn area(&self) -> Result<f64, String> {
        if !((self.a + self.b > self.c) && (self.a + self.c > self.b) && (self.b + self.c > self.a))
        {
            return Err("No Build Triangle".to_string());
        }
        //海伦公式
        let s = (self.a + self.b + self.c) / 2.;
        Ok(f64::sqrt(s * (s - self.a) * (s - self.b) * (s - self.c)))
    }
}

struct Rect {
    width: f64,
    height: f64,
}

impl Area for Rect {
    fn area(&self) -> Result<f64, String> {
        if (self.width < 0.) || (self.height < 0.) {
            return Err("No Build Rect".to_string());
        }

        Ok(self.width * self.height)
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> Result<f64, String> {
        if self.side < 0. {
            return Err("No Build Square".to_string());
        }

        Ok(self.side * self.side)
    }
}

fn area_generic<T: Area>(x: &T) -> Result<f64, String> {
    x.area()
}

#[cfg(test)]
mod tests {
    use super::{area_generic,  Circle, Rect, Square, Triangle};

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(area_generic(&circle).unwrap(), 78.53981633974483);
    }

    #[test]
    fn test_circle_area_negative_radius() {
        let circle = Circle { radius: -1.0 };
        assert!(area_generic(&circle).is_err());
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        assert!(area_generic(&triangle).is_ok());
        let area = area_generic(&triangle).unwrap();
        // 由于海伦公式计算结果可能存在浮点数精度问题，这里使用比较接近的值进行比较
        assert!((6.0 - area).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area_invalid() {
        let triangle = Triangle {
            a: 1.0,
            b: 1.0,
            c: 2.0,
        };
        assert!(area_generic(&triangle).is_err());
    }

    #[test]
    fn test_rect_area() {
        let rect = Rect {
            width: 4.0,
            height: 6.0,
        };
        assert_eq!(area_generic(&rect).unwrap(), 24.0);
    }

    #[test]
    fn test_rect_area_negative_dimensions() {
        let rect = Rect {
            width: -1.0,
            height: 6.0,
        };
        let result = area_generic(&rect);
        assert!(area_generic(&rect).is_err());
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 5.0 };
        assert_eq!(area_generic(&square).unwrap(), 25.0);
    }

    #[test]
    fn test_square_area_negative_side() {
        let square = Square { side: -5.0 };
        assert!(area_generic(&square).is_err());
    }
}

fn main() {
    let circle = Circle { radius: 2. };
    if let Ok(area) = area_generic(&circle) {
        println!("{area}")
    };

    let triangle = Triangle {
        a: 3.,
        b: 4.,
        c: 5.,
    };
    if let Ok(area) = area_generic(&triangle) {
        println!("{area}")
    };

    let rect = Rect {
        width: 3.,
        height: 4.,
    };
    if let Ok(area) = area_generic(&rect) {
        println!("{area}")
    };

    let square = Square { side: 8. };
    if let Ok(area) = area_generic(&square) {
        println!("{area}")
    };
}
