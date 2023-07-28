/*
    实现类型linux中grep命令

    使用方式：

    <1> 大小写敏感模式：在poem.txt文件中查询相关frog的内容
    cargo run frog poem.txt

    <2> 大小写无敏感模式：在poem.txt文件中查询相关to的内容
    CASE_INSENSITIVE=1 cargo run to poem.txt
 */
fn main() {
    // 通过env库下的args方法收集命令行参数
    let args = std::env::args(); // args: Args
    // cargo run 
    // <args>: ["target/debug/minigrep"]
    // cargo run query test.txt
    // <args>: ["target/debug/minigrep", "query", "test.txt"]
    // 注意：默认第一个元素输出的是执行当前程序的二进制命令的相对路径
    // println!("<args>: {:?}", args);
    let args: Vec<String> = args.collect();

    // Config::new返回类型是Result<Config, &str>
    // 通过调用unwrap_or_else表示当没有panic则返回Config；有panic则返回unwrap_or_else方法中闭包的逻辑
    // 这里当参数解析失败之后直接退出当前进程
    let config: minigrep::Config = minigrep::Config::new(&args).unwrap_or_else(|err: &str| {
        // 将错误信息输出到标准错误输出流
        eprintln!("解析参数出错：{}", err);
        // 退出程序
        std::process::exit(1);
    });

    println!("<query>: {}, <filename>: {}", config.query, config.filename);

    if let Err(err) = minigrep::run(config) {
        eprintln!("应用出错：{}", err);
        std::process::exit(1);
    }
}