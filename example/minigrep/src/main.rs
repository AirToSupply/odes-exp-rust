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
    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("解析参数出错：{}", err);
        // 退出程序
        std::process::exit(1);
    });

    println!("<cammond>: {}, <filename>: {}", config.cammond, config.filename);

    if let Err(err) = run(config) {
        println!("应用出错：{}", err);
        std::process::exit(1);
    }
}

// 参数配置结构体
struct Config {
    cammond: String,
    filename: String,
}
 
impl Config {
    // 通过工厂方法解析参数
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("参数不够！");
        }
        let cammond = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { cammond, filename })
    }
}

// 返回类型的Box<dyn Error>表示返回任意类型Error 
fn run(config: Config) -> Result<(), Box<dyn std::error::Error>>{
    // 读取文件内容
    let content = std::fs::read_to_string(config.filename)?; // content: String 

    println!("<content>\n {}", content);

    Ok(())
}
