# homework 2

## 关于作业的测试编写的开篇说明

依照作业的要求，本次的所有作业均需要提供单元测试，因此本期作业非常适合使用测试驱动开发的方式进行解答编写，对于 Rust 的测试编写，Rust 官方提供了两种方案，一种是单元测试的方案，具体实现的逻辑代码与测试代码在同一文件当中，如下所示：
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
```

另外一种则是集成测试，在项目目录下建立名为 tests 的目录，将所有测试代码放在 tests 目录下，实际如果把单元测试的代码也写在这个目录下，运行的时候也是正常运行。

我在本次的作业当中采用第一个方案，测试用例与具体实现代码分开编写，逻辑代码文件 xxx.rs 所对应的 测试代码名为 test_xxx.rs。

## Rust coding
### 第一题：使用Rust语言写一个冒泡排序的算法
- 基础要求：固定类型（比如i32）的数组排序
- 提高部分：能够使用范型和PartialOrd实现对任意类型的排序

解答：

基础题：
第一步写测试，第一个测试用例如下：
```rust
// test_bubble_sort.rs
use crate::bubble_sort;
#[test]
fn test_in_i32_list() {
    let mut arr = [4, 2, 1, 3];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4]);
}
```
运行一下 `cargo test`，失败，实现初步的逻辑代码如下：
```rust
// bubble_sort.rs
pub fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}
```
再次运行`cargo test`，成功。基础题部分依据题目要求，仅需要对单一类型数组排序即可，解答选择了数据类型为i32的数组。至此，基础部分解决。

提高部分：

提高部分要求这个算法可以对更多类型的数组进行排序，所以我们会用到泛型，回到我们的测试，我们加2个测试用例，分别用 f64 、String 与 char 类型：
```rust
// test_bubble_sort.rs 
#[test]
fn test_in_f64_list() {
    let mut arr = [-4.5, 2.6, -1.1, 3.7];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, [-4.5, -1.1, 2.6, 3.7]);
}

#[test]
fn test_in_string_list() {
    let mut arr = ["ask","tear","rain","book"];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, ["ask","book","rain","tear"]);
}

#[test]
fn test_in_char_list() {
    let mut arr = ['k','u','r','t'];
    bubble_sort::bubble_sort(&mut arr);
    assert_eq!(arr, ['k','r','t','u']);
}
```

我们此时运行一下`cargo test`，由于逻辑代码没有改动，自然直接报错`error[E0308]: mismatched types`。

第一次写一下泛型的，无需关注约束的时候会写出这样的：
```rust
pub fn bubble_sort<A>(arr: &mut [A])
{
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}
```
这时候运行`cargo test`，会报错比较数据大小的操作无法在泛型 A 应用，编译器此时会建议为泛型 A 加入 `std::cmp::PartialOrd` 的约束，因此遵照编译器的建议，我们改写一下：
```rust
pub fn bubble_sort<A>(arr: &mut [A])
where
    A: PartialOrd,
{
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}
```
运行`cargo test`，成功，至此结束

### 第二题：为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

解答：

对于交通信号灯，我们自然知道要分红、黄、绿三个颜色，因此我们的测试用例也按照红、黄、绿来编写，我这里设定的红、黄、绿的时间分别为60、10、90秒：
```rust
#[test]
fn red_light_works(){
    let light = TrafficLight::Red;
    assert_eq!(light.duration(), 60);
}

#[test]
fn yellow_light_works(){
    let light = TrafficLight::Yellow;
    assert_eq!(light.duration(), 10);
}

#[test]
fn green_light_works(){
    let light = TrafficLight::Green;
    assert_eq!(light.duration(), 90);
}
```

我们在这里为我们将要实现的方法命名为duration，因此可以初步写出如下代码：
```rust
pub enum TrafficLight {
    Red,
    Yellow,
    Green
}

pub trait TrafficLightTime {
    fn duration(&self) -> u32;
}

impl TrafficLightTime for TrafficLight {
    fn duration(&self) -> u32 {
    }
}

```

执行`cargo test`，测试失败，补充具体实现代码如下：
```rust
match self {
    TrafficLight::Red => 60,
    TrafficLight::Yellow => 10,
    TrafficLight::Green => 90
}
```

执行测试，成功，本题完成。
### 第三题：实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
解答：

依照前面题目的做法，我们先编写测试用例如下：
```rust
#[test]
fn normal_pass_test() {
    let a = [1, 2, 3, 4, 5];
    assert_eq!(array_sum(&a), Some(15));
}

#[test]
fn overflow_test() {
    let a = [u32::MAX,1];
    assert_eq!(array_sum(&a), None);
}
```

运行`cargo test`，不成功，现在可以开始编写实现代码。

第一个版本的代码我写的是这样：
```rust
pub fn array_sum(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in arr {
        if sum > u32::MAX {
            return None;
        }
        sum += *i;
    }
    Some(sum)
}
```
这个版本很明显是错误的，循环里的 if 判断很明显不起作用，后面的加法遇到溢出是直接报 overflow 的 panic，并不符合我们的需求。这里我们打开 Rust 的官方文档，对于数字类型，应对加法溢出，有一个 checked_add() 方法，该方法直接返回 Option<T> 的类型的值，我们可以直接调用，修改代码如下：
```rust
pub fn array_sum(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in arr {
        sum = sum.checked_add(*i)?;
    }
    Some(sum)
}
```
测试运行，通过，本题完成。

### 第四题：实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。

解答：

做这道题的时候，突然想到一个问题，对于这种数学计算的程序，在输入需要计算的数据的时候，我们不能假设使用这个程序的用户必然会输入整数或者小数，最后的输出结果的数据类型，我们可以控制统一的类型。

基于这样的思路，我在编码过程当中最开始花了不少时间考虑这里如何实现，因为众所周知，Rust 是一门强类型的编程语言，如果达成前面述及的效果，需要考虑类型转换，接下来我还是简单记录一下这个过程。

首先我们还是考虑一下一般一些的实现，即假设我们的输入数据和输出数据的类型均为 f64 （事实上输出也必然保持 f64 类型，除去边长非整数的情况外，圆的面积里的 $ \pi $ 本身也是个无理数），我们以打印计算长方形面积为例来阐述设计过程。

首先写一下单元测试：
```rust
#[test]
fn test_rectangle() {
    let rectangle = Rectangle { width: 3.0, height: 2.0 };
    assert_eq!(rectangle_area(&rectangle), 6.0);
}
```
`cargo test`，失败，现在可以开始实现长方形的结构设定：
```rust
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
```
实现第一版计算面积的方法如下：
```rust
pub fn rectangle_area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}
```
`cargo test`，成功

看着做得不错，但是稍等，我们不仅要算长方形一种图形的面积，如果按这样的编写一个图形结构就要单独写一个计算面积的函数，代码的复用性就会下降，阅读起来不够精炼简洁，另外，我们最终需要做的是，要打印他们的面积，如果单独对每一个图形结构编写一个打印的函数，这样的代码冗余也是非常影响简洁的，那么有没有好一些的方案作为改进？

Rust 确实提供了一个不错的方案，那就是 trait，那么接下来的设计就清晰了：我们给每个图形结构实现一个相同的 trait，这个 trait 包含了一个计算面积的函数声明，具体的实现则是交给各个结构再来编写。这样的好处在于，统一各个结构计算面积时调用相同的函数，接下来我们编写打印面积的函数，直接限定只有实现了指定 trait 的参数才能使用。熟悉面向对象的朋友们到这里会发现，我们在用一个并非是面向对象的语言写了一套类似多态的用法的代码，这里也体现了 Rust 灵活的抽象力。

事不宜迟，重构测试用例：
```rust
#[test]
fn test_rectangle() {
    let rectangle = Rectangle { width: 3.0, height: 2.0 };
    assert_eq!(rectangle.area(), 6.0);
}
```
编写 trait ：
```rust
pub trait Shape {
    fn area(&self) -> f64;
}
```
编写实现：
```rust
impl Shape for Rectangle
{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```
`cargo test`，成功

有了这个实现后，编写打印面积的函数，借助 trait 泛型限定，我们要求打印的面积保留两位小数，于是可以写出如下函数：
```rust
pub fn print_area<T>(shape: T)
where
    T: Shape,
{
    println!("Area: {:.2?}", shape.area());
}
```

其实到这里，我们的题目已经可以算是解答完了，只需要再补充三角形和圆形的计算公式就可以了。

但如我在前面所说，我们要在输入的数据上，保证整数和小数都可以使用，因此需要对于结构实现修改到使用泛型，当然这些参数使用泛型也要做一定的约束，这里我们分别使用了 `std::ops` 、`Copy` 、`Into` trait，重构代码最终如下：
```rust
pub struct Rectangle<T, K>
where
    T: Mul<Output = T> + Copy + Into<f64>,
    K: Mul<Output = K> + Copy + Into<f64>,
{
    pub width: T,
    pub height: K,
}

impl<T, K> Shape for Rectangle<T, K>
where
    T: Mul<Output = T> + Copy + Into<f64>,
    K: Mul<Output = K> + Copy + Into<f64>,
{
    fn area(&self) -> f64 {
        self.width.into() * self.height.into()
    }
}
```
经测试，符合我们的预期，其他图形结构也按照这样的方式重构即可，这道题到这里才可以说正式解答完成。