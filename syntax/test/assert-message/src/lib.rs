/*
   assert! 自定义断言消息
 */
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
       ❌ 
       thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello!`'
     */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("AirToSupply");
        assert!(result.contains("AirToSupply"), "Greeting did not contain name, value was `{}`", result);
    }
}
