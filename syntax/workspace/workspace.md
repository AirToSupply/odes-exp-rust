

​		通过前面的案例，其实会发现一个crate就类型一个项目功能，但是随着项目深入开发，其复杂性也在增加，crate中的代码逻辑也会持续增大，此时希望将其进一步拆分成多个库crate，每个crate负责自己独有的逻辑。当有多个crate时，如何有效的管理多个crate是一个问题。

​		写过Java的工程师都知道，可以通过maven来构建自己的项目，同时maven也能构建父子项目，它可以让其管理多个子模块，而从使得项目更加紧凑。就像下面这样：

```shell
rush
  |-- rush-api           // API模块
  |-- rush-common        // 通用工具模块
  |-- rush-engine        // 引擎模块
  |-- rush-controller    // 控制器模块
  |-- ...
  |-- pom.xml             // 依赖文件
```

​		那么Rust是否也有这样的特性能够有效的组织多个crate呢？这需要通过Cargo提供了**工作空间**（workspaces）来进行组织。

# 工作空间管理多个crate

​		我们希望做成如下这个样子：

```shell
add
  |-- add_one         // 库crate
  |-- adder           // 二进制crate
  |-- Cargo.toml      // 构建配置文件
```

​		创建workspace所在加目录：

```shell
mkdir add && cd add
```

​		在家目录下创建Cargo.toml文件，这个配置文件用于配置了整个工作空间。大致如下：

```toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

​		由于是工作空间，需要它以 `[workspace]` 部分作为开始，然后通过members参数指定需要管理哪些crate。

​		接下来创建members中指定多个的crate：

```shell
cargo new adder
cargo new add_one --lib
```

# 工作空间crate如何协作

​		由于工作空间管理了多个crate，不同的crate之间是可以相互调用的。比如：需要实现二进制crate的adder需要调用库crate的add_one中的add_one函数。

​		在`add_one/src/lib.rs`文件中，增加一个 `add_one` 函数：

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

​		由于adder这个crate是调用方，为了让adder认识add_one这个crate，需要在adder自己的Cargo.toml添加如下内容：

```toml
add_one = { path = "../add_one" }
```

【⚠️】在cargo中并不会在工作空间中的crates会相互依赖，所以需要明确的定义工作空间中crate的依赖关系。

​		在二进制crate的adder中添加主逻辑方法。

```rust
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

【❓】如何运行这个二进制crate的adder。

【方法一】在二进制crate的adder的家目录中直接运行：

```shell
cargo run
```

【方法二】在工作空间add的家目录下指定crate名称运行：

```shell
cargo run -p adder
```



# 工作空间crate如何构建

## 构建所有crate

​		在工作空间家目录下直接构建：

```shell
cargo build
```



## 构建单个crate

【方法一】进入某个具体crate的家目录中直接构建：

```shell
cargo build
```

【方法二】在工作空间家目录下指定crate名称构建：

```shell
cargo build -p add_one
```

# 工作空间中如何测试

​		在add_one库crate下添加如下测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```



## 构建所有crate测试

​		在工作空间家目录下直接构建：

```shell
cargo test
```



## 构建单个crate测试

【方法一】进入某个具体crate的家目录中直接测试：

```shell
cargo test
```

【方法二】在工作空间家目录下指定crate名称进行测试：

```shell
cargo test -p add_one
```



# 工作空间中依赖外部包

  【⚠️】工作空间只在根目录有一个 *Cargo.lock*，而不是在每一个 crate 目录都有 *Cargo.lock*。这样做的好处是保了所有的 crate 都使用完全相同版本的依赖。

​		如果add_one和adder两个crate都需要rand这个库的0.8.5版本的依赖，那么需要同时在都两个crate的Cargo.toml下都要添加依赖定义：

```toml
[dependencies]
rand = "0.8.5"
```

​		然后通过整体构建即可，我们会在工作空间下Cargo.lock中看到这个库的定义：

```toml
[[package]]
name = "rand"
version = "0.8.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
dependencies = [
 "libc",
 "rand_chacha",
 "rand_core",
]
```

# PS

​		[通过工作空间管理crate-样例项目add](./add/)

