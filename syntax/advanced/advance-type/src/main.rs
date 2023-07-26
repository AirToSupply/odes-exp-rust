fn main() {
    type_alias();
    type_alias_for_complex_type();
}

// [case-1] 类型别名（type alias）
// 作用：类型别名的主要用途是减少重复，或者对定义复杂的类型进行重命名完成简化
fn type_alias() {
    // 为基本数据类型i32定义一个别名Kilometers
    type Kilometers = i32;

    
    let x = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {

}

// 对复杂的数据类型定义别名
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type_for_complex(g: Thunk) {

}

fn type_alias_for_complex_type() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    takes_long_type(f);

    let g: Thunk = Box::new(|| println!("hi"));
    takes_long_type_for_complex(g);
}
