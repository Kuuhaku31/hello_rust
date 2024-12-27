// minigrep.rs

use std::env; // 用于获取命令行参数
use std::error::Error;
use std::fs;
use std::process;

struct Config
{
    query: String,
    file_path: String,
}

impl Config
{
    // 解析命令行参数
    fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3
        {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        return Ok(Config { query, file_path });
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> // 返回 Result 类型
{
    let contents: String = fs::read_to_string(config.file_path)?; // ? 用于处理 Result 类型

    println!("With text:\n{contents}");

    return Ok(());
}

pub fn minigrep()
{
    let args: Vec<String> = env::args().collect(); // 获取命令行参数

    // 从命令行参数中解析查询字符串和文件路径
    let config: Config = Config::build(&args) // 返回 Result 类型
        .unwrap_or_else(|err: &str| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    // 如果解析失败则打印错误信息并退出

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config)
    {
        println!("Application error: {e}");
        process::exit(1);
    }
}
