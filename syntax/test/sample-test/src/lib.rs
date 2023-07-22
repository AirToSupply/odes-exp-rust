// 模拟对外暴露的函数
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// 模拟私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// 测试私有函数
#[cfg(test)]
mod tests {
    use super::*;

    // 通常一个测试用例对应于一个私有函数测试
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}