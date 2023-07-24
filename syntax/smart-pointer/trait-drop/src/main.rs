/*
    什么是 Drop Trait？
    实现了 Drop Trait的类型在值要离开作用域时执行一些代码。
    也就是说在 Rust 中，可以指定每当值离开作用域时被执行的代码，编译器会自动插入这些代码。
    于是就不需要在程序中到处编写在实例结束时清理这些变量的代码（而且还不会泄漏资源）。

    Drop Trait使用场景？
    当 Box<T> 被丢弃时会释放 box 指向的堆空间。
 */
fn main() {
    impl_drop_trait();
    println!("--------------------");
    invoke_drop_trait();
}

// [case-0] 如何使用Drop Trait
// 注意：
//      <1> Drop trait 包含在 prelude 中，所以无需导入它。
//      <2> 实现了Drop Trait的实例离开作用域时 Rust 会自动调用 drop，并调用我们指定的代码，不需要显式调用drop方法。
//      <3> 生产代码不建议这样做，本例为了验证drop方法的调用时机。

// 模拟一个结构体
struct CustomSmartPointer {
    data: String,
}

// use std::ops::Drop;

// Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/*
    🌠
    CustomSmartPointers created...
    Dropping CustomSmartPointer with data `other stuff`!
    Dropping CustomSmartPointer with data `my stuff`!
 */
fn impl_drop_trait() {
    let c = CustomSmartPointer { data: String::from("my stuff")};    // 实例c的drop方法会后调用
    let d = CustomSmartPointer { data: String::from("other stuff")}; // 实例d的drop方法会先调用
    /*
       ❌ 不能显式调用drop方法
          explicit destructor calls not allowed
     */
    // c.drop(); 
    println!("CustomSmartPointers created...");
}

// [case-1] 如何正确的显式调用drop方法
/*
    🌠
    Dropping CustomSmartPointer with data `my stuff`!
    CustomSmartPointers created...
    Dropping CustomSmartPointer with data `other stuff`!
 */
fn invoke_drop_trait() {
    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};
    // 手动调用drop
    drop(c);
    println!("CustomSmartPointers created...");
}