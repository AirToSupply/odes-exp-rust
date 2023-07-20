fn main() {
    let number_list = vec![6, 2, 3, 1, 8];
    println!("max number : {}", get_largest(number_list));

    let char_list = vec!['H', 'e', 'l', 'l', 'o', 'R', 'u', 's', 't'];
    println!("max character : {}", get_largest_char(char_list));

    println!("[generic] max number: {}", largest(vec![6, 2, 3, 1, 8]));
    println!("[generic] max character: {}", largest(vec!['H', 'e', 'l', 'l', 'o', 'R', 'u', 's', 't']));

    mix_generic(); 
}

// [case-0] 求数组中的最大元素
// 注意：
//      这种方法对Vec中的类型进行的限定，如果需要比较浮点数类型的Vec，则该函数无法具备一定的通用性
fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for num in number_list {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn get_largest_char(char_list: Vec<char>) -> char {
    let mut max = char_list[0];
    for c in char_list {
        if c > max {
            max = c;
        }
    }
    max
}

// [case-1] 通过泛型让函数具有一定的通用性，优化[case-1]
// 注意：
//      <1> 为了让泛型具有可比较性，需要对泛型进行约束和界定
//      <2> ParialOrd是一个trait，表示用于可比较的实现，这里表示传入函数的泛型必须实现了ParialOrd这个特征
//      <3> 泛型T可以有多个约束，多个约束通过 `+` 操作符号指定
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut max = list[0]; // max: T
    for c in list {
        if c > max {
            max = c;
        }
    }
    max
}

// [case-2] 结构体泛型
struct Location<T, U> {
    x: T,
    y: U,
}

fn struct_generics() {
    let p1 = Location{x: 5, y: 6};      // p1: Location<i32, i32>
    let p2 = Location{x: 5, y: 8.9};    // p2: Location<i32, f64>
    let p3 = Location{x: 5.1, y: 6.2};  // p3: Location<f64, f64>
}

// [case-3] 结构体实现块的泛型
struct Points<T> {
    x: T,
    y: T,
}

impl<T> Points<T> {
    // 方法返回值表示返回泛型T的引用 
    fn x(&self) -> &T {
        &self.x
    }
 }

 // [case-4] 对结构体实现块特定类型的定义
 // 注意：
 //      这个结构体实现块只适用于f64类型的Point结构体
 impl Points<f64> {
    // 方法返回值表示返回泛型T的引用 
    fn y(&self) -> f64 {
        self.y
    }
 }

 // [case-5] 泛型混用的案例
 struct Tuple<T, U> {
    x: T,
    y: U,
 }

 impl <T, U> Tuple<T, U> {
    // self泛型是<T, U>
    // other泛型是<V, W>
    // 这里可以是完全不同的
    fn mixup<V, W>(self, other: Tuple<V, W>) -> Tuple<T, W> {
        Tuple {
            x: self.x,
            y: other.y,
        }
    }
 }

 fn mix_generic() {
    let t1 = Tuple{x: 5, y: 10.4};           // t1: Tuple<i32, f64>
    let t2 = Tuple{x: "Hello", y: "Rust"}; // t2: Tuple<&str, &str>
    let t3 = t1.mixup(t2);                  // t3: Tuple<i32, &str>
    println!("Tuple<{}, {}>", t3.x, t3.y);
 }

// [case-5] 枚举泛型
enum Option<T> {
    Some(T),
    None,
}

fn enum_generic() {
    let integer = Option::Some(1);  // integer: Option<i32>
    let float = Option::Some(4.3);  // float:   Option<f64>
}
// 注意
/*
    在编译期，Rust会将泛型具体化，也就是说Option中的泛型T是会插除掉，大致变成下面这样：
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    fn enum_generic() {
        let integer: Option_i32 = Option_i32::Some(1);
        let float: Option_f64 = Option_f64::Some(4.3);
    }
*/