use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    // micro_panic();

    // open_file_match_result();
    // open_file_and_touch_file();
    // open_file_and_touch_file_closure();

    // open_file_unwrap();
    // open_file_expect();

    // read_username_from_file();
    // read_username_from_file_question_mark();
    // read_username_from_file_method_cascading();
}

// [case-1] 通过panic宏触发错误
// 在Rust中panic一般代表不可恢复错误
fn micro_panic() {
    panic!("crash & burn");
}

// [case-2] 在Rust中一般通过Result枚举来处理可恢复错误
// 注意：
//      一般可以同构match表达式来对Result枚举进行匹配

/*
 enum Result<T, E> {
    Ok(T),  // 代表成功情况
    Err(E), // 代表失败情况
 }
*/
// 下面将通过打开本地文件的例子来进行演示
// 返回异常如下：
// thread 'main' panicked at '无法打开文件: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:??:??
// stack backtrace:
// ...
fn open_file_match_result() {
    let f = File::open("hello.txt"); // f: Result<File, Error>
    let v = match f { // v: File
        // file: File
        Ok(file) => file,
        // error: Error
        Err(error) => panic!("无法打开文件: {:?}", error),
    };
}

// [case-3] 当文件不存在创建文件
fn open_file_and_touch_file() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => 
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("无法创建文件: {:?}", e),
                },
            // other_error: ErrorKind
            other_error => 
                panic!("无法打开文件: {:?}", other_error),
        }
    };
}

// [case-4] 通过unwrap_or_else+闭包优化[case-3]
fn open_file_and_touch_file_closure() {
    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("无法创建文件: {:?}", error);
            })
        } else {
            panic!("无法打开文件: {:?}", error);
        }
    });
}

// [case-5] 通过unwrap方法代替用match表达式匹配Result
// 注意：
//      <1> 当发生程序异常时可调用Result的unwrap方法来触发panic
//      <2> unwrap不能定义error message
fn open_file_unwrap() {
    let f = File::open("hello.txt").unwrap(); // f: File
}

// [case-6] 通过expect方法代替用match表达式匹配Result
// 注意：
//      调用Result的expect方法来代替unwrap不能定义error message的不足
fn open_file_expect() {
    let f = File::open("hello.txt").expect("无法打开文件：hello.txt"); // f: File
}

// [case-7] 将异常返回给调用者
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// [case-8] 通过?操作符简化[case-7]
// 注意：
//       当语句正确执行则正确赋值给相关变量，当语句执行异常通过?操作符直接将异常返回给std::io::Error
fn read_username_from_file_question_mark() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?; // f: File
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// [case-9] 通过方法级联简化[case-8]
fn read_username_from_file_method_cascading() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// [case-10] 通过std::fs包下的read_to_string助手函数简化[case-9]
/*
   pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> { 
 */
fn read_username_from_file_helper() -> Result<String, std::io::Error> {
    std::fs::read_to_string("hello.txt")
}