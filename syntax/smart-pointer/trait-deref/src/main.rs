/*
   什么是Deref特征？
   实现 Deref trait 允许我们重载 解引用运算符（dereference operator）即：*（解引用）

   Deref特征的使用场景？
   实现 Deref trait 的智能指针可以被当作常规引用来对待，例如：String和Vector等
 */
fn main() {
    trace_value_for_ptr();
    trace_value_for_box();
    trace_value_for_my_box();
    my_box_to_string_slice();
}

// [case-0] 使用普通指针追踪指针的值
fn trace_value_for_ptr() {
    let x = 5;
    let y = &x; // y: &i32

    assert_eq!(5, x);

    // *代表解引用 表示通过指针y或者该指针指向的值
    assert_eq!(5, *y);

    /*
        ❌
        can't compare `{integer}` with `&{integer}`
        the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
        the following other types implement trait `PartialEq<Rhs>`:
        f32
        f64
        i128
        i16
        i32
        i64
        i8
        isize
        and 6 others
     */
    // assert_eq!(5, y); // ❌
}

// [case-1] 通过指针指针Box追踪指针的值（优化[case-0]）
// 注意：
//      <1> 变量x是存储在栈上的
//      <2> 通过变量x构建Box智能指针的这个x是会进行一次拷贝至堆上
fn trace_value_for_box() {
    let x = 5;
    let y = Box::new(x); // y: Box<i32>
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y.as_ref());
}

// [case-2] 实现自定义智能指针MyBox（重点关注Deref trait ）

// 定义个MyBox结构体
struct MyBox<T>(T);

// 定义关联函数
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现Deref特征
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // 注意：这里返回的引用
        &self.0
    }
}

fn trace_value_for_my_box() {
    let x = 5;
    let y = MyBox::new(x); // y: MyBox<i32>
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// [case-3] 函数和方法的隐式 Deref 强制转换

// 定义一个测试函数，该函数入参类型为字符串切片
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn my_box_to_string_slice() {
    let y = MyBox::new(String::from("Rust")); // y: MyBox<String>
    // ❓ 传入的&y的类型为&MyBox<String> 为何调用函数不会报错
    hello(&y);
    hello(&(*y)[..]);

    let y_ref = &y;       // y_ref:   &MyBox<String>
    let y_deref = y_ref.deref(); // y_deref: &String
    let value = y_deref.deref();    // value:   &strs
}