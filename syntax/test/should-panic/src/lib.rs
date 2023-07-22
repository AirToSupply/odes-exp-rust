pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        ✅ 通过should_panic注解测试当value=200时构建Guess一定会发生异常
     */
    #[test]
    // #[should_panic] 属性位于 #[test] 之后，对应的测试函数之前
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    /*
        ✅ 为了使 should_panic 测试结果更精确，可以给 should_panic 属性增加一个可选的 expected 参数用来匹配panic的异常消息
           
           比如这个测试用例在构建Guess中的value=-3时 必须触发panic同时异常消息包含"Guess value must be greater than or equal to 1"。
           这样做的好处是为了更精准的匹配panic是否是预期的那个panic
     */
    #[test]
    #[should_panic = "Guess value must be greater than or equal to 1"]
    fn less_than_1() {
        Guess::new(-3);
    }
}
