/*
    使用 Result<T, E> 编写测试
 */
#[cfg(test)]
mod tests {

    /*
        在这个例子中函数的返回值类型为 Result<(), String>，
        这种方式表示：在测试通过时返回 Ok(())，在测试失败时返回带有 String 的 Err
     */
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}