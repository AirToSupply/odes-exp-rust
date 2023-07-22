/*
    assert! 宏表示在希望确保测试中一些条件为 true。
    如果值是 true，assert! 什么也不做，同时测试会通过。
    如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /*
        检查一个较大的矩形确实能放得下一个较小的矩形
     */
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    /*
        一个更小的矩形不能放下一个更大的矩形：
     */
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7};
        let smaller = Rectangle {width: 5, height: 1};
        assert!(!smaller.can_hold(&larger));
    }
}
