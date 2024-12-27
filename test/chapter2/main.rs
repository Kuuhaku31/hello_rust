//main.rs

// 将 io 输入/输出库引入当前作用域
use rand::Rng;
use std::cmp::Ordering; // 为了比较用户输入的数字和随机数
use std::io;

fn main()
{
    println!("Guess the number!");

    // 生成一个随机数
    /*
    首先，我们新增了一行 use rand::Rng;。Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中。

    rand::thread_rng 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。
    接着调用随机数生成器的 gen_range 方法。
    这个方法由 use rand::Rng 语句引入到作用域的 Rng trait 定义。
    gen_range 方法获取一个范围表达式（range expression）作为参数，并生成一个在此范围之间的随机数。
    这里使用的这类范围表达式使用了 start..=end 这样的形式，也就是说包含了上下端点，所以需要指定 1..=100 来请求一个 1 和 100 之间的数。
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop
    {
        println!("Please input your guess.");

        /*
        变量默认是不可变的，这意味着一旦我们给变量赋值，这个值就不再可以修改了
        为了让变量可变，我们需要在变量名之前加上 mut 关键字
        String 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块
        :: 语法表明 new 是 String 类型的一个 关联函数（associated function）

        这一行创建了一个可变变量，当前它绑定到一个新的 String 空实例上。
        */
        let mut guess = String::new();

        // 获取用户输入并将其存入 guess 变量中
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>
            {
                println!("You win!");
                break;
            }
        }
    }
}
