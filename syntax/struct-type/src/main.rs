fn main() {
    define_struct();
    struct_attrs_set_get();
    struct_method();
    struct_inline_func();
}

// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 定义元组结构体，此时不需要写属性名
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

// [case-1]
// 声明义结构体
fn define_struct() {
    // 方式一
    let u1 = User {
        email: String::from("airtosupply@xxx.com"),
        username: String::from("airtosupply"),
        sign_in_count: 1,
        active: true,
    };

    // 方式二：如果赋值给结构体属性的变量名称和结构体属性名称一致，在声明结构体时可以省略赋值
    let username = String::from("airtosupply");
    let email = String::from("airtosupply@xxx.com");
    let u2 = User {
        email,
        username,
        sign_in_count: 2,
        active: true,
    };

    // 方式三：可以通过如下简写方式给新的结构体赋值，不必一个属性一个一个属性的赋值
    let u3 = User{
        ..u2
    };
}

// [case-2]
// 获取和修改属性
fn struct_attrs_set_get() {
    // 声明结构体
    let mut user = User {
        email: String::from("airtosupply@xxx.com"),
        username: String::from("airtosupply"),
        sign_in_count: 1,
        active: true,
    };
    // 获取属性
    println!("username: {}", user.username);
    println!("sign_in_count: {}", user.sign_in_count);
    // 修改属性
    // 注意：
    //      <1> 如果声明结构体是没有通过mut进行修饰，则无法修改结构体对象中的属性，会产生如下异常：
    //          [ERROR] cannot mutate immutable variable `user`
    user.sign_in_count = 2;
    println!("sign_in_count: {}", user.sign_in_count);
}

// [case-3] 给结构体定义方法
// 定义结构体
// 这里是#[derive(Debug)]是衍生trait为了使结构体能够println友好打印
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 通过在impl语句块中定义结构体方法
impl Rectangle {
    /*
        计算矩形面积
     */
    // 这里的&self表示结构体Rectangle具体实例的引用
    fn area(&self) -> u32 {
        // (*self).width * (*self).height
        self.width * self.height
    }

    /*
        判断是否能够容纳其它矩形
     */
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn struct_method() {
    let rect = Rectangle{
        width: 3,
        height: 5,
    };
    let rect_area = rect.area();
    // let rect_area = (&rect).area();
    // 这里格式化打印也可以为:#? 表示更加漂亮的打印输出
    println!("Rectangle struct: {:?}", rect);
    println!("Rectangle area: {}", rect_area);

    let rect2: Rectangle = Rectangle{
        width: 2,
        height: 1,
    };
    let rect3: Rectangle = Rectangle{
        width: 15,
        height: 1,
    };
    let can_hold_rect2 = rect.can_hold(&rect2);
    let can_hold_rect3 = rect.can_hold(&rect3);
    println!("Rectangle rect2 {:?} can be hold: {}", rect2, can_hold_rect2);
    println!("Rectangle rect3 {:?} can be hold: {}", rect3, can_hold_rect3);
}

// [case-4] 给结构体定义关联函数
// 注意：
//      <1> 关联函数不和某个具体的结构体实例相关，有点像Java中的static静态方法
//      <2> 一个结构体可以定义多个impl实现快
//      <3> 关联函数没有self参数，因为它和个具体的结构体实例无关
//      <4> 关联函数调用直接通过`结构体::函数名`即可
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn struct_inline_func() {
    let square = Rectangle::square(5);
    println!("square {:?}", square);
    println!("square area {}", square.area());
}
 