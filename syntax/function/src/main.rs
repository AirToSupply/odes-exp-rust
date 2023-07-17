fn main() {

    foo();

    let s = sum(9, 6);
    println!("function `sum` value: {}", s);

    let arr = [10.0, 20.0, 30.0, 40.0];
    println!("function `avg` value: {}", avg(&arr));
}

// [case-1]
// 定义无参函数
fn foo() {
    println!("function `foo`");
}

// [case-2]
// 定义有参有返回值函数
// 注意：
//      <1> 在Rust中函数的最后一个表达式的值为函数的返回值
fn sum(x: i32, y: i32) -> i32 {
    println!("x value is: {}", x);
    println!("y value is: {}", y);
    let _sum  = x + y;
    // 返回值方式一
    // return _sum;
    // 返回值方式二
    x + y
}

// [case-3]
// 定义一个函数求数组的平均值
fn avg(arr: &[f64]) -> f64 {
    let mut total = 0.0;
    for ele in arr.iter() {
        total += *ele
    }
    return total / arr.len() as f64;
}
