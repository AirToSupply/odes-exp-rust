fn main() {
    // 1.整数
    // 十进制
    let a: i32 = 98_222;
    // 十六进制
    let b: i32 = 0xff;
    // 八进制
    let c: i32 = 0o77;
    // 二进制
    let d: i32 = 0b1111_0000;
    // 字节
    let e: u8 = b'A';
    // [error] literal out of range for `u8`
    // let f: u8 = 256;

    // 2.浮点数
    // 注意：
    //      <1> Rust中浮点数在不显示声明的情况下是双精度浮点数
    let f = 3.0; // --> let f: f64 = 3.0;
    let g: f32 = 2.0;
    
    // 基本数值运算
    let sum = 5 + 10;
    let diff = 85.1 - 1.7;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 43 % 5;

    // 注意：
    //      一个整数和浮点数可以相加吗？
    let a = 10;
    let b = 5.0;
    // [ERROR] cannot add a float to an integer
    // println!("a + b = {}", a + b);
    println!("a + b = {}", a as f64 + b);
    
    // 3.布尔值
    let t = true;
    let f = false;

    // 4.字符
    // 注意：
    //      <1> 它表示`unicode`字符，需要使用单引号扩起来
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';  
    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

    // 5.复合类型-元组（tuple）
    let tuple = ("珠穆朗玛峰", 8848.13); // tuple: (&str, f64)
    let (name, height) = tuple;
    println!("{} 的高度为 {} 米", name, height);

    // 6.复合类型-数组
    // 注意：
    //      <1> 数组中的元素类型必须是相同类型
    //      <2> 数组的长度是固定的，Rust在编译期数组的长度就必须确定
    let error_codes = [200, 404, 500]; // error_codes: [i32; 3]
    let not_found_code = error_codes[1];
    // [ERROR] index out of bounds: the length is 3 but the index is 3
    // let unkown_code = error_codes[3];
    // 创建一个具有8个元素的数组，并且所有元素初始化为0
    let byte = [0; 8]; // byte: [i32; 8]
}
