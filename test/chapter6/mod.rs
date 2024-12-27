// 第六章 枚举和模式匹配

fn test_enum()
{
    println!("\nTest enum");

    enum IpAddr
    {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home: IpAddr = IpAddr::V4(127, 0, 0, 1);
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

    let home: String = match home
    {
        IpAddr::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
        IpAddr::V6(s) => s,
    };

    let loopback: String = match loopback
    {
        IpAddr::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
        IpAddr::V6(s) => s,
    };

    println!("{home}");
    println!("{loopback}");
}

enum Message
{
    Quit,
    Move
    {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message
{
    fn call(&self)
    {
        if let Message::Quit = self
        {
            println!("Quit");
        }
        else if let Message::Move { x, y } = self
        {
            println!("Move to ({x}, {y})");
        }
        else if let Message::Write(text) = self
        {
            println!("Write '{text}'");
        }
        else if let Message::ChangeColor(r, g, b) = self
        {
            println!("Change color to ({r}, {g}, {b})");
        }
    }
}

fn test_message()
{
    println!("\nTest message");

    let m: Message = Message::Write(String::from("hello"));
    m.call();

    let m: Message = Message::Move { x: 1, y: 2 };
    m.call();

    let m: Message = Message::ChangeColor(255, 0, 0);
    m.call();

    let m: Message = Message::Quit;
    m.call();
}

// Option 枚举
fn test_option()
{
    println!("\nTest option");

    let some_number: Option<i32> = Some(5);
    let some_string: Option<String> = Some(String::from("a string"));
    let absent_number: Option<i32> = None;

    let some_number: i32 = match some_number
    {
        Some(i) => i,
        None => -1,
    };

    let some_string: String = match some_string
    {
        Some(s) => s,
        None => String::from("default"),
    };

    let absent_number: i32 = match absent_number
    {
        Some(i) => i,
        None => -1,
    };

    println!("{some_number}");
    println!("{some_string}");
    println!("{absent_number}");
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState
{
    _Alabama,
    Alaska,
    // --snip--+
}
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8
{
    return match coin
    {
        Coin::Penny =>
        {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel =>
        {
            println!("Nickel");
            5
        }
        Coin::Dime =>
        {
            println!("Dime");
            10
        }
        Coin::Quarter(state) =>
        {
            println!("State quarter from {state:?}!");
            25
        }
    };
}

fn test_coin()
{
    println!("\nTest coin");

    let coin: Coin = Coin::Penny;
    let value: u8 = value_in_cents(coin);
    println!("{value}");

    let coin: Coin = Coin::Nickel;
    let value: u8 = value_in_cents(coin);
    println!("{value}");

    let coin: Coin = Coin::Dime;
    let value: u8 = value_in_cents(coin);
    println!("{value}");

    let coin: Coin = Coin::Quarter(UsState::Alaska);
    let value: u8 = value_in_cents(coin);
    println!("{value}");
}

// Option 枚举
fn test_option2()
{
    fn plus_one(x: Option<i32>) -> Option<i32>
    {
        return match x
        {
            Some(i) => Some(i + 1),
            other => other, // 不执行任何操作
        };
    }

    fn print_option(x: Option<i32>)
    {
        let value: i32 = match x
        {
            None => -1,
            Some(i) => i,
        };

        if value == -1
        {
            println!("None");
        }
        else
        {
            println!("{value}");
        }
    }

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    print_option(five);
    print_option(six);
    print_option(none);

    let dice_roll = 9;
    match dice_roll
    {
        3 => println!("You rolled a 3!"),
        7 => println!("You rolled a 7!"),
        _ => (), // 不执行任何操作
    }

    // 使用 if let 简化代码
    let config_max: Option<u8> = Some(3);
    if let Some(max) = config_max
    {
        println!("The maximum is configured to be {max}");
    }
    else
    {
        println!("No maximum configured");
    }
}

pub fn chapter6()
{
    println!("\n=== Chapter 6 ===");

    test_enum();
    test_message();
    test_option();
    test_coin();
    test_option2();
}
