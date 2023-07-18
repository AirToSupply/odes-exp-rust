fn main() {
    string_slice();
    slice_by_index();
    string_literal();
    array_slice();
}

// [case-1]
// 如何产生字符串切片
// 此时内存中堆栈布局如下:
/*
    <s>               <"hello world">
    ptr -------------> 0  'h'
    len=11             1  'e'
    capacity=11        2  'l'
                       3  'l'
                       4  'o'
    <world>            5
    ptr -------------> 6  'w'
    len=5              7  'o'
                       8  'r'
                       9  'l'
                       10 'd'
 */
// 注意：
//      <1> 字符串变量s具有对字符串"hello world"的所有权
//      <2> 字符串切片world是对字符串"hello world"的部分引用，不具有所有权
fn string_slice() {
    let mut s = String::from("hello world");
    // &str即为字符串切片类型
    let hello = &s[0..5];  // hello: &str  index from 0 to 4
    let world = &s[6..11]; // world: &str  index from 6 to 10
    println!("hello value from string slice: {}", hello);
    println!("world value from string slice: {}", world);
}

// [case-2]
// 通过索引取值字符串切片
fn slice_by_index() {
    let mut s = String::from("hello world");
    // 从索引0开始取值，此时0可以省略
    let hello = &s[..5];
    // 取值至字符串末尾结束，则下界可以省略
    let world = &s[6..];
    // 取整个字符串为切片
    let content = &s[..];
    println!("hello value: {}", hello);
    println!("world value: {}", world);
    println!("content value: {}", content);
}

// [case-3]
// 字符串字面量
// 注意：
//      <1> 变量s是对字符串切片的一个引用，它指向字符串字面量"hello world"在二进制可执行文件中的地址，所以变量是一个引用
fn string_literal() {
    let s = "hello world"; // s: &str
    let hello = &s[..5];
    let world = &s[6..];
    println!("hello value from string literal: {}", hello);
    println!("world value from string literal: {}", world);
}

// [case-4]
// 数组切片
fn array_slice() {
    let arr = [1, 2, 3, 4, 5]; // arr:   [i32; 5]
    let slice = &arr[0..2];      // slice: &[i32]
    println!("slice value from array: {:?}", slice);
}