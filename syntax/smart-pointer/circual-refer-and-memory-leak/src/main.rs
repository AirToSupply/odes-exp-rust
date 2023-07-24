use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// 通过枚举定义Cons + List数据结构
#[derive(Debug)]
enum List {
    // Cons value不能改变，所以value不采用任何智能指针
    // Cons next ptr可以被改变，所以使用内部可变性智能指针RefCell
    // Cons next ptr使用引用计数智能指针是希望可以拥有多个所有者
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // 获取下一个元素
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item), // item: &RefCell<Rc<List>>
            Nil => None,
        }
    }
}

// 在Rust中如何制造引用循环

// 构建阶段产生[循环引用]
// 步骤<1>
/*
     Stack                         Haap
                                   +---------------+
     [[a]]  ============>  <<a'>>  |   5   |  Nil  |
                                   +---------------+
 */
// [a rc count]: 1

// 步骤<2>
/*
     Stack                         Haap
                                   +---------------+
     [[a]]  ============>  <<a'>>  |   5   |  Nil  |
                                   +---------------+

                                   +---------------+
     [[b]]  ============>  <<b'>>  |   10  |<<a'>> |
                                   +---------------+
 */
// [a rc count]: 2
// [b rc count]: 1

// 步骤<3>: 产生循环引用
/*
     Stack                         Haap
                                   +---------------+
     [[a]]  ============>  <<a'>>  |   5   |<<b'>> |
                                   +---------------+

                                   +---------------+
     [[b]]  ============>  <<b'>>  |   10  |<<a'>> |
                                   +---------------+
 */
// [a rc count]: 2
// [b rc count]: 2
// 🆖：此时获取a或者b的下一次元素都会产生：stack overflow panic

// 释放阶段产生[内存泄漏]
// 步骤<1> 清理b 
/*
     Stack                         Haap
                                   +---------------+
     [[a]]  ============>  <<a'>>  |   5   |<<b'>> |
                                   +---------------+

                                   +---------------+
                           <<b'>>  |   10  |<<a'>> |
                                   +---------------+
 */
// [a rc count]: 2
// [b rc count]: 1
// 🆖：b的引用计算不为0，所以栈空间上会进行释放，堆空间不会进行释放

// 步骤<2> 清理a
/*
     Stack                         Haap
                                   +---------------+
                           <<a'>>  |   5   |<<b'>> |
                                   +---------------+

                                   +---------------+
                           <<b'>>  |   10  |<<a'>> |
                                   +---------------+
 */
// [a rc count]: 1
// [b rc count]: 1
// 🆖：a的引用计算不为0，所以栈空间上会进行释放，堆空间不会进行释放
// 🆖：最终导致内存泄漏


fn main() {
    // <1>
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))); // a: Rc<List>
    println!("[create List a]");
    // [a] rc count: 1, next item: Some(RefCell { value: Nil })
    println!("[a] rc count: {}, next item: {:?}", Rc::strong_count(&a), a.tail());

    // <2>
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // b: Rc<List>
    println!("[create List b]");
    // [a] rc count: 2, next item: Some(RefCell { value: Nil })
    println!("[a] rc count: {}, next item: {:?}", Rc::strong_count(&a), a.tail());
    // [b] rc count: 1, next item: Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
    println!("[b] rc count: {}, next item: {:?}", Rc::strong_count(&b), b.tail());

    // <3> 开始构建循环引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("[circual reference]");
    // [a] rc count: 2
    // [b] rc count: 2
    println!("[a] rc count: {}", Rc::strong_count(&a));
    println!("[b] rc count: {}", Rc::strong_count(&b));

    // ❌❌❌ 获取a的下一个元素会产生：stack overflow panic
    // println!("[a] next item = {:?}", a.tail());
}