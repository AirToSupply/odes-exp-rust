/*
    ❓ 引用计数器Rc存在什么问题？
       通过调用Rc的关联函数clone会增加Rc<T>实例的引用计数（通过Rc::strong_count可以查看），只在其strong_count为0时才会被清理的Rc<T>实例，这其实是一种强引用，会增加循环引用的风险。

    ❓ 什么是弱引用Weak<T>？
       通过调用Rc的关联函数downgrade并传递Rc<T>实例的引用来创建其值的弱引用，即：弱引用智能指针Weak<T>
       通过调用Rc的关联函数downgrade，会将weak_count加1，也就是说weak_count来记录其存在多少个Weak<T>引用

    ❓ 强引用 vs. 弱引用
       强引用代表如何共享 Rc<T> 实例的所有权，弱引用并不属于所有权关系
       弱引用不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为0时被打断
       强引用计数strong_count必须为0，Rc<T> 实例被清理，而弱引用计数weak_count无需计数为0就能使Rc<T>实例被清理
 */


// 通过弱引用构建创建树形数据结构

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

// 构建节点
#[derive(Debug)]
struct Node {
    // 定义当前节点记录值
    value: i32,
    // 定义父节点
    // 注意：
    //       子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在
    parent: RefCell<Weak<Node>>,
    // 定义子节点列表
    // 注意：
    //       希望节点中的每一个Node节点都能共享所有权，所以子节点列表中的每一个元素都需要被Rc包裹
    //       希望能修改其他节点的子节点，所以整个子节点列表被RefCell包裹
    //       父节点应该拥有其子节点：如果父节点被丢弃了，其子节点也应该被丢弃，所以这里需要定义为强引用
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // 定义子节点
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // [leaf] strong = 1, weak = 0
    println!("[leaf] strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        // 定义父节点，并将子节点加入到父节点
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // 将子节点中的父节点引用和父节点关联起来
        // 这里通过Rc::downgrade将branch的Rc智能指针降级为弱引用智能智能
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // [branch] strong = 1, weak = 1
        println!("[branch] strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        // [leaf] strong = 2, weak = 0
        println!("[leaf] strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }

    // 注意在branch离开作用域时，仍然有一个弱引用为1，但是由于是弱引用不影响branch的清理
    // leaf parent = None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // [leaf] strong = 1, weak = 0
    println!("[leaf] strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
