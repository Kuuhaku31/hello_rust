// closures

#[derive(Debug, PartialEq, Copy, Clone)]
// 颜色枚举
enum ShirtColor
{
    Red,
    Blue,
}

// 库存结构体
struct Inventory
{
    shirts: Vec<ShirtColor>,
}

impl Inventory
{
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor
    {
        // 使用闭包
        return user_preference.unwrap_or_else(|| self.most_stocked());
        /*
        Option<T> 上的 unwrap_or_else 方法 由标准库定义。
        它接受一个无参闭包作为参数，该闭包返回一个 T 类型的值（与 Option<T> 的 Some 变体中存储的值类型相同，这里是 ShirtColor）。
        如果 Option<T> 是 Some 成员，则 unwrap_or_else 返回 Some 中的值。
        如果 Option<T> 是 None 成员，则 unwrap_or_else 调用闭包并返回闭包的返回值。

        我们将闭包表达式 || self.most_stocked() 作为 unwrap_or_else 的参数。这是一个本身不获取参数的闭包
        （如果闭包有参数，它们会出现在两道竖杠之间）。
        闭包体调用了 self.most_stocked()。我们在这里定义了闭包，而 unwrap_or_else 的实现会在之后需要其结果的时候执行闭包。
        */
    }

    fn most_stocked(&self) -> ShirtColor
    {
        let mut num_red: u32 = 0;
        let mut num_blue: u32 = 0;

        for color in &self.shirts
        {
            match color
            {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        return if num_red > num_blue
        {
            ShirtColor::Red
        }
        else
        {
            ShirtColor::Blue
        };
    }
}

pub fn test_closures()
{
    let store: Inventory = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);
    let giveaway1: ShirtColor = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2: Option<ShirtColor> = None;
    let giveaway2: ShirtColor = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
