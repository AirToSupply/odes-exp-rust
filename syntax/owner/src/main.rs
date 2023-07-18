fn main() {
    // [case-1]
    // 基本数据类型赋值 演示Copy
    // 注意：
    //       <1> integer/boolean/character由于实现了Copy Trait，所以用该值赋值给其他变量其实是进行了一次数据的拷贝
    let x = 7;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // [case-2]
    // 字符串类型赋值 演示Move
    let s1 = String::from("hello");
    // 这一步进行了移入（Move）
    let s2 = s1;
    println!("s2 = {}", s2);
    // [ERROR] borrow of moved value: `s1`
    // println!("s1 = {}", s1);

    // [case-3]
    // 字符串类型赋值 演示Copy
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // [case-4]
    // 函数实参的传递导致所有权的转移
    let c: String = String::from("rust");
    take_ownership(c);
    // [ERROR] value borrowed here after move
    // println!("c = {}", c);

    // [case-5]
    // 函数形参传递时，如果形参数据类型实现了Copy Trait则不会发生移入，因为进行了数据的拷贝
    let num = 15;
    make_copy(num);
    println!("num = {}", num);

    // [case-6]
    // 解决函数实参的传递导致所有权的转移
    let w: String = String::from("going");
    // <1> 先移入至函数内
    // <2> 在由函数返回值将所有权移入至新变量，同时通过变量遮蔽
    let w = takes_and_gives_back(w);
    println!("w = {}", w);

}

fn take_ownership(some_string: String) {
    println!("some_string = {}", some_string)
}

fn make_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer)
}

fn takes_and_gives_back(a: String) -> String {
    a
}