// errors.rs

// 第九章 错误处理

const FILE_PATH: &str = "misc/hello.txt";

fn read_file()
{
    use std::fs::File;
    use std::io::Error;
    use std::io::ErrorKind;

    // 尝试打开文件
    let greeting_file_result: Result<File, Error> = File::open(FILE_PATH);

    // 使用 match 处理 Result
    let greeting_file: File = match greeting_file_result
    {
        // 成功打开文件
        Ok(file) =>
        {
            println!("File opened: {file:?}");
            file
        }

        // 未找到文件
        Err(error) => match error.kind()
        {
            // 创建文件
            ErrorKind::NotFound => match File::create(FILE_PATH)
            {
                Ok(fc) =>
                {
                    println!("File created: {fc:?}");
                    fc
                }
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            // 其他错误
            other_error => panic!("Problem opening the file: {other_error:?}"),
        },
    };

    println!("{greeting_file:?}");
}

fn read_file2()
{
    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file: File = File::open(FILE_PATH).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound
        {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        }
        else
        {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("{greeting_file:?}");
}

pub fn errors()
{
    println!("\n=== Chapter 9 ===");

    read_file();
    read_file2();
}
