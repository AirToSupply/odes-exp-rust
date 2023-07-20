// [case-4] 使用trait bound特征作为函数参数对泛型进行界定
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 通过trait bound对泛型进行界定
// 这里表示泛型T必须实现Summary这个特征
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());s
}

// trait bound可以对泛型界定多个特征
// 这里表示型T必须同时实现Summary和Display两个特征
pub fn notify_and_display<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());s
}

// 当函数参数很多，而且每个参数可能又是泛型且又有很多界定，可以通过下面方式简化
// 下面这个例子要求
//    <1> 泛型T需要同时实现Display和Clone两个特征
//    <2> 泛型U需要同时实现Clone和Debug两个特征

// 方法一：
fn trait_bound_generic<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// 方法二：通过where关键字简化trait bound
fn trait_bound_generic<T, U>(t: &T, u: &U) -> i32 
where 
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// 测试方法
fn foo() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", notify(&tweet));
 
    println!("1 new tweet: {}", notify_and_display(&tweet));
}
