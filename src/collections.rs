// collections.rs

// 第八章 集合

// vector
fn vectors()
{
    println!("\nTest Vectors");

    fn print_vector(v: &Vec<i32>)
    {
        println!("There are {} elements in the vector: ", v.len());
        for i in v
        {
            println!("{}", i);
        }
    }

    {
        // 创建一个新的空vector
        let _v: Vec<i32> = Vec::new();

        // 使用宏创建一个包含元素的vector
        let _v: Vec<i32> = vec![1, 2, 3];
    }

    {
        // 更新vector
        let mut v: Vec<i32> = Vec::new();
        print_vector(&v);
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        print_vector(&v);
    }
    // 丢弃vector时也会丢弃其所有元素

    {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];

        // 使用 & 和 [] 访问元素
        let third: &i32 = &v[2];
        println!("The third element is {third}");

        // get 方法返回一个 Option<&T>
        let third: Option<&i32> = v.get(2);
        match third
        {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }

    {
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
        // let first: &i32 = &v[0];
        v.push(6);
        print_vector(&v);
    }

    {
        let mut v: Vec<i32> = vec![100, 32, 57];
        for i in &mut v
        {
            *i += 50;
        }
        print_vector(&v);
    }

    {
        enum SpreadsheetCell
        {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row: Vec<SpreadsheetCell> = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in &row
        {
            match i
            {
                SpreadsheetCell::Int(val) => println!("Int: {val}"),
                SpreadsheetCell::Float(val) => println!("Float: {val}"),
                SpreadsheetCell::Text(val) => println!("Text: {val}"),
            }
        }
    }
}

fn string()
{
    println!("\nTest Strings");

    {
        let data: &str = "initial contents";

        let s: String = data.to_string();
        println!("{s}");

        // 该方法也可直接用于字符串字面值：
        let s: String = "initial contents".to_string();
        println!("{s}");

        let s: String = String::from(&data[..]);
        println!("{s}");
    }

    {
        let chinese: String = String::from("这是一段测试中文……");
        println!("{chinese}");
        for c in chinese.chars()
        {
            print!("{c} ");
        }
        println!();
        for b in chinese.bytes()
        {
            print!("{b} ");
        }
        println!();

        let japanese: String = String::from("これは日本語のテストです……");
        println!("{japanese}");
        for c in japanese.chars()
        {
            print!("{c} ");
        }
        println!();
        for b in japanese.bytes()
        {
            print!("{b} ");
        }
        println!();
    }
}

fn hash_map()
{
    println!("\nTest Hash Map");

    {
        use std::collections::HashMap;

        fn print_hash_map(map: &HashMap<String, i32>)
        {
            // 会以任意顺序打印出每一个键值对
            for (key, value) in map
            {
                println!("{key}: {value}");
            }
        }

        // 创建一个新的空 HashMap
        let mut scores: HashMap<String, i32> = HashMap::new();

        let green: String = String::from("Green");

        // 插入键值对
        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);
        scores.insert(green, 100);

        // 访问键值对
        let team_name: String = String::from("Blue");
        let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
        println!("{team_name} score: {score}");

        // 遍历键值对
        print_hash_map(&scores);
    }
}

pub fn chapter_8()
{
    println!("\n=== Chapter 8 ===");

    vectors();
    string();
    hash_map();
}
