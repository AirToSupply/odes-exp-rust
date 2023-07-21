fn main() {
    println!("Hello, world!");
}

// [case-1] 悬挂指针和借用检查器（borrow checker）
/*
fn dangle_ptr() {
    let r: &i32;

    {
        let x = 5;
        // [ERROR] `x` does not live long enough
        //         borrowed value does not live long enough
        r = &x; // ❌ 
    } // 这个点 由于当前作用域结束导致变量x失效了，即生命周期结束

    println!("r: {}", r); // 到这个点，此时r指向了一个无效地址，所以r就为悬挂指针
}
*/

// ❓ Rust是如何知道这个代码在编译期不能通过呢？
// 在Rust中是通过借用检查器（borrow checker）来检查所有变量和指针的有效性，所以这就是为什么Rust能够在编译期检查出悬挂指针

// ❓ 上面的例子是如何通过借用检查器（borrow checker）来进行进行检查的呢？
/*
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
*/
// 将 r 的生命周期标记为 'a
// 将 x 的生命周期标记为 'b
// 在编译时，Rust 比较这两个生命周期的大小，并发现 r 拥有生命周期 'a，不过它引用了一个拥有生命周期 'b 的对象
// 因为生命周期 'b 比生命周期 'a 要小：
// [重要] 比较好的理解就是：被引用的对象比它的引用者存在的时间更短
//              也就是说：如果能够保证被引用对象的生命周期比引用者的生命周期尽可能的大就不会出现在编译期异常

// [case-2] 没有产生悬挂指针
// 注意：
//       一个有效的引用，因为数据比引用有着更长的生命周期
fn non_dangle_ptr() {
    let x = 5;             // ----------+-- 'b
                                //           |
    let r = &x;           // --+-- 'a  |
                                //   |       |
    println!("r: {}", r);       //   |       |
                                // --+       |
}                               // ----------+

// [case-3] 函数中的泛型引发的生命周期
/*
// 一个 longest 函数的实现，它返回两个字符串 slice 中较长者
// [ERROR] 
//         missing lifetime specifier
//         this function's return type contains a borrowed value, 
//         but the signature does not say whether it is borrowed from `x` or `y`
fn longest(x: &str, y: &str) -> &str { // ❌
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn func_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
*/

// 注意：
//      函数的返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y。
//      事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用，而且x 和 y的生命周期可能是不同的！
//      ✅ 所以在Rust中需要通过 生存期注解 来让Rust编译器进行生命周期的判断

// [case-4] 在函数中使用生存期注解解决[case-3]
// 注意：
//      <1> 这里的a表示一个生存期的名称，可以是任意字符
//      <2> 生存期注解并不改变生存期，这是表达他们生存期之间的关联关系
//          这个例子中：将如入参x和y已经返回值都标记为'a，表示：函数返回值的生存期等于x和y中较小的那一个生存期
//      <3> 生存期注解也是一种的泛型，也是通过<>中进行标注
//      <4> 生存期注解并不一定要定义在函数中，比如下面的例子
//             &i32        // 引用
//             &'a i32     // 带有显式生命周期的引用
//             &'a mut i32 // 带有显式生命周期的可变引用
//      <5> 对于函数来说传入和传出的生存期必须是相关关联的，特别需要注意引用类型 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ✅
fn func_lifetime_x() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// ✅
// result --> string1
// result --> string2
// 此时 string2的生存期较小
// result和string2基本平齐
fn func_lifetime_y() {
    let string1: String = String::from("long string is long");                // ----------------------------------------| - 'string1
                                                                              //                                         |
    {                                                                         //                                         |
        let string2 = String::from("xyz");                            // --------------------------|- 'string2   |
        let result = longest(string1.as_str(), string2.as_str());  // -------|- 'result         |             |
        println!("The longest string is {}", result);                         // -------|------------------|             |
    }                                                                         //                                         |
                                                                              // -------|--------------------------------|
}
// ❌
// result --> string1
// result --> string2
// 此时 string2的生存期较小
// 但是被引用者string2生存期小于引用者result的生存期，所以无法通过编译
/*
fn func_lifetime_z() {
    let string1 = String::from("long string is long");         // ----------------------------------------| - 'string1
    let result;                                                // --------------------| - 'result         |
    {                                                          //                     |                   |
        let string2 = String::from("xyz");                     // --- ｜- 'string2    |                   |
        // [ERROR] `string2` does not live long enough         //     |               |                   |
        //         borrowed value does not live long enough    //     |               |                   |
        // string2.as_str() ❌                                 //     |               |                   |
        result = longest(string1.as_str(), string2.as_str());  // --- |               |                   |
    }                                                          //                     |                   |
    println!("The longest string is {}", result);              // --------------------|-------------------|
}
*/

// [case-4] 结构体中的生命周期
// 注意：
//      生存期注解表明，结构体的生存周期不能大于part字段中的生存期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// first_sentence (&str) --> i(struct: ImportantExcerpt)
fn struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// [case-5] 生命周期规则
//   前置概念：如果有一个函数: fn(a: i32) -> i32 
//           这里的参数入参数被称为输入生存期
//           这里的参数返回值被称为输出生存期
//   Rust编译期对于生命周期有如下规则，在不需要显式标注生存期注解的前提下，编译期会自动推导
//
//   <1> 每一个引用参数都是一个具有生存期的参数
//   <2> 如果只有一个输入生存期参数，那么这个生存期就会被赋给所有的输出生存期参数
//   <3> 如果有多个生存期参数，但是其中一个是&self或者&mut self，那么self的生存期会被赋给所有的输出生存期
//
//   注意：如果上述规则仍然没有推导出生存期在编译期就会报错，此时需要手工对参数的生存期进行标注

// 案例一：
/*
fn first_word<'a>(s: &'a str) -> &'a str {
*/
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 案例二：
/*
struct ImportantExcerpt<'a> {
    part: &'a str,
}
*/

impl<'a> ImportantExcerpt<'a> {
    // 此时&self的生存期会赋给返回类型&str指向的变量的生存期 
    /*
    fn announce_and_return_part(&a' self, announcement: &str) -> &a' str {
     */
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/*
fn struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
*/

// [case-6] 泛型类型参数 + trait bounds + 生命周期定义函数
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str, // 生存期注解(输入生存期)
    y: &'a str, // 生存期注解(输入生存期)
    ann: T,     // 泛型
) -> &'a str    // 生存期注解(输出生存期)
where
    T: Display, // 泛型特征界定
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn mix() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}