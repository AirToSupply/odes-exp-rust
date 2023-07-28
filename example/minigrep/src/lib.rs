// 参数配置结构体
pub struct Config {
    pub query: String,
    pub filename: String,
    // 是否是大小写敏感 
    pub case_sensitive: bool
}
 
impl Config {
    // 通过工厂方法解析参数
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("参数不够！");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // 从环境变量中读取是否大小写敏感
        // 注意：如果环境中没有环境变量CASE_INSENSITIVE则is_err返回true 代表读取环境变量失败
        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// 返回类型的Box<dyn Error>表示返回任意类型Error 
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>>{
    // 读取文件内容
    let content = std::fs::read_to_string(config.filename)?; // content: String 

    // if表达式
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    println!("<content>");

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

// 定义搜索函数（大小写敏感）
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
     let mut results = Vec::new();
     for line in content.lines() {
        if line.contains(query)  {
            results.push(line);
        }
     }
     results
}

// 定义搜索函数（大小写不敏感）
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in content.lines() {
       if line.to_lowercase().contains(&query.to_lowercase())  {
           results.push(line);
       }
    }
    results
}

/*
   单元测试
 */
#[cfg(test)]
mod tests {
    use super::*;

    // 大小写敏感
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    // 大小写不敏感
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
    
}
