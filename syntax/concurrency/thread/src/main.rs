use std::{thread, time::Duration};

fn main() {
    thread_closuer();
}

// [case-1] 如何在Rust中创建线程
fn create_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("[spawned thread] number: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // 主线程执行完毕，其它线程会被迫终止
    for i in 1..5 {
        println!("[main thread] number: {}", i);
        thread::sleep(Duration::from_millis(500));
    }
}

// [case-2] 如何让线程（非主线程）执行完毕再退出
/*
    运行结果：
    [main thread] number: 1
    [spawned thread] number: 1
    [main thread] number: 2
    [spawned thread] number: 2
    [main thread] number: 3
    [spawned thread] number: 3
    [main thread] number: 4
    [spawned thread] number: 4
    [spawned thread] number: 5
    [spawned thread] number: 6
    [spawned thread] number: 7
    [spawned thread] number: 8
    [spawned thread] number: 9
 */
fn join_thread() {
    // t: JoinHandle<()>
    let t = thread::spawn(|| {
        for i in 1..10 {
            println!("[spawned thread] number: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("[main thread] number: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    // 调用线程t的join方法，让线程t执行完毕之后，主线程才能退出 
    // 线程t有可能运行出错，所以这里推荐再调用unwrap方法 
    t.join().unwrap();
}

// [case-3] 线程闭包中的变量问题
/*
❌❌❌
fn thread_closuer() {
    let v = vec![1, 2, 3];
    // 在线程闭包中Rust可以自动推导到闭包中的变量v为借用的值，
    // 但是问题在线程闭包中Rust无法界定这个v的借用是否在有效期内，比如：在线程闭包之外的某个作用域变量v会被drop掉
    let t = thread::spawn(|| {
                                          // ^^ may outlive borrowed value `v`
        println!("vector: {:?}", v); // ❌  `v` is borrowed here
    });
    t.join().unwrap();
}
 */

fn thread_closuer() {
    let v = vec![1, 2, 3];
    // 这里通过move将变量v的所有权移入到线程闭包中，那么之后线程闭包之外的后续代码就不能在使用变量v，因为变量v的生存期已经被固定到线程闭包中 
    let t = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    t.join().unwrap();
}



