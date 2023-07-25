// Rust面向对象-多态
// Rust中实现多态是通过比较特别方式，即：特征对象（trait object）
use trait_object::*;

// 自定义组件-下拉选择框
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw Button[width = {}, height = {}, options = {:?}]", self.width, self.height, self.options);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run(); 
}