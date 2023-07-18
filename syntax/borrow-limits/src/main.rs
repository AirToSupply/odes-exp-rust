/*
 <总结> 引用规则
    1.引用可以借用数据而不需要获取数据的所有权
    2.对于特定数据，在某个特定的时间，只能有一个可变引用，或者多个不可变引用（即：一旦有了可变引用，就不用同时再拥有不可变引用）
    3.引用必须是有效的
 */
fn main() {
    x1();
    x1_adv();

    x2();
    x2_adv();
}

// [限制-1] 🚫 在一个特定的作用域中，对于某个特定的数据只能有一个可变引用(模拟同时写)
fn x1() {
    let mut s = String::from("hello");
    let r1 = &mut s; // r1: &mut String
    // [ERROR] cannot borrow `s` as mutable more than once at a time
    // let r2 = &mut s; // r2: &mut String
    println!("r1 = {}", r1);
    // println!("r2 = {}", r2);
}

// [限制-1] ✅ 在一个特定的作用域中，对于某个特定的数据可以有多个不可变引用(模拟同时读)
fn x1_adv() {
    let mut s = String::from("hello");
    let r1 = &s; // r1: &String
    let r2 = &s; // r2: &String
    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
}

// [限制-2] 🚫 在一个特定的作用域中，对于某个特定的数据，如果已经存在不可变引用，不能同时存在可变引用(模拟同时读写)
fn x2() {
    let mut s = String::from("hello");
    let r1 = &s; // r1: &String
    let r2 = &s; // r2: &String
    // [ERROR] cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r3 = &mut s; // r3: &mut String
    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
    // println!("r3 = {}", r3);
}

// [限制-2] ✅ 
fn x2_adv() {
    let mut s = String::from("hello");
    let r1 = &s; // r1: &String
    let r2 = &s; // r2: &String
    println!("r1 = {}", r1);
    println!("r2 = {}", r2); // 因为编译器在这个点之后知道r1和r2后续不会再使用，所以r1和r2的作用域已经失效

    let r3 = &mut s; // r3: &mut String // 从个点开始往下的作用域，只有一个可变引用
    println!("r3 = {}", r3);
}

// 🚫 悬挂指针
// 该函数返回了变量s的引用，在函数运行结束之后变量s的作用域随之失效，此时指向变量s的指针指向是一个不存在并且失效的数据，即为悬挂指针
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/