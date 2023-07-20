mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 通过use关键字引入路径
use crate::front_of_house::hosting;
// 注意：
//       这里也可以通过self为根进行引入
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    /*
       多次引入函数
     */
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    /*
        通过use关键字引入路径来避免重复写路径
     */
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
