fn main() {
    define_enum();
    define_enum_by_type();
    define_enum_by_mix_type();

    enum_option();
    enum_option_operation();
    enum_option_match();

    if_let_replace_option_match();
}

// [case-1] 定义和使用枚举
fn define_enum() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// [case-2] 使用类似元组结构体的方式定义兼容混合数据类型的枚举
fn define_enum_by_type() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

// [case-3] 定义混合数据类型的枚举
fn define_enum_by_mix_type() {
    enum Message {
        // 不存放数据
        Quit,
        // 存放一个匿名结构体
        Move { x: i32, y: i32 },
        // 存放一个字符串
        Write(String),
        // 存放三个整形
        ChangeColor(i32, i32, i32),
    }
    let quit = Message::Quit;
    let _move = Message::Move{ x: 19, y: 32};
    let write = Message::Write(String::from("Writing"));
    let change_color = Message::ChangeColor(256, 256, 256);
}

// [case-4] Rust中的可选枚举
/*
 enum Option<T> { // T表示泛型，表示可以使任意数据类型
    // 有值的情况
    Some(T),
    // 没有值的情况
    None,
 }
 */
fn enum_option() {
    let some_number = Some(5);  //  some_number: Option<i32>
    let some_char = Some('e'); //  some_char  : Option<char>
    let absent_number: Option<i32> = None;   //  注意： 当为None需要显式标注类型
    println!("some_number value: {:?}", some_number);
    println!("some_char value: {:?}", some_char);
    println!("absent_number value: {:?}", absent_number);
}

// [case-5] 可选枚举参与运算
fn enum_option_operation() {
    let x: u8 = 7;
    let y: Option<u8> = Some(4);
    // unwrap_or方法表示当Option为None时用指定的默认值进行填充
    let sum = x + y.unwrap_or(0);
    println!("x + y = {}", sum);

    let y: Option<u8> = None;
    let sum = x + y.unwrap_or(0);
    println!("x + y = {}", sum);
}

// [case-6] match匹配枚举
// 注意：
//      <1> match表达式进行匹配时需要匹配所有枚举的情况，否则无法通过编译
enum CoinX {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_centsx(coin: CoinX) -> u8 {
    match coin {
        CoinX::Penny => 1,
        CoinX::Nickel => 5,
        CoinX::Dime => 10,
        CoinX::Quarter => 25,
    }
}

// [case-7] match匹配枚举进行值绑定
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter分支用于匹配进行值绑定
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// [case-8] match匹配Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plus_one_or_absent(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}

fn enum_option_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one_or_absent(None);
    println!("six value: {:?}", six);
    println!("none value: {:?}", none);
}

// [case-9] if let 语法简化 option + match匹配
fn if_let_replace_option_match() {
    let some_value: Option<i32> = Some(2);

    if let Some(2) = some_value {
        println!("two!")
    } else {
        println!("not match!")
    }
}