# 什么是宏？

​		通俗的来说，**宏**（*Macro*）指的是 Rust 中一系列的功能。例如：在Hello Rust程序中的打印语句println!就是使用了宏的特性。

​		宏非为如下两类：

​		1⃣️ **声明**（Declarative）宏：使用**macro_rules**!进行声明。

​		2⃣️ **过程**（Procedural）宏。



# 声明宏

​		声明宏大致是采用一种匹配表达式来实现的。我们在创建Vector时可以采用vec!宏进行创建：

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

​		我们可以通过尝试来定义vec!这个宏，代码参考：[lib.rs](./declarative-macros/src/lib.rs)

# 过程宏

​		过程宏大致分为如下3种过程宏：

​		1⃣️ 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码。

​		2⃣️ 类属性（Attribute-like）宏定义可用于任意项的自定义属性。

​		3⃣️ 类函数宏看起来像函数不过作用于作为参数传递的 token。		

## 如何编写自定义 `derive` 宏

​		大致看起来像下面这个样子：

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

​		上面的use导入两个自定义的crate先无需关心，这个例子主要作用是通过在结构体Pancakes加入名称为HelloMacro的derive宏之后使得Pancakes自动具有了hello_macro关联函数，这个关联函数的作用能够打印当前结构的类型名称，即Pancakes。由于Rust没有像Java或者Golang高级语言的反射特性，不过需要完成这种需求就可以通过自定义derive宏来完成。

​		首先我们需要脱离当前代码项目，重新创建一个名称为hello_macro的库crate：

```shell
cargo new hello_macro --lib
```

​		然后在lib.rs文件定义#[derive(HelloMacro)]这个注解中提到这个名称为HelloMacro的trait同时定义一个关联函数hello_macro，可以参考：[lib.rs](./hello_macro/src/lib.rs)。然后我们后续需要完成的derive宏的能力就是像办法去自动生成类似这样的代码：

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

​		现在我们来定义过程式宏，目前过程式宏必须在其自己的 crate 内，对于这个例子来说我们当前是处于hello_macro的crate下，需要回到hello_macro这个项目的家目录下，定义这个过程式宏的crate，并且这个crate命名必须是当前crate加上_derive，命名如下：

```shell
cd hello_macro
cargo new hello_macro_derive --lib
```

​		那现在我们需要在hello_macro_derive这个项目下去编写过程式宏的代码，首先需要在hello_macro_derive这个Cargo.toml文件添加如下内容：

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

​		proc-macro设置为true表示定义一个过程式宏，同时导入syn和quote这两个库。然后在hello_macro_derive这个crate下的lib.rs定义自定义宏代码，参考：[lib.rs](./hello_macro/hello_macro_derive/src/lib.rs)。

​		完成上述两步之后，需要将hello_macro_derive和hello_macro这两个crate执行cargo build命令。回到最初的样例工程中的Cargo.toml文件添加这两个crate依赖：

```toml
[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

​		然后再次执行开始测试代码即可，参考：[main.rs](./procedural-macros/src/main.rs)。

