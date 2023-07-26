/*
    ❓什么是不安全的Rust?
        一般而言，Rust在编译时会强制执行的内存安全保证。在保证相对安全的条件下，Rust也提供了不会强制执行这类内存安全保证，这种"超能力"被称之为不安全的Rust。
        在Rust中通过unsafe关键字来完成不安全的操作，这点类似于Java中的Unsafe API。

    ❓为什么需要不安全的Rust？
        <1> Rust编译器会尝试确定一段代码是否支持某个保证规则，比如：内存安全保证，它拒绝一些合法的程序比接受无效的程序要好一些。
            也就是说有时代码可能是合法的，但如果Rust编译器没有足够的信息来确定，它将拒绝该代码。
            在这种情况下，可以使用不安全代码告诉编译器，这段代码是安全的。不过需要注意，使用不安全Rust风险自担，
            如果不安全代码出错了，比如：解引用空指针，可能会导致不安全的内存使用。
        <2> 底层计算机硬件固有的不安全性，那么Rust作为一门相对底层的编程语言。如果 Rust 不允许进行不安全操作，那么有些任务则根本完成不了！
            Rust 需要能够进行像直接与操作系统交互，甚至于编写你自己的操作系统这样的底层系统编程！

    ❓不安全的Rust有哪些使用场景？
        <1> 解引用裸指针
        <2> 调用不安全的函数或方法
        <3> 访问或修改可变静态变量
        <4> 实现不安全 trait
        <5> 访问 union 的字段
 */
fn main() {
    deref_raw_ptr();
    invoke_unsafe_func();
    split_at_mut_func();
    unsafe_split_at_mut_func();
    invoke_c_func();
    static_var();
}

// [case-1] 解引用裸指针
/*
   裸指针对于Rust的"危害"
        <1> 绕开Rust借用规则
        <2> 它可以同时拥有可变和不可变的指针
        <3> 多个可变指针可以指向同一个地址
        <4> 不能保证指向一个合法的地址
        <5> 可以指向null
        <6> 没有清理机制
 */
// 注意：这样使用的好吃
//      <1> 裸指针会绕开复杂的Rust借用规则，在通过unsafe使用时需要避免数据竞争问题
//      <2> 有利于和C语言进行互操作
fn deref_raw_ptr() {
    let mut num = 5;

    // 将Rust不可变引用转换为不可变的裸指针（C式指针）
    let r1 = &num as *const i32; // r1: *const i32
    // 将Rust可变引用转换为可变的裸指针（C式指针）
    let r2 = &mut num as *mut i32; // r2: *mut i32

    // ❌ 无法通过编译
    /*
       this operation is unsafe and requires an unsafe function or block
       dereference of raw pointer is unsafe and requires unsafe function or block
       raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: 
       all of these are undefined behavior
     */
    // println!("r1 value is {}", *r1); // ❌
    // println!("r2 value is {}", *r2); // ❌

    // ✅ 通过编译
    unsafe {
        println!("r1 value is {}", *r1); // ✅
        println!("r2 value is {}", *r2); // ✅
    }
}

// [case-2] 调用不安全函数或方法
// 注意：
//      关键字unsafe表示该函数具有调用时需要满足的要求，而 Rust 不会保证满足这些要求。
//      通过在 unsafe 块中调用不安全函数，表明我们已经阅读过此函数的文档并对其是否满足函数自身的契约负责。

// 定义一个不安全的函数
unsafe fn dangerous() {
    println!("dangerous function ...");
}

// 调用不安全的函数
fn invoke_unsafe_func() {
    unsafe {
        dangerous();
    }
}

// [case-3] 创建不安全代码的安全抽象

// 先看一段代码
fn split_at_mut_func() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..]; // r: &mut [i32]

    // 将i32类型的切片按照索引3处切割然后解构为a和b两个切片
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// 如果自己实现一个split_at_mut函数大概如下
// 但是Rust编译这个函数时就会报错：不能在同一时刻对一个切片做两次可变引用
// ❓ 但是实现这个函数的本意是对一个切片的两个部分分别进行可变引用，但是Rust编译器却无法理解这一行为
/*
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    // ❌
    // cannot borrow `*slice` as mutable more than once at a time
    (&mut slice[..mid], &mut slice[mid..]) // ❌
}
*/

// 通过unsafe实现如下：
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // ptr: *mut i32

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            // 对指针ptr进行偏移
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_split_at_mut_func() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..]; // r: &mut [i32]

    // 将i32类型的切片按照索引3处切割然后解构为a和b两个切片
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// [case-4] 使用 extern 函数调用外部代码
// 注意：
//      Rust通过extern关键字提供对和其它语言交互的支持

// 通过这个extern列出了我们希望能够调用的另一个语言中的外部函数的签名和名称
extern "C" {
    fn abs(input: i32) -> i32;
}

fn invoke_c_func() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// [case-5] 访问或修改可变静态变量

// 什么是静态变量？
// 在Rust中的静态(static)变量是全局变量，也就是说在Rust中支持全局变量，单其实全局变量的使用是有诸多问题的：
//   <1> 所有权规则的限制
//   <2> 可能会造成数据竞争，比如：两个线程访问相同的可变全局变量

// 定义和使用一个不可变静态变量
// 注意：
//      需要显式声明数据类型
//      静态变量在内存中有一个固定的地址
//      静态变量可以是可变的，也可以是不可变的
static HELLO_WORLD: &str = "Hello, world!";

fn use_static_var() {
    println!("name is: {}", HELLO_WORLD);
}

// 定义一个可变的静态变量
// 注意：❗️读取或修改一个可变静态变量是不安全的
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // 修改可变静态变量
    unsafe {
        COUNTER += inc;
    }
}

fn static_var() {
    add_to_count(3);

    // 读取可变静态变量
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// [case-6] 实现不安全 trait

// 定义一个不安全特征
unsafe trait Foo {

}

// 给一个类型实现一个不安全特征，那么这个实现块也要被声明不安全
unsafe impl Foo for i32 {

}