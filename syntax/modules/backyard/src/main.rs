// 从当前package下的root crate导入子模块vegetables中的Asparagus结构体
use crate::garden::vegetables::Asparagus;

// 声明并定义父模块garden，同时需要有一个同模块名相同的文件即garden.rs文件来管理garden模块下的各种类型
pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("I'm growing {:?}!", plant);
}
