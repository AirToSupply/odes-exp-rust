// 需求：假设需要创建一个图形用户接口（GUI）工具，通过遍历列表并调用每一个项目的draw方法来将其绘制到屏幕上。
//      比如：Button或者TextField。在此之上，用户希望创建自定义的可以绘制于屏幕上的类型：比如，能会增加Image，另一个可能会增加SelectBox。

// 定义一个带有 draw 方法的 特征 Draw
pub trait Draw {
    fn draw(&self);
}

//  一个 Screen 结构体的定义，它带有一个字段 components，用于存放所有组件列表
pub struct Screen {
    // 注意：列表中的每个元素都是通过智能指针Box包装
    //      并且每个具体的元素都必须实现特征Draw
    //      这里的dyn(dynamic)关键字代表动态派遣，Rust会在运行时决定具体调用哪个具体类型的方法
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // 该方法会对其 components 上的每一个组件调用 draw 方法将所有的组件绘制到屏幕上
    pub fn run(&self) {
        for c in self.components.iter() { // c: &Box<dyn Draw>
            c.draw();
        }
    }
}

// ❓ 对于Screen也可以通过泛型，为什么这里不通过泛型实现？
//    对于泛型实现只能存放一种类型，这种被称之为【静态派遣】，
//        例如：实现了Draw特征的对象有Text和Button，那么Screen中的components列表要么只能存放Text，要么只能存放Button。
//    对于动态派遣实现能够存放多种类型，这种被称之为【动态派遣】，
//        例如：实现了Draw特征的对象有Text和Button，那么Screen中的components列表中可以同时混合存放Text和Button。
/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> 
where T: Draw
{
    pub fn run(&self) {
        for c in self.components.iter() { // c: &T
            c.draw();
        }
    }
}
*/

// 定义按钮组件
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw Button[width = {}, height = {}, label = {}]", self.width, self.height, self.label);
    }
}