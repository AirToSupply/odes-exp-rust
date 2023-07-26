/*
    模式匹配使用场景
 */
fn main() {
    match_literal();
    match_var();
    matchs();
    match_range();
    match_struct();
    match_enum();
    match_nested_enum();
    match_nested_complex();

    ignore_var_for_func();
    ignore_part_var_for_nested();
    ignore_part_var_for_tuple();
    ignore_unuse_var();
    ignore_unuse_var_for_if_let_binding();
    ignore_unuse_var_for_if_let_no_binding();
    ignore_remaining_var_for_range();

    match_guard();

    match_binding();
}

// [case-1] 匹配字面值
fn match_literal() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// [case-2] 匹配命名变量
// 注意：
//      匹配块会重新创建一个新的作用域
//      代码<1>处的匹配块会遮蔽代码<2>定义的变量y
/*
    运行结果：
    Matched, y = 5
    at the end: x = Some(5), y = 10
 */
fn match_var() {
    let x = Some(5);
    let y = 10; //<2>

    match x {
        Some(y) => println!("Matched, y = {y}"), // <1>
        Some(50) => println!("Got 50"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

// [case-3] 匹配多个模式
fn matchs() {
    let x = 1;

    match x {
        // 这里通过或操作符，意思就是当x匹配1或者x匹配2都会进入对应的匹配块
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// [case-4] 通过 ..= 匹配值的范围
fn match_range() {
    let x = 5;

    match x {
        // x 是 1、2、3、4 或 5，第一个分支就会匹配
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// [case-5] 解构结构体
struct Point {
    x: i32,
    y: i32,
}

fn match_struct() {
    let p = Point { x: 0, y: 7 };
    // 解构后的变量名称未必要和结构体中的属性名称一致
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

// [case-6] 匹配枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn match_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

// [case-7] 匹配嵌套枚举
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum MessageX {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn match_nested_enum() {
    let msg = MessageX::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        MessageX::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        MessageX::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

// [case-8] 解构结构体和元组
fn match_nested_complex() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

// [case-10] 使用 _ 忽略整个值
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignore_var_for_func() {
    foo(3, 4);
}

// [case-11] 使用嵌套的 _ 忽略部分值
/*
    运行结构：
    Can't overwrite an existing customized value
    setting is Some(5)
 */
fn ignore_part_var_for_nested() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // 匹配
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }

    println!("setting is {:?}", setting_value);
}

// [case-12] 忽略元组的多个部分
fn ignore_part_var_for_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

// [case-13] 通过在名字前以一个 _ 开头来忽略未使用的变量
fn ignore_unuse_var() {
    let _x = 5;
    let y = 10; // 警告：if this is intentional, prefix it with an underscore: `_y`
}

// [case-14] 以下划线开头的未使用变量仍然会绑定值，它可能会获取值的所有权 
/*
    运行结果：
    found a string
 */
fn ignore_unuse_var_for_if_let_binding() {
    let s = Some(String::from("Hello!"));

    // 虽然if let匹配中变量s标记了_ 但是变量s还是绑定到了未使用的_s上，所以if let匹配匹配时成功
    // 同时变量s的所有权也发生的转移，导致后续使用失效
    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s); // ❌ borrow of partially moved value: `s`
}

// [case-15] 单独使用下划线不会绑定值
fn ignore_unuse_var_for_if_let_no_binding() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

// [case-16] 用 .. 忽略剩余值
fn ignore_remaining_var_for_range() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        // 通过使用 .. 来忽略 Point 中除 x 以外的字段
        Point { x, .. } => println!("x is {}", x),
    }


    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // 只匹配元组中的第一个和最后一个值并忽略掉所有其它值
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // ❌
    /*
    match numbers {
        (.., second, ..) => {
                          // ❌ ^^ can only be used once per tuple pattern
            println!("Some numbers: {}", second)
        },
    }
    */
}

// [case-17] 在模式中加入匹配守卫
fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x), // ✅
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // 使用匹配守卫来测试与外部变量的相等性
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x), // ✅
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

// [case-18] @ 绑定
fn match_binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // 测试通过之后 将id通过@绑定到id_variable变量中
        Message::Hello { id: id_variable @ 3..=7,} => 
            println!("Found an id in range: {}", id_variable), // ✅
        Message::Hello { id: 10..=12 } => 
            println!("Found an id in another range"),
        Message::Hello { id } => 
            println!("Found some other id: {}", id),
    }
}
