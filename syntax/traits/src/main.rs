use std::fmt::Display;

// [case-5] 使用 trait bound 有条件地实现方法

struct Pair<T> {
    x: T,
    y: T,
}

// 实现块1用于实现匹配任何类型
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 实现块2用于实现Display和PartialOrd这两个特征的实现块
// 也就是说如果结构体Pair中的泛型T同时实现Display和PartialOrd这两个特征，可以拥有这个实现块中所定义的所有的能力
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn main() {
    println!("Hello, world!");
}