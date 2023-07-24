/*
     什么是引用计数智能指针？
     在Rust中大部分情况下所有权是明确的，即所有权只属于一个实例，引用计数智能指针，即：Rc<T>。可以启用多所有权进行的所有权的共享。
     举一个常见的需求：在图数据结构中，多个边可能指向相同的节点，而这个节点从概念上讲为所有指向它的边所拥有。
                    节点直到没有任何边指向它之前都不应该被清理因此也没有所有者。

    引用计数智能指针的原理？
    该指针记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。

    引用计数智能指针的缺点？
    <1>引用计数智能指针只能用于单线程场景，如果是多线场景下推荐使用：原子引用计数Arc<T>。
    <2>引用计数智能指针智能用于读取数据，并不能修改数据，否则就会有多个可变引用的问题从而范围了Rust的借用规则，一般推荐使用RefCell<T>
 */
fn main() {
    // graph_list_for_rc();
    trace_rc();
}

// [case-0] 智能指针Box案例回顾

// Cons数据结构枚举定义
/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/

// ❓ 我们能够通过List这种枚举数据结构解决下面的问题：
/*
    <<b>>
    +-------+
    | 3 |   |===+
    +-------+   |
                |
                |    <<a>>
                |    +-------+         +--------+  
                |==> | 5 |   |  =====> | 10 |   |  =====>  <Nil>
                |    +-------+         +--------+
                |
    <<c>>       |
    +-------+   |
    | 4 |   |===+
    +-------+
     
 */

// ❌
// value used here after move
// 出错的原因主要是由于：Cons 成员拥有其储存的数据，所以当创建 b 列表时，a 被移动进了 b 这样 b 就拥有了 a。
//                   接着当再次尝试使用 a 创建 c 时，这不被允许，因为 a 的所有权已经被移动。
// 总结：Cons类型的数据结构b和向同时拥有Cons类型的a
/*
use crate::List::{Cons, Nil};
fn graph_list_for_box() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); // ❌
}
*/

// [case-1] 引用计数智能指针
// 注意：
//      <1> 需要使用 use 语句将 Rc<T> 引入作用域，因为它不在 prelude 中。
//      <2> 引用计数智能指针需要使用Rc::clone的关联函数进行引用计数。
//      <3> 引用计数智能指针的clone函数实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。

// 引用引用智能指针
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn graph_list_for_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // a: Rc<List>
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

// [case-2] 如何显式的查看引用计数智能指针的引用计数
// 注意：
//       <1> 这里通过Rc的strong_count关联函数查看用计数智能指针的引用计数
/*
    🪢
    [count after creating <<a>>]: 1
    [count after creating <<b>>]: 2
    [count after <<c>> goes in scope]: 3
    [count after <<c>> goes out of scope]: 2
    [count after <<b>> has been dropped]: 1
 */
fn trace_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("[count after creating <<a>>]: {}", Rc::strong_count(&a));          // 1

    let b = Cons(3, Rc::clone(&a));
    println!("[count after creating <<b>>]: {}", Rc::strong_count(&a));          // 2

    {
        let c = Cons(4, Rc::clone(&a));
        println!("[count after <<c>> goes in scope]: {}", Rc::strong_count(&a)); // 3
    }

    println!("[count after <<c>> goes out of scope]: {}", Rc::strong_count(&a)); // 2

    drop(b);
    println!("[count after <<b>> has been dropped]: {}", Rc::strong_count(&a));  // 1
}
