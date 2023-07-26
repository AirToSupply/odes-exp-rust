fn main() {
    overload_add_ops();
    overload_add_ops_for_diff_type();
    call_func_for_traits();
    call_inline_func_for_traits();
    super_trait();
}

// -----------------------------------------------------------------------------
// [case-1] 关联类型在 trait 定义中指定占位符类型
// 注意：
//       <1> 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型。（用于占位）
//       <2> trait 的实现者会针对特定的实现在这个占位符类型指定相应的具体类型。（由实现者在关联类型上指定具体的类型）

// 定义一个带有关联类型的迭代器特征
trait IteratorBasedOnAssociatedTypes {
    // 定义一个关联类型，名字叫做Item
    // 也就是说Item 是一个占位符类型
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

// 定义一个实现者
struct CounterBasedOnAssociatedTypes {

}

impl IteratorBasedOnAssociatedTypes for CounterBasedOnAssociatedTypes {
    // 指定关联类型为i32
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// ❓ 定义trait中定义关联类型和trait中定义泛型有什么区别？
// ✅ [最佳实践]
//       <1> 针对一个类型的单独实现，推荐使用trait+关联类型
//       <2> 针对多个类型的多个实现，推荐使用trait+泛型类型

// 再次为CounterBasedOnAssociatedTypes实现者定义一个关联类型为f64的实现块
// ❌
// conflicting implementations of trait `IteratorBasedOnAssociatedTypes` for type `CounterBasedOnAssociatedTypes`
// 编译器的提示告诉我们：IteratorBasedOnAssociatedTypes存在有相互冲突的实现
//                即：一个实现者不能两个不同关联类型实现的triat实现块
/*
impl IteratorBasedOnAssociatedTypes for CounterBasedOnAssociatedTypes { // ❌
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
*/

// 考察通过在trait中定义泛型来进行不同数据类型的实现
// 定义一个泛型迭代器
trait IteratorBasedOnGenericTypes<T> {
    fn next(&mut self) -> Option<T>;
}

struct CounterBasedOnGenericTypes {

}

// 定义一个i32的实现块
// ✅ 编译通过，没有定义冲突
impl IteratorBasedOnGenericTypes<i32> for CounterBasedOnGenericTypes {
    fn next(&mut self) -> Option<i32> {
        todo!()
    }
}

// 定义一个f64的实现块
// ✅ 编译通过，没有定义冲突
impl IteratorBasedOnGenericTypes<f64> for CounterBasedOnGenericTypes {
    fn next(&mut self) -> Option<f64> {
        todo!()
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
// [case-2] 运算符重载（Operator overloading）
// 注意：
//       Rust 不允许创建自定义运算符或重载任意运算符，不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载

// Point结构体运算符重载
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 实现Add特征，让Point结构体具体+操作符的重载
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x, 
            y: self.y + rhs.y,
        }
    }
}

fn overload_add_ops() {
    let x: Point = Point{x: 1, y: 2};
    let y = Point{x: 9, y: 8};
    let z = x + y;
    println!("x = {:?}", x);
    println!("y = {:?}", y);
    println!("z = {:?}", z);
}

// std::ops::Add特征的定义如下：
/*
pub trait Add<Rhs = Self> {

    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
 */
// Add特征的泛型Rhs默认情况下指定为Self表示当前类型，Rhs名称的意思就是操作符右手边的类型
// 例如：x + y这个表达式，泛型Rhs情况下为操作符+右边的变量y的类型。
// 上面的这个例子中操作符左右两边都是Point类型，所以在实现块中不需要定义泛型Rhs的类型

// 操作符两侧不同数据类型重载
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl std::ops::Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + 1000 * rhs.0)
    }
}

fn overload_add_ops_for_diff_type() {
    let x = Millimeters(1000);
    let y = Meters(3);
    let z = x + y; // z: Millimeters
    println!("z = {:?}", z);
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
// [case-3] 调用相同名称的方法
// 注意：
//       <1> Rust 不能避免一个 trait 与另一个 trait 拥有相同名称的方法, 无法阻止为同一类型同时实现这两个 trait
//           ❓ 假设A B两个特征中定义了相同方法签名的方法foo，类型X分别实现了A B中的foo方法，此时调用x.foo方法是哪个特征的方法？
//             （有点类型菱形继承）
//       <2> Rust 直接在类型上实现开始已经有的同名方法也是可能的。
//           ❓ 假设A特征有方法foo，类型X自生也有foo方法，同时类型X实现了A特性，此时x.foo调用的是自生的方法还是特征A的方法？

// 在多个特征中存在同名的方法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("[Pilot] This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("[Wizard] Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("[Human] *waving arms furiously*");
    }
}

// 上面定义可以表示为下图：
/*
       
       ---------------------
      | <<trait>> Pilot     |
       ---------------------
      | <fn> fly(&self)     | <|----------------------
       ---------------------                         |
                                                     |
                                                     |
                                                     |          ---------------------
                                                     |--------- | <<struct>> Human   |
                                                     |          ---------------------
                                                     |          | <fn> fly(&self)    | 
                                                     |          ---------------------
                                                     |
                                                     |
       ---------------------                         |
      | <<trait>> Wizard    |                        |
       ---------------------                         |
      | <fn> fly(&self)     | <|----------------------
       ---------------------
 */

fn call_func_for_traits() {
    let person = Human{};         // person: Human

    // 调用Human自身的fly方法
    person.fly();                        // [Human] *waving arms furiously*
    // 等价于下面的写法                     
    Human::fly(&person);           // [Human] *waving arms furiously*

    // 调用Human实现了Pilot特征的fly方法
    Pilot::fly(&person);            // [Pilot] This is your captain speaking.

    // 调用Human实现了Wizard特征的fly方法
    Wizard::fly(&person)            // [Wizard] Up!
}

// 在多个特征中存在同名关联函数
trait PilotX {
    fn fly();
}

trait WizardX {
    fn fly();
}

struct HumanX;

impl PilotX for HumanX {
    fn fly() {
        println!("[Pilot] This is your captain speaking.");
    }
}

impl WizardX for HumanX {
    fn fly() {
        println!("[Wizard] Up!");
    }
}

impl HumanX {
    fn fly() {
        println!("[Human] *waving arms furiously*");
    }
}

fn call_inline_func_for_traits() {
    // 调用HumanX自身的fly关联函数
    HumanX::fly();                           // [Human] *waving arms furiously*

    // 调用HumanX实现了PilotX特征的fly关联函数
    <HumanX as PilotX>::fly();               // [Pilot] This is your captain speaking.

    // 调用HumanX实现了WizardX特征的fly关联函数
    <HumanX as WizardX>::fly()               // [Wizard] Up!
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// [case-4] 父特征（super trait）
// 例如希望创建一个带有 outline_print 方法的 trait OutlinePrint，它会将给定的值周围格式化为带有星号框

trait OutlinePrint: std::fmt::Display {
                    // 这类似于为 trait 增加 trait bound
    fn outline_print(&self) {
        // 这里写希望self是实现了Display特征的类型，才可以调用这个to_string进行答应
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Location {
    x: i32,
    y: i32,
}

impl OutlinePrint for Location {}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

fn super_trait() {
    let location = Location {x: 1, y: 5 };
    location.outline_print();
}
// -----------------------------------------------------------------------------
