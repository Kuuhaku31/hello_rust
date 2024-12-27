// 第四章 所有权（系统）

// 所有权以及相关功能：借用（borrowing）、slice 以及 Rust 如何在内存中布局数据

fn test_string()
{
    fn calculate_length(s: String) -> (String, usize)
    {
        let length = s.len(); // len() 返回字符串的长度

        (s, length)
    }

    println!("\nTest string");

    let mut s1: String = String::from("hello");
    s1.push_str(", world!");
    println!("The value of s is: {s1}");

    let s2: String = s1;
    println!("The value of s2 is: {s2}");

    let s3: String = String::from("hello");
    let s4: String = s3.clone();

    println!("{s4}, world!");

    let s_info: (String, usize) = calculate_length(s4);
    println!("The length of '{}' is {}", s_info.0, s_info.1);
}

fn takes_ownership(some_string: String)
{
    // some_string 进入作用域
    println!("{some_string}");
}
// 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32)
{
    // some_integer 进入作用域
    println!("{some_integer}");
}
// 这里，some_integer 移出作用域。没有特殊之处

fn ownership()
{
    // s 进入作用域
    let s: String = String::from("hello");

    // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    takes_ownership(s);

    // x 进入作用域
    let x: i32 = 5;

    // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    // 所以在后面可继续使用 x
    makes_copy(x);
}
// 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 没有特殊之处

//
// 引用与借用
fn reference()
{
    {
        //（默认）不允许修改引用的值
        fn calculate_length(s: &String) -> usize { s.len() }
        fn change_string(s: &mut String) { s.push_str(", world!") }

        let s1: String = String::from("hello");
        let len: usize = calculate_length(&s1);
        println!("The length of '{}' is {}", s1, len);

        let mut s2: String = String::from("hello");
        change_string(&mut s2);
        println!("The value of s2 is: {s2}");
    }

    // 如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用
    // {
    //     let r1: &String = &mut s2;
    //     let r2: &String = &mut s2;
    //     println!("{}, {}", r1, r2);
    // }

    {
        let mut s: String = String::from("hello");

        {
            let r1: &mut String = &mut s;
            println!("{r1}");
        }
        // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

        {
            let r2: &mut String = &mut s;
            println!("{r2}");
        }
    }

    // 多个不可变引用是可以的
    {
        let mut s: String = String::from("hello");
        println!("{s}");

        let r1: &String = &s;
        let r2: &String = &s;
        // let r3: &mut String = &mut s; 也不允许同时存在可变引用
        println!("{r1}, {r2}");
        // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止

        s.push_str(", world!");
        println!("{s}");
    }
}

// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。

//
// slice
// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权

// 一个字符串 slice 的类型声明写作 &str
fn first_word(s: &str) -> &str
{
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice()
{
    {
        let string: String = String::from("hello world");
        let first_word: &str = first_word(&string);
        println!("The first word of '{}' is '{}'.", string, first_word);
    }

    {
        let s: &str = "hello world"; // 字符串字面值就是 slice
        println!("{s}");

        let s: &str = first_word(&s[1..4]);
        println!("{s}");
    }
}

pub fn chapter4()
{
    println!("\n=== Chapter 4 ===");
    test_string();
    ownership();
    reference();
    slice();
}
