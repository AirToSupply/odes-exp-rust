// 通过mod关键字定义一个模块
mod front_of_house {
    // 在模块中可以定义一个子模块
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
