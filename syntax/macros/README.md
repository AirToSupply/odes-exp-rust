# 什么是宏？

​		通俗的来说，**宏**（*Macro*）指的是 Rust 中一系列的功能。例如：在Hello Rust程序中的打印语句println!就是使用了宏的特性。

​		宏非为如下两类：

​		1⃣️ **声明**（Declarative）宏：使用macro_rules!进行声明。

​		2⃣️ **过程**（Procedural）宏，大致分为如下3种过程宏：

   		（1）自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码。

​		   （2）类属性（Attribute-like）宏定义可用于任意项的自定义属性。

​		   （3）类函数宏看起来像函数不过作用于作为参数传递的 token。

# 声明宏

​		声明宏大致是采用一种匹配表达式来实现的。我们在创建Vector时可以采用vec!宏进行创建：

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

​		我们可以通过尝试来定义vec!这个宏，代码参考：[lib.rs](./declarative-macros/src/lib.rs)

# 过程宏