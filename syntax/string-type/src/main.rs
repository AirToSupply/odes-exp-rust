use unicode_segmentation::UnicodeSegmentation;

fn main() {
    create_string();
    append_string();
    add_string(); 
    format_string();
    understand_string();
}

// [case-1] 创建字符串
fn create_string() {
    // 方式一：创建一个空字符串
    let s1 = String::new();
    // 方式二：将一个字符串切片转换为一个字符串
    let s2 = "Hello Rust"; // s2: &str
    let s3 = s2.to_string();
    // 方式三：基于一个字符串切片创建为一个字符串
    let s4 = String::from("Hello Rust");

    // 字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

// [case-2] 追加字符串
fn append_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("s = {}", s)
}

// [case-3] 通过+运算符拼接字符串
fn add_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    // 由于 s1 + &s2 操作中将s1的所有权移入到了s3，后续s1将失效
    // [ERROR]
    // println!("s1 = {}", s1);
}

// [case-4] 通过format宏拼接字符串
fn format_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // format宏并不会获取s1和s2的所有权
    let s3 = format!("{}{}", s1, s2);
    println!("format s3 = {}", s3);
    println!("s1 = {}", s1);
    println!("s1 = {}", s2);
}

// [case-5] 能够通过字符串索引获取字符串中的某个字符
// 注意：
//      在Rust中的字符串不支持索引访问
fn index_from_string() {
    let s1 = String::from("hello");
    // [ERROR] the type `String` cannot be indexed by `{integer}`
    // let h = s1[0];
}

// [case-6] 字节、标量值和字形簇
// 注意：
//      从 Rust 的角度来讲，事实上有三种相关方式可以理解字符串：字节、标量值和字形簇
fn understand_string() {
    // 梵文：你好
    let hello = String::from("नमस्ते"); // 南🈚️斯特

    // 字节表现形式
    // 最终它储存在vector中的u8 值看起来像这样：
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for b in hello.bytes() { // b: u8
        print!("{} ", b)
    }
    println!();

    // 数量表现形式
    // Scalar数值量(char) 第四个和第六个是发音符本身没有意义
    // ['न', 'म', 'स', '्', 'त', 'े']
    for c in hello.chars() { // b: char
        print!("{} ", c)
    }
    println!();

    // 字形簇表现形式
    // 就会得到人们所说的构成这个单词的四个字母
    // ["न", "म", "स्", "ते"]
    // 注意：
    //      在Rust标准库中不支持访问字形簇表现形式的方法，需要引入第三方库unicode-segmentation(v1.7.1)
    for g in hello.graphemes(true) { // s: &str
        print!("{} ", g);
    }
    println!();
}