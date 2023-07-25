use std::sync::Arc;
use std::{sync::Mutex, thread};

/*
   原子引用计数器智能Arc<T>

   注意：
        <1> RefCell<T>一般配合Rc<T>一起使用，但是需要注意循环引用问题
        <2> Mutex<T>一般配合Arc<T>一起使用，但是需要注意死锁问题
 */

// 多线程环境下计算器
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
