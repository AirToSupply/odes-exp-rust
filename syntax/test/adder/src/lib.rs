// 定义一个用于被测试的方法
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 该注解表示这个模块是被用于测试的
#[cfg(test)]
mod tests {
    use super::*;

    // 该注解表示这个方法it_works为被测试的测试方法，这个函数用于测试函数add
    #[test]
    fn it_works() {
        let result = add(2, 2);
        // 通过断言判断结果是否符合预期
        assert_eq!(result, 4);
    }

    // 构建一个错误的测试函数
    #[test]
    fn failing_test() {
        panic!("Make this test fail!");
    }
}
