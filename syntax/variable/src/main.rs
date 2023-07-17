fn main() {
    // [case-1]
    // 声明一个变量
    // 注意：
    //      <1> 在Rust中会进行变量类型的自动推导
    //      <2> 默认情况下变量是不可变的(immutable)
    let x = 5; // --> let x: i32 = 5;
    println!("x value is: {}", x);
    // ❌ 给不可变变量进行赋值
    // [ERROR]: cannot assign twice to immutable variable `x`
    // x = 10;

    // [case-2]
    // 声明一个可变变量
    let mut y = 5;
    println!("y value is: {}", y);
    y = 10;
    println!("y value is: {}", y);

    // [case-3]
    // 声明一个常量
    // 注意：
    //       <1> 常量的值在整个程序运行的生命周期中是不可修改的
    //       <2> 常量定义时需要显式声明数据类型
    const SUBSCRIBE_COUNT: u32 = 10_0000;
    println!("cconst `SUBSCRIBE_COUNT` value is: {}", SUBSCRIBE_COUNT);
    // ❌ 常量不可以通过mut声明
    // [ERROR]: const globals cannot be mutable
    // const mut COUNT: u32 = 100000;

    // [case-4]
    // 变量的遮蔽（shadowing）
    //    在Rust中可以对已经定义的变量进行重复定义，这种方式在JS语言中会产生变量重复定义异常
    // 注意：
    //       <1> Rust中重新定义变量可以改变数据类型 
    let content = 10;
    println!("[shadowing before] content value is: {}", content);
    let content = "中国";
    println!("[shadowing after] content value is: {}", content);
}
