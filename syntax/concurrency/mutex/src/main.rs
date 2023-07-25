use std::rc::Rc;
use std::{sync::Mutex, thread};

fn main() {
    prt_mutex();
}

// [case-0] 如何使用互斥器Mutex<T>
// 注意：
//       <1>互斥器Mutex<T>其实是一个互斥器智能指针
fn prt_mutex() {
    // 创建一个互斥器，保护的对象是一个i32类型的数据5
    let mutex = Mutex::new(5); // mutex: Mutex<i32>、

    {
        // 调用互斥器lock()方法会阻塞当前线程直到获取锁为止
        // 在获取锁的过程中其它线程可能已经获取锁之后在操作数据的过程中引发了panic导致无法释放锁，那么此时容易造成死锁
        // 所以在获取锁无果时推荐调用unwrap方法让其快速失败
        let mut num = mutex.lock().unwrap(); // num: MutexGuard<i32>
        // 修改数据
        // 这里获取锁之后返回一个MutexGuard类型，MutexGuard也是一个智能指针
        // 可以通过*符号修改数据说明MutexGuard智能指针实现了Deref trait
        *num  = 6;
    }   // 由于MutexGuard智能指针实现了Drop trait所以在退出作用域时不需要显式的释放锁

    println!("m = {:?}", mutex);
}

// [case-1] 多线程情况下使用互斥器Mutex<T>完成计数器
// ❌
// [分析] 在循环中创建了多个线程闭包的情况下，在第一个闭包已经将互斥器的所有权移入到第一个线程闭包中，那么在第二个线程闭包继续通过移入互斥器显然已经失效了
// [解决] 此时需要通过引用计数器智能指针Rc<T>来共享互斥器的所有权
/*
fn count() {
    // 初始化一个互斥器，置计数器的初始值为0
    let counter = Mutex::new(0);
    // 定义一个列表保存多个线程句柄
    let mut handles = vec![];
    // 生成10个线程，每个线程对计数进行+1操作
    for _ in 0..9 {
        let handle = thread::spawn(move || {
            // 获取锁
            let mut num = counter.lock().unwrap(); // ❌
                                               // ❌ ------- use occurs due to use in closure
            // +1
            *num += 1;
        });
        handles.push(handle);
    }
    // 让所有线程都执行完毕后进行后续操作
    for h in handles {
        h.join().unwrap();
    }
    // 获取计数器结果
    println!("Result: {}", *counter.lock().unwrap());
}
*/

// [case-2] 多线程情况下使用互斥器Mutex<T>完成计数器（通过引用计数器智能指针完成数据共享）
// ❌ [分析] 引用计数器智能指针虽然可以完成数据共享，但是并不是线程安全的
/*
fn count() {
    // 通过引用计数器智能指针将互斥器进行包裹从而完成多线程环境下所有权共享
    let counter = Rc::new(Mutex::new(0));
    // 定义一个列表保存多个线程句柄
    let mut handles = vec![];
    // 生成10个线程，每个线程对计数进行+1操作
    for _ in 0..9 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || { // ❌
                                     // ❌ Rc<Mutex<i32>>` cannot be sent between threads safely
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
*/