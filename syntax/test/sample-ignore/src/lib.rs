/*
   通过#[ignore]注解用于标记默认情况下哪些的测试用例不运行
 */
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}