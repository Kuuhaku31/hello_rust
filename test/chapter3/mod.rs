// charpter3/mod.rs

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn variables()
{
    println!("\nTest variables");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Constant: {THREE_HOURS_IN_SECONDS}");
}

fn test_shadowing()
{
    println!("\nTest shadowing");

    let x = 500_000_000u64;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

/*
    两类数据类型子集：标量（scalar）和复合（compound）

    标量类型：整型、浮点型、布尔型、字符型
        整型：i8、i16、i32、i64、u8、u16、u32、u64、isize、usize
        浮点型：f32、f64
        布尔型：bool
        字符型：char

    复合类型：元组（tuple）、数组（array）
        元组：元组允许将多个值组合成一个复合类型
        数组：数组中的每个元素必须是相同类型
*/

fn test_tuple_and_array()
{
    println!("\nTest tuple and array");

    let tup: (i32, f64, u8, bool) = (500, 6.4, 1, true);
    let (x, y, z, w) = tup;
    println!("The value of tup is: ({x}, {y}, {z}, {w})");
    let x: i32 = tup.0;
    let y: f64 = tup.1;
    let z: u8 = tup.2;
    let w: bool = tup.3;
    println!("The value of tup is: ({x}, {y}, {z}, {w})");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for (i, &item) in a.iter().enumerate()
    {
        println!("The value of a[{i}] is: {item}");
    }
    let a: [i32; 5] = [3; 5];
    for (i, &item) in a.iter().enumerate()
    {
        println!("The value of a[{i}] is: {item}");
    }
}

fn array_error()
{
    println!("\nTest array error");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let index: usize = 4;
    let element: i32 = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn test_function()
{
    println!("\nTest function");

    // 表达式和语句
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    another_function(9);

    // 函数的返回值等同于函数体最后一个表达式的值
    fn another_function(x: i32) -> i32
    {
        println!("The value of x is: {x}");

        if x >= 5
        {
            println!("The value of x is greater than or equal to 5");
            return 5;
        }

        x + 1
    }

    let res: i32 = another_function(5);
    println!("The value of res is: {res}");
}

fn test_branches()
{
    println!("\nTest branches");

    let number: i32 = 3;

    if number < 5
    {
        println!("The condition was true");
    }
    else if number == 5
    {
        println!("The condition was 5");
    }
    else
    {
        println!("The condition was false");
    }

    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 }; // 代码块的值是其最后一个表达式的值
    println!("The value of number is: {number}");
}

fn test_loops()
{
    println!("\nTest loops");

    let mut count: i32 = 0;
    'counting_up: loop
    {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop
        {
            println!("remaining = {remaining}");
            if remaining == 9
            {
                break; // 退出内层循环
            }
            if count == 2
            {
                break 'counting_up; // 利用标签退出外层循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;
    while index < 5
    {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a
    {
        println!("the value is: {element}");
    }

    // 利用vector的rev方法反转迭代器
    for number in (1..4).rev()
    {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

pub fn chapter3()
{
    println!("\nHello from 3.rs!");
    variables();
    test_shadowing();
    test_tuple_and_array();

    array_error();
    test_function();

    test_branches();
    test_loops();
}
