// lib.rs

// 第七章 餐厅
mod front_of_house;

// 第八章 集合
pub mod collections;

// 第九章 错误处理
pub mod errors;

// 第十二章 I/O 项目：构建一个命令行程序
pub mod minigrep;

// 第十三章 函数式语言特性：迭代器和闭包
pub mod closures;

mod back_of_house
{
    fn _fix_incorrect_order()
    {
        _cook_order();
        super::_deliver_order(); // 使用 super 调用父模块
    }

    fn _cook_order() {}

    pub struct Breakfast
    {
        pub toast: String,       // 公有字段
        _seasonal_fruit: String, // 私有字段
    }

    impl Breakfast
    {
        pub fn summer(toast: &str) -> Breakfast
        {
            return Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            };
        }
    }
}

pub use crate::front_of_house::hosting as RestaurantHosting;

pub fn eat_at_restaurant()
{
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    RestaurantHosting::add_to_waitlist();
}

fn _deliver_order() { RestaurantHosting::add_to_waitlist(); }
