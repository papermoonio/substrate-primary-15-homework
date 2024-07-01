fn bubble_sort_i32(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait LightDuration {
    fn duration(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 45,
            TrafficLight::Yellow => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort_i32() {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort_i32(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_bubble_sort_f64() {
        let mut arr = [64.5, 34.2, 25.7, 12.1, 22.9, 11.3, 90.0];
        bubble_sort(&mut arr);
        assert_eq!(arr, [11.3, 12.1, 22.9, 25.7, 34.2, 64.5, 90.0]);
    }

    #[test]
    fn test_red_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 30);
    }

    #[test]
    fn test_yellow_light_duration() {
        assert_eq!(TrafficLight::Yellow.duration(), 5);
    }

    #[test]
    fn test_green_light_duration() {
        assert_eq!(TrafficLight::Green.duration(), 45);
    }

    #[test]
    fn test_all_lights_different_durations() {
        let red = TrafficLight::Red;
        let yellow = TrafficLight::Yellow;
        let green = TrafficLight::Green;

        assert_ne!(red.duration(), yellow.duration());
        assert_ne!(yellow.duration(), green.duration());
        assert_ne!(red.duration(), green.duration());
    }

    

}


#[cfg(test)]
mod test_bubble_sort_i32 {
    use super::*;

    #[test]
    fn test_sum_empty() {
        let numbers: &[u32] = &[];
        assert_eq!(sum_u32(numbers), Some(0));
    }

    #[test]
    fn test_sum_normal() {
        let numbers = &[1, 2, 3, 4, 5];
        assert_eq!(sum_u32(numbers), Some(15));
    }

    #[test]
    fn test_sum_large_numbers() {
        let numbers = &[u32::MAX - 10, 5, 5];
        assert_eq!(sum_u32(numbers), Some(u32::MAX));
    }

    #[test]
    fn test_sum_overflow() {
        let numbers = &[u32::MAX, 1];
        assert_eq!(sum_u32(numbers), None);
    }

    // TODO
    #[test]
    fn test_sum_multiple_large_numbers() {
        let numbers = &[u32::MAX / 2, u32::MAX / 2, 1];
        assert_eq!(sum_u32(numbers), None);
    }

    #[test]
    fn test_sum_all_max() {
        let numbers = &[u32::MAX, u32::MAX, u32::MAX];
        assert_eq!(sum_u32(numbers), None);
    }

}

use std::f64::consts::PI;

// 定义 Shape trait
pub trait Shape {
    fn area(&self) -> f64;
}

// 实现圆形
pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// 实现三角形
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现正方形
pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 泛型函数，打印任何实现了 Shape trait 的类型的面积
pub fn print_area<T: Shape>(shape: &T) {
    println!("The area of the shape is: {}", shape.area());
}

// 单元测试
#[cfg(test)]
mod tests_4 {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.area(), 78.53981633974483);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 4.0, height: 3.0 };
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 4.0 };
        assert_eq!(square.area(), 16.0);
    }

    #[test]
    fn test_print_area() {
        let circle = Circle { radius: 5.0 };
        let triangle = Triangle { base: 4.0, height: 3.0 };
        let square = Square { side: 4.0 };

        // 这里我们不能直接测试打印输出，但我们可以确保函数不会 panic
        print_area(&circle);
        print_area(&triangle);
        print_area(&square);
    }
}
