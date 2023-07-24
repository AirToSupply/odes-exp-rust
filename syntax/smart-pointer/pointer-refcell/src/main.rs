/*
    🫵 什么是内部可变性模式？
       内部可变性模式是Rust中的一种设计模式。
       在Rust中如果拥有某个数据的不可变引用是不能对这个数据进行修改的。 

    🫵 内部可变性模式原理？
       内部可变性模式可以在不可变引用的情况下修改对应的数据，它的原理是使用了unsafe不安全代码（编译时不会被检查是否安全的代码）来绕开可变性的和借用规则。

    🫵 内部可变性智能指针RefCell的特点
       <1> 与Box智能指针一样对指向的数据拥有唯一的所有权
       <2> 和Box智能指针不同的是RefCell是在运行时检查借用规则， 而Box是编译期强制检查借用规则 
       <3> RefCell只适合使用单线程场景
 */
fn main() {
    ptr_for_rc_and_refcell();
}

/*
    需求：假设需要编写一个记录某个值与最大值的差距的库，并根据当前值与最大值的差距来发送消息。
         例如，这个库可以用于记录用户所允许的 API 调用数量限额。
 */
// 定义一个消息发送的接口
pub trait Messenger {
    // 注意：这里是&self是不可变引用
    fn send(&self, msg: &str);
}

// 定义一个库用来模拟发送器并设置发送的值和阙值
pub struct LimitTracker<'a, T: Messenger> {
    // 消息发送器
    messenger: &'a T,
    // 当前值
    value: usize,
    // 最大值
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// [case-0] 在没有内部可变性模式情况下引发的问题
/*
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // ❌
            // [分析] 
            //       <1> 这里需要通过自定义的MockMessenger结构体的sent_messages这个Vector中添加元素，但是send方法的签名self并不是可变的导致无法修改
            //       <2> 如果Messenger接口定义的send方法是&mut self即可以解决问题，但问题是接口的方法签名一些是无法修改的
            // ❓
            // [问题] 有没有办法在不可变引用的情况下能够改变指向数据的内部值可变
            self.sent_messages.push(String::from(message)); // ❌
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
*/

// [case-1] 通过内部可变性智能指针RefCell<T>解决不可变引用的内部可变性
// 注意：
//      <1> RefCell需要通过use关键字进行引入
//      <2> 调用RefCell的borrow_mut()方法可以获得类似的可变引用
//      <3> 调用RefCell的borrow()方法可以获得类似的不可变引用
#[cfg(test)]
mod tests {
    use super::*;
    // 引入内部可变性智能指针
    use std::cell::RefCell;

    struct MockMessenger {
        // 需要在定义是将这个Vector时使用内部可变性智能指针包一层
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // 对Vector的初始化时也需要通过内部可变性智能指针包一层
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 此时sent_messages已经是内部可变性智能指针
            // 通过调用RefCell的borrow_mut方法返回的其实相当于Vector的可变引用，这样可以完成堆上Vector内部数据的修改
            // 注意:
            //       self.sent_messages.borrow_mut()返回的类型是RefMut<Vec<String>>
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // 这里由于不需要修改内部数据，只需要获取不可变引用即可
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// [case-2] 内部可变性智能指针RefCell<T>是否违反Rust借用规则
// 注意：
//       <1> RefCell在多次调用borrow_mut方法后代码在编译器不会发生panic，而是在运行时发生panic
//       <2> RefCell在调用borrow_mut产生可变引用仍是然遵守Rust借用规则，即在同一时刻，只能有一个可变引用
// use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

/*
    这里通过sent_messages连续两次borrow_mut方法调用获取两个可变引用，但是下面这两行代码不会在编译报错，而是在运行发生panic
    同时报错信息也可以看出在同一个时刻不能有两个可变引用，这仍然遵守Rust借用规则
 */
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut(); // ❌  already borrowed: BorrowMutError

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

// [case-3] 结合Rc和RefCell拥有多个可变数据所有者
use std::cell::RefCell;
use std::rc::Rc;
// 定义List枚举的Cons结构
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

/*
    通过List枚举的Cons结构实现如下依赖

    <<b>>
    +-------+
    | 3 |   |===+
    +-------+   |
                |
                |    <<a>>
                |    +-------+         
                |==> | 5 |   |  =====>  <Nil>
                |    +-------+         
                |
    <<c>>       |
    +-------+   |
    | 4 |   |===+
    +-------+
     
 */
/*
    🪢：
    a before = Cons(RefCell { value: 5 }, Nil)
    b before = Cons(RefCell { value: 3 }, Cons(RefCell { value: 5 }, Nil))
    c before = Cons(RefCell { value: 4 }, Cons(RefCell { value: 5 }, Nil))
    a after = Cons(RefCell { value: 15 }, Nil)
    b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
 */
fn ptr_for_rc_and_refcell() {
    let value = Rc::new(RefCell::new(5)); // value: Rc<RefCell<i32>>

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    // 修改内部值
    *value.borrow_mut() += 10; 

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
