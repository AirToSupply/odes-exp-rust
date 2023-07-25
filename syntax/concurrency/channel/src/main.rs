use std::{thread, time::Duration};

fn main() {
    // single_producer_to_single_consumer_for_message();
    // single_producer_to_single_consumer_for_messages();
    multi_producer_to_single_consumer_for_messages();
}

// [case-1] 通过channel完成线程间通信（单发单收单消息）
// 注意：
//      <1> Receiver的recv方法会阻塞主线程
//      <2> Receiver的try_recv方法不会阻塞主线程
fn single_producer_to_single_consumer_for_message() {
    // 创建一个mpsc（Multi-Producer Single-Consumer）模式的数据管道channel
    // 返回一个元组，其中tx为发送者，rx为接收者
    // tx: Sender<String>
    // rx: Receiver<String>
    let (tx, rx) = std::sync::mpsc::channel();
    
    // 在子线程中将数据发送到channel中
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        let messgae = String::from("Hi");
        // 注意：这里Sender的send方法发送消息之后，messgae变量的所有权被移入到send方法中，后续将不能使用
        tx.send(messgae).unwrap();
        // println!("send message: {}", messgae); // ❌ borrow of moved value: `messgae`
    });

    // 主线程从管道中接受子线程发送的数据 
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// [case-2] 通过channel完成线程间通信（单发单收多消息）
fn single_producer_to_single_consumer_for_messages() {
    let (tx, rx) = std::sync::mpsc::channel();
    
    thread::spawn(move || {
        let mail_box = vec![
            String::from("I"), 
            String::from("am"), 
            String::from("a"), 
            String::from("robot")];
        for m in mail_box {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(2));
        }        
    });

    // 通过Receiver遍历多个消息
    for received in rx {
        println!("Got: {}", received);
    }
}

// [case-3] 通过channel完成线程间通信（多发单收多消息）
fn multi_producer_to_single_consumer_for_messages() {
    let (tx1, rx) = std::sync::mpsc::channel();
    let tx2 = tx1.clone();
    
    // 生产者1
    thread::spawn(move || {
        let mail_box = vec![
            String::from("[1] I"), 
            String::from("[1] am"), 
            String::from("[1] a"), 
            String::from("[1] robot")];
        for m in mail_box {
            tx1.send(m).unwrap();
            thread::sleep(Duration::from_secs(2));
        }        
    });

    // 生产者2
    thread::spawn(move || {
        let mail_box = vec![
            String::from("[2] I"), 
            String::from("[2] am"), 
            String::from("[2] a"), 
            String::from("[2] T800")];
        for m in mail_box {
            tx2.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }        
    });

    // 通过Receiver遍历多个消息
    for received in rx {
        println!("Got: {}", received);
    }
}