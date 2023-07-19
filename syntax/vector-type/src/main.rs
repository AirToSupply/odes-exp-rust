fn main() {
    create_vector();
    push_vector();
    get_vector_by_index();
    get_vector_by_option();
    read_and_write_vector();
    foreach_vector();
    foreach_mut_vector();
    mix_data_type_vector();
}

// [case-1] 创建vector
fn create_vector() {
    // 方式一：创建一个类型为i32的空vector
    let v1: Vec<i32> = Vec::new();
    println!("[create] vec : {:?}", v1);
    // 方式二：通过vec宏创建并同时初始化vector
    let v2 = vec![10, 20, 30];
    println!("[create] vec : {:?}", v2); 
}

// [case-2] 向vector添加添加元素
fn push_vector() {
    let mut v1: Vec<i32> = Vec::new();
    println!("[push] vec : {:?}", v1);
    v1.push(1);
    v1.push(3);
    v1.push(5);
    println!("[push] vec : {:?}", v1);
}

// [case-3] 通过索引访问vector
fn get_vector_by_index() {
    let v = vec![10, 20, 30, 40, 50];
    let third = &v[2];
    println!("[get by index] third : {}", third);
}

// [case-4] 通过option match解决访问vector缺失
fn get_vector_by_option() {
    let v = vec![10, 20, 30, 40, 50];
    match v.get(10) {
        Some(value) => println!("vec[10] = {}", value),
        None => println!("vec[10] is none!"),
    }
}

// [case-5] 同时读写vector违反Rust可变形问题
fn read_and_write_vector() {
    let mut v = vec![10, 20, 30, 40, 50];
    // 获取vector元素，此时对v对产生一个不可变引用
    let third = &v[2];
    // 向vector添加一个元素，此时对v产生一个可可变引用
    // 此时对v同时有可变和不可变引用，违反了Rust的可变性规则
    // [ERROR] cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(100);
    println!("[get by index] third : {}", third);
}

// [case-6] 通过不可变的方式遍历vector中的元素
fn foreach_vector() {
    let mut v = vec![10, 20, 30, 40, 50];
    for element in &v { // element: &i32
        println!("[foreach] element: {}", element)
    }
}

// [case-7] 通过可变的方式遍历vector中的元素 对vector中每个元素加10
fn foreach_mut_vector() {
    let mut v = vec![10, 20, 30, 40, 50];
    for element in &mut v { // element: &mut i32
        *element += 10
    }
    for element in &v { // element: &i32
        println!("[foreach mut] element: {}", element)
    }
}

// [case-8] 通过枚举包装不同数据类型使vector存储不同数据类型的值
fn mix_data_type_vector() {
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let rows = vec![
        Cell::Int(10), 
        Cell::Float(3.14), 
        Cell::Text(String::from("Rust"))
    ];

    match &rows[0] {
        Cell::Int(value) => println!("vec[0] = {}", value),
        _ => println!("not matched!")
    }
}