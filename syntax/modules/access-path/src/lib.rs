// ---------------------------------------------------------------
// [case-1] 路径访问
// 绝对路径 vs. 相对路径
// ---------------------------------------------------------------
// 父mod
mod front_of_house {
    // 子mod
    // 注意；
    //      <1> pub表示该子模块可以对外暴露，相当于public
    //      <2> 如果没有pub修饰，默认则表示私有的，相当于private
    pub mod hosting {
        // pub表示该函数可以对外暴露，相当于public
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 通过绝对路径访问
    // 这里的crate为crate root
    crate::front_of_house::hosting::add_to_waitlist();

    // 通过相对路径访问(从当前模块开始)
    // 这里的front_of_house为当前文件所在crate
    front_of_house::hosting::add_to_waitlist();
}

// ---------------------------------------------------------------
// [case-2] 路径访问
// 不通过路径，通过super关键字的相对路径
// ---------------------------------------------------------------
mod back_of_house {
    fn fix_incorrect_order() {

        // 访问当前模块下的函数
        cook_order();

        // 访问当前模块以外的函数
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}