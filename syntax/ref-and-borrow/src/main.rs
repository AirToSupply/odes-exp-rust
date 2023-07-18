fn main() {
    // [case-1]
    // 引用
    // 注意：
    //      <1> 引用不具有所有权，自然也就不会发生所有权的转移，因为只是借用某个变量
    //      <2> 向函数传入引用参数的行为倍称之为借用（borrw），因此只是借用某个数据，不会发生这个数据所有权的转移 
    //      <3> 引用默认是不可变的
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // 因为s1没有发生的所有权的转移，所有下面打印仍然是可以使用的 
    println!("s = {}, len = {}", s1, len);

    // [case-2]
    // 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 = {}", s2);
}

// 函数接受String类型引用，这样不会让实参的所有权发生转移
// 这里s不具有变量s1的所有权
// 在内存中如下：
// 
//   s           s1          heap
//   ptr --->    ptr --->    "hello"
//               len 5
//               capacity 5
fn calculate_length(s: &String) -> usize {
    s.len()
} // 当函数退出，引用变量s失效 

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}