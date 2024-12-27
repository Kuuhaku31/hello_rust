// lib.rs

// 第七章 餐厅
mod front_of_house;

mod back_of_house
{
    fn fix_incorrect_order()
    {
        cook_order();
        super::deliver_order(); // 使用 super 调用父模块
    }

    fn cook_order() {}

    pub struct Breakfast
    {
        pub toast: String,      // 公有字段
        seasonal_fruit: String, // 私有字段
    }

    impl Breakfast
    {
        pub fn summer(toast: &str) -> Breakfast
        {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
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

fn deliver_order() { RestaurantHosting::add_to_waitlist(); }
