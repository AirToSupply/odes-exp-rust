# 如何构建测试？

​		通过如下命令构建一个名称为adder的库项目：

```shell
cargo new adder --lib
```

​		Rust在创建库项目时，会在lib.rs文件中提供测试用例的样例框架：[lib.rs](./adder/src/lib.rs)。

​		该文件中默认会自动生成it_works测试函数，在adder库项目的家目录下执行如下指令进行测试：

```shell
cargo test
```

​		返回结果如下：

```shell
running 1 test
// 列出当前所有的测试用例列表
test tests::it_works ... ok // ✅

// 列出所有测试用例的测试情况
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

​		我们可以在lib.rs文件中刻意构建一个错误的函数，看看会发生什么，代码如下：

```rust
#[test]
fn failing_test() {
  panic!("Make this test fail!");
}
```

​		继续运行测试，返回结果如下：

```shell
running 2 tests
test tests::it_works ... ok
test tests::failing_test ... FAILED // ❌ 列举出失败的测试用例

// 列举出具体的测试异常
failures:

---- tests::failing_test stdout ----
thread 'tests::failing_test' panicked at 'Make this test fail!', src/lib.rs:22:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::failing_test

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```



# 如何使用断言？

## [1.assert宏](./assert/src/lib.rs)

## [2.assert_eq宏和assert_ne宏](./assert-equal/src/lib.rs)

## [3.自定义断言消息](./assert-message/src/lib.rs)

## [4.should_panic注解](./should-panic/src/lib.rs)

## [5.将Result用于测试](./assert-result/src/lib.rs)



# 控制测试如何运行

## 1.并行或连续的运行测试

​	    当运行多个测试时，Rust默认使用多个线程来并行运行，这样进行测试速度会更快。当然了也可以指定使用多少个线程来进行测试。比如下面的指令希望单线程运行测试用例：

```shell
cargo test -- --test-threads=1
```

## 2.显示函数输出

​		默认情况下，当测试通过时，Rust的测试库会截获打印到标准输出的所有内容。比如在测试中调用了`println!` 进行了终端打印，只有测试未通过才会显示终端打印，但是测试通过却不会显示终端打印。比如：[sample](./sample/src/lib.rs)。

​		在这个测试用例中有this_test_will_pass和this_test_will_fail两个测试用例，this_test_will_pass会测试通过，this_test_will_fail会测试失败，通过`cargo test`我们看下测试效果：

```shell
unning 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8 ✅
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

​		可以看到this_test_will_pass这个测试成功的用例没有打印终端输出。可以通过如下执行让所有测试用例打印终端输出：

```shell
cargo test -- --show-output
```

​		返回结果：

```shell
running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4 ✅


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8 ✅
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 3.通过指定名字来运行部分测试

​		参考这段测试用例：[sample-adder](./sample-adder/src/lib.rs)

​		显然这个测试中提供了三个测试用例，如果只想运行one_hundred这个用例，可以使用如下指令：

```shell
cargo test one_hundred
```

​		返回结果如下：

```shell
running 1 test
test tests::one_hundred ... ok ✅

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

## 4.过滤运行多个测试

​		参考这段测试用例：[sample-adder](./sample-adder/src/lib.rs)

​		显然这个测试中提供了三个测试用例，如果只想运行add_打头的两个用例，可以使用如下指令：

```shell
cargo test add_
```

​		返回结果如下：

```shell
running 2 tests
test tests::add_three_and_two ... ok ✅
test tests::add_two_and_two ... ok   ✅

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

​		如果想要运行tests模块下的add_打头的用例，可以这么做：

```shell
cargo test tests::add_
```

## 5.如何忽略某些测试

​		参考这段测试用例：[sample-ignore](./sample-ignore/src/lib.rs/)

​		通过**#[ignore]**注解用于标记默认情况下哪些的测试用例不运行，比如这个例子中的expensive_test被标记#[ignore]表示在默认情况下这个用例不运行，通过`cargo test`运行结果如下：

```shell
running 2 tests
test expensive_test ... ignored 
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests sample-ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

​		那么如果想运行所有标记为#[ignore]注解的用例，可以通过如下指令：

```shell
cargo test -- --ignored
```

​		测试结果如下：

```shell
running 1 test
test expensive_test ... ok  ✅

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

   Doc-tests sample-ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

# 测试的组织结构

​		参考项目：[sample-test](./sample-test/)

## 1.单元测试

​		sample-test项目中的lib.rs文件如下：

```rust
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
```

​		通常来说，测试模块的 用`#[cfg(test)]` 注解进行标记告诉Rust只在执行 `cargo test` 时才编译和运行测试代码。一般而言在测试模块下定义函数用于对私有函数进行测试，当然了也可以测试对外暴露的函数，这取决于开发者自己。

## 2.集成测试

​		在Rust中，集成测试主要用于测试系统中对外暴露的函数或者接口。一般将每个集成测试文件放置在项目的家目录下的tests目录中。比如：[sample-test](./sample-test/)。集成测试的目录结构如下：

```shell
sample-test
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs                 // 存放单元测试用例
└── tests
    └── integration_test.rs    // 存放集成测试用例
    └── xxx.rs
```

​		通过`cargo test`执行查看测试效果：

```shell
running 1 test
test tests::internal ... ok ✅

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-e799926926559cd1)

running 1 test
test it_adds_two ... ok ✅

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests sample-test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

​		可以看到**通过`cargo test`执行测试，会对单元测试、集成测试和文档测试这个三部分测试集都进行测试**。

​		一般而言，开发者只想进行集成测试来测试系统项目的外部接口，比如这个例子只想测试integration_test.rs中定义了集成测试用例，可以通过`--test`参数指定具体的集成测试用例：

```shell
cargo test --test integration_test
```

​		返回结果如下：

```shell
running 1 test
test it_adds_two ... ok ✅

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

