fn main() {
    func_ptr();
    itr();
}

// [case-1] 函数指针
// 注意：
//       可以将函数作为参数传给函数

// 定义一个普通函数
fn add_one(x: i32) -> i32 {
    x + 1
}

// 定义一个具有函数参数的函数
// fn(i32) -> i32 表示一个函数指针，入参和出参都是i32类型
fn do_twice_for_func(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 定义一个具体泛型界定并且以闭包作为函数参数的函数
// 注意：这里将一个函数指针传给该函数，编译也是没有问题的，因为函数指针已经实现了Fn trait
fn do_twice_for_closure<T>(f: T, arg: i32) -> i32 
where T: Fn(i32) -> i32
{
    f(arg) + f(arg)
}

fn func_ptr() {
    let arg = 5;
    let f = add_one; // f: fn add_one(i32) -> i32

    let answer_one = do_twice_for_func(f, arg);
    let answer_two = do_twice_for_closure(f, arg);

    println!("The answer_one is: {}", answer_one);
    println!("The answer_two is: {}", answer_two);
}

// [case-2] 迭代模式下既可以传入闭包，也可以传入函数指针
fn itr() {
    let list_of_numbers = vec![1, 2, 3];

    // map函数接受闭包
    let _v1: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // map函数接受函数指针
    let _v2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

// [case-3] 从函数中返回闭包

// 注意： 这里无法返回具体的类型，需要使用impl特征语法
fn _return_closure() -> impl Fn(i32) -> i32  {
    |x: i32| x + 1
}

// 上面的例子返回是非常简单的，通常来时对于相对复杂的返回闭包上面的例子是没有办法工作的，比如下面这个例子
// 通过一个参数判断来返回不同的闭包
// 注意： 
//       <1> 即便两个闭包所返回的类型也是不相同的
//       <2> impl特征语法只能适应返回一种闭包的情况
/*
fn return_closures(a: i32) -> impl Fn(i32) -> i32 {
    if a > 0 {
       move |b: i32| a + b 
    } else {
       // ❌
       // `if` and `else` have incompatible types
       // no two closures, even if identical, have the same type
       move |b: i32| a - b  // ❌
    }
}
*/

// 通过Box智能指针+dyn动态派遣解决返回多闭包
// 注意：
//       <1> dyn Fn(i32) -> i32 是为了返回多闭包，因为每个闭包的类型是不同的，为了让Rust在运行时动态的根据逻辑判断确定的类型，所以这里要动态派遣
//       <2> 使用动态派遣的返回类型为Rust编译期无法知道返回类型确定的大小，所以通过Box智能指针进行包裹
fn _return_closures(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
       Box::new(move |b: i32| a + b)
    } else {
        Box::new(move |b: i32| a - b)
    }
}
