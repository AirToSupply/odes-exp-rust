use std::collections::HashMap;

fn main() {
    create_map();
    get_map_value();
    foreach_map();
    update_map();
    word_count();
}

// [case-1] 创建map并插入元素
fn create_map() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut map: HashMap<String, i32> = HashMap::new();

    // insert方法会获取key的所有权
    map.insert(blue, 10);
    map.insert(yellow, 20);

    println!("map : {:?}", map);

    // [ERROR] borrow of moved value: `blue`
    // println!("blue: {}", blue);
}

// [case-2] 通过key获取map的value值
fn get_map_value() {
    let blue = String::from("Blue");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(blue, 10);

    let team_name = String::from("Blue");
    let value = scores.get(&team_name); // value: Option<&i32>
}

// [case-3] 遍历map
fn foreach_map() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow") , 20);
    for (k, v) in &map { // k: &String  v: &i32
        println!("key = {}, value = {}", k, v)
    }
}

// [case-4] 更新map
fn update_map() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Blue") , 20); // 会覆盖
    println!("map : {:?}", map);

    // 如果key不存在就插入，如果存在就不插入
    map.entry(String::from("Yellow")).or_insert(30);
    map.entry(String::from("Yellow")).or_insert(40); // 不会覆盖
    println!("map : {:?}", map);
}

// [case-5] 通过map完成单词计数
fn word_count() {
    let content = "Hello World Hello Rust";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in content.split_whitespace() { // word: &strs
        let count = map.entry(word).or_insert(0); // count: &mut i32
        *count += 1;
    }
    println!("map : {:?}", map);
}