// -------------------------------------------------------
// [case-1] 结构体的访问修饰符
// -------------------------------------------------------
mod back_of_house {
    // <1> pub修饰结构体对外暴露
    pub struct Breakfast {
        // <2> pub修饰结构体属性对外暴露
        pub toast: String,
        // <4> 没有pub修饰结构体属性不对外暴露
        seasonal_fruit: String,
    }

    impl Breakfast {
        // <3> pub修饰结构体关联函数对外暴露
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // [ERROR] field `seasonal_fruit` of `Breakfast` is privatess
    // meal.seasonal_fruit = String::from("blueberries");
}


// -------------------------------------------------------
// [case-2] 枚举的访问修饰符
// 注意：
//      将枚举设为公有，则它的所有成员都将变为公有
// -------------------------------------------------------
mod back_of_house_for_appetizer {
    // pub修饰枚举对外暴露
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_for_appetizer() {
    let order1 = back_of_house_for_appetizer::Appetizer::Soup;
    let order2 = back_of_house_for_appetizer::Appetizer::Salad;
}