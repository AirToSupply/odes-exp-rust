use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use server::ThreadPool;

/*
    多线程WebServer-服务端
 */
fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap(); // listner: TcpListener
    // 初始化线程池，默认设置4个初始线程，这样可以同时接受4个请求处理
    let pool = ThreadPool::new(4);

    for result in listner.incoming() { // result: Result<TcpStream, Error>
        let stream = result.unwrap(); // stream: TcpStream
        // thread::spawn(|| handle_connection(stream));
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_body_prefix = b"GET / HTTP/1.1\r\n";
    let sleep_body_prefix = b"GET /sleep HTTP/1.1\r\n";

    // 匹配预期的请求是GET请求 并且请求路径是/     则返回欢迎页面，
    // 匹配预期的请求是GET请求 并且请求路径是/sleep则返回欢迎页面，并且在返回前通过睡眠堵住后需请求（在单线程环境下模拟请求阻塞）
    // 则否返回404页面
    let (status_line, resource_filename) = if buffer.starts_with(get_body_prefix) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep_body_prefix) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // 读取本地html资源文件
    let content = fs::read_to_string(resource_filename).unwrap();
    // 构建响应体
    let reponse = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content.len(), content);
    // 返回响应
    stream.write(reponse.as_bytes()).unwrap();
    stream.flush().unwrap();   
}
