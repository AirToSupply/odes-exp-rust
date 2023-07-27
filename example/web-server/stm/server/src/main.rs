use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/*
    单线程WebServer-服务端
 */
fn main() {
    // 通过TCP协议绑定IP地址和端口号，进行对7878这个端口的监听
    // 因为绑定过程中可能发生异常，例如端口冲突等，所以采用unwrap方法做简单的异常处理，在一般生产环境中需要更加复杂异常处理来保证代码的健壮性
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap(); // listner: TcpListener

    // incoming方法用于获取建立连接的迭代器，这里对迭代器中的每个Result进行循环
    for result in listner.incoming() { // result: Result<TcpStream, Error>
        // 通过unwrap方法从Result获取TcpStream对象 
        let stream = result.unwrap(); // stream: TcpStream
        // 处理当前请求连接
        handle_connection(stream);
    }
}

// 定义处理请求连接的方法
// 这里需要对TCP流的状态进行改变，所以需要声明为可变类型
fn handle_connection(mut stream: TcpStream) {
    // 定义一个1024大小的缓冲
    let mut buffer = [0; 1024];
    // 从TCP流中读取数据至缓冲中
    stream.read(&mut buffer).unwrap();

    // 这里接受到的请求内容类似如下：
    // GET / HTTP/1.1
    // Host: localhost:7878
    // Connection: keep-alive
    // Cache-Control: max-age=0
    // sec-ch-ua: "Not.A/Brand";v="8", "Chromium";v="114", "Google Chrome";v="114"
    // sec-ch-ua-mobile: ?0
    // sec-ch-ua-platform: "macOS"
    // Upgrade-Insecure-Requests: 1
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
    // Sec-Fetch-Site: none
    // Sec-Fetch-Mode: navigate
    // Sec-Fetch-User: ?1
    // Sec-Fetch-Dest: document
    // Accept-Encoding: gzip, deflate, br
    // Accept-Language: zh-CN,zh;q=0.9
    
    // 这里from_utf8_lossy关联函数是将一组字节切片转为一个字符串
    // println!{"[Request]: {}", String::from_utf8_lossy(&buffer[..])};

    // 下面将输出一段响应，大致内容类似如下：
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    // 例如：这是一个使用 HTTP 1.1 版本的响应例子，其状态码为 200，原因短语为 OK，没有 header，也没有 body
    // HTTP/1.1 200 OK\r\n\r\n

    // 定义正常请求标识头
    // 这里的b表示将一个字符串切片转换为u8类型数组
    let get_body_prefix = b"GET / HTTP/1.1\r\n";

    // 匹配预期的请求是GET请求。并且请求路径是/则返回欢迎页面，则否返回404页面
    let (status_line, resource_filename) = if buffer.starts_with(get_body_prefix) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // 读取本地html资源文件
    let content = fs::read_to_string(resource_filename).unwrap();
    // 构建响应体
    let reponse = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content.len(), content);
    // 将响应内容转变为字节切片写入TCP流缓冲中
    stream.write(reponse.as_bytes()).unwrap();
    // flush会等待所有的字节写入到TCP流中才会返回Result
    stream.flush().unwrap();


    
    
}
