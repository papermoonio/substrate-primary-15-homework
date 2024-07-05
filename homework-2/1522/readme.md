
# 创建项目准备
```sh
# create new File folder
cargo new folder_name
# get into it
cd folder_name
# open the folder use vscode
code .
```

# Task1
题目：使用Rust语言写一个冒泡排序的算法 基础要求：固定类型（比如i32）的数组排序 提高部分：能够使用范型和PartialOrd实现对任意类型的排序

## 1. bubble sort logic: for*2 + swap

## 2.结果
```sh
#cargo run

 Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/bubble_sort_project`
before Sort[5, 123, 3, 21, 30]
after Sort[3, 5, 21, 30, 123]
before Sorted i32 array: [5, 20, 9, 1, 5, 6]
after Sorted i32 array: [1, 5, 5, 6, 9, 20]
before Sorted f64 array: [3.1, 2.2, 5.5, 1.0, 4.7]
after Sorted f64 array: [1.0, 2.2, 3.1, 4.7, 5.5]
before Sorted string array: ["car", "bike", "computer", "cup"]
after Sorted string array: ["bike", "car", "computer", "cup"]
```

# Task2 
题目：为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

## 1. 概念学习
1. `impl` 是 Rust 用于实现某个类型的方法或 trait 的关键字。通过 `impl`，我们可以为一个结构体、枚举等定义方法

2. `match` 是 Rust 中的一个控制流结构，类似于其他语言中的 switch 语句。它可以根据不同的模式执行不同的代码块。

3. `self` 关键字在方法中用于表示调用该方法的实例。类似于其他面向对象语言中的 `this` 关键字。


## 2. 定义一个枚举TrafficLight 
`表示交通信号灯的三种状态`
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

## 3. 定义一个 trait: LightDuration
`包含一个返回灯持续时间的方法`
```rust
trait LightDuration {
    fn duration(&self) -> u32;
}
```
## 4. TrafficLight 枚举实现 LightDuration trait
`不同的灯返回不同的持续时间`
```rust
impl LightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}
```
## 5. 编写测试


# Task3
题目：实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None

## 1. 概念学习

## 2. 函数声明
`函数接收一个切片 &[u32]，返回一个 Option<u32>`
```rust
pub fn sum_u32_slice(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    //way1  
    // for &num in nums {
    //     sum = sum.checked_add(num)?;
    // }

    //way2  
    sum = sum.checked_add(num)?;

    Some(sum)
}
```

## 3.溢出处理求和
`在计算过程中，如果发生溢出，返回 None。`
```rust
#表达式链写法
sum = sum.checked_add(num)?;
Some(sum) // 返回求和结果 隐式

#
if let Some(result) = sum.checked_add(num) {
    sum = result;
} 
else {
    return None; // 溢出返回 None
}

Some(sum) // 返回求和结果 

```
## 4.运行结果
```sh
#cargo run res
 Compiling sum_u32_slice v0.1.0 (/root/sum_u32_slice)
 Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
 Running `target/debug/sum_u32_slice`
Sum: 15
Overflow occurred!
```


# Task4
题目：实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。

## 1.定义trait ->Area
## 2.定义Struct:  shape(Square Triangle Circle) 
## 3.实现trait
```rust
impl Area for shape {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
```
## 结果
```rust
# cargo run 
  Compiling print_area v0.1.0 (/root/print_area)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/print_area`
The area of the shape is: 78.54
The area of the shape is: 6.00
The area of the shape is: 36.00
```