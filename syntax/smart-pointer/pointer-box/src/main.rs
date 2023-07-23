/*
    引用 vs. 智能指针
    引用只是借用数据，没有数据的所有权，但是智能指针拥有它指向的数据，例如：字符串和Vector其实本质都是智能指针

    智能指针Box的使用场景：
    <1> 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候。
    <2> 当有大量数据时想转移这份数据的所有权，但同时不被拷贝。
    <3> 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候。

    Box智能指针基本原理：
    <1> Box<T> 类型是一个智能指针，因为它实现了 `Deref trait`，它允许 Box<T> 值被当作引用对待。
    <2> 当 Box<T> 值离开作用域时，由于 Box<T> 类型 `Drop trait` 的实现，box 所指向的堆数据也会被清除。
 */
fn main() {
    allocate_primitive_in_heap();
    define_recursive_list_for_box();
}

// [case-1] 如何将基本数据类型在堆上分配
fn allocate_primitive_in_heap() {
    // 这里表示一个i32类型的整数存储在堆上，变量b代表一个智能指针指向存储在堆上的i32类型的整数的地址
    let b = Box::new(5); // b: Box<i32>
    println!("b = {}", b);
}

// [case-2] 如何通过Box定义递归数据类型
// 观察下面这个枚举的定义：
/*
enum List { // ❌ recursive type `List` has infinite size
            //    insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable: `Box<`, `>`
    Cons(i32, List),
    Nil,
}
*/
// 在编译期，Rust并不知道List的具体大小是多少不确定，Rust编译器智能提示告诉开发者需要通过智能指针等技术间接让List的大小确定下来

// 可以先考察下面这个枚举的在编译期的大小
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/*
   对于Quit是不需要额外存储空间的
   对于Move类型的元组都是i32类型，存储空间也是确定的，即需要足够储存两个 i32 值的空间
   对于Write和ChangeColor类型也是同样确定的
   所以整体来说Message值所需的空间等于储存其最大成员的空间大小
 */

// 对于上面定义的List枚举的Cons类型在Rust编译器是无法知道其存储空间，这里可以使用Box来解决这个问题
#[derive(Debug)]
enum List { 
    Cons(i32, Box<List>), // 这里Box智能指针在编译时大小是确定的，并且该Box智能指针的值存储在栈上，其指向的数据是存储在堆上的
    Nil,
}

// Cons 成员将会需要一个 i32 的大小加上储存 box 指针数据的空间，看起来像这样：
/*
   Cons
   +-----+---------+
   |     |         |
   |     |   Box   |
   |     |         |
   | i32 |---------+
   |     | ******* |
   |     | [usize] |
   |     | ******* |
   +-----+---------+
 */
fn define_recursive_list_for_box() {
     let three =  List::Cons(3, Box::new(List::Nil));
     let two = List::Cons(2, Box::new(three));
     let one = List::Cons(1, Box::new(two));
     println!("one: {:?}", one); // Cons(1, Cons(2, Cons(3, Nil)))
}
