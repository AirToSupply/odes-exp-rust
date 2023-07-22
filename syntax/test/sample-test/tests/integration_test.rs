// 引入sample_test模块
use sample_test;

// 测试sample_test模块中公有函数
// 注意：
//      对于集成测试来说不需要#[cfg(test)]注解进行标记，在每个测试方法直接通过#[test]注解标记即可
#[test]
fn it_adds_two() {
    assert_eq!(4, sample_test::add_two(2));
}