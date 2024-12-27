// 第五章 使用结构体组织相关联的数据

// 定义和实例化结构体

// 用户结构体
struct User
{
    username: String,   // 用户名
    email: String,      // 邮箱
    sign_in_count: u64, // 登录次数
    active: bool,       // 是否激活
}

fn create_user()
{
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!(
        "User 1:\n\tusername: {}\n\temail: {}\n\tsign_in_count: {}\n\tactive: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    user1.email = String::from("xxx");

    println!(
        "User 1:\n\tusername: {}\n\temail: {}\n\tsign_in_count: {}\n\tactive: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32);

// struct AlwaysEqual;

fn test_struct()
{
    let black = Color(0, 0, 0);
    let origin = Point(0, 0);

    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {})", origin.0, origin.1);

    // let subject: AlwaysEqual = AlwaysEqual;
}

#[derive(Debug)] // 为了使用 println! 打印结构体
struct Rectangle
{
    width: u32,
    height: u32,
}

// 定义结构体方法
impl Rectangle
{
    fn get_area(&self) -> u32 { self.width * self.height }
    fn set_rect(&mut self, width: u32, height: u32)
    {
        self.width = width;
        self.height = height;
    }
}

impl Rectangle
{
    // 关联函数
    fn square(size: u32) -> Self
    {
        Self {
            width: size,
            height: size,
        }
    }
}

fn rectangles()
{
    println!("\nTest rectangles");

    {
        fn area(rectangle: &Rectangle) -> u32 { rectangle.width * rectangle.height }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        println!("rect1 is {rect1:#?}");
    }

    {
        let scale: u32 = 2;
        let rect1: Rectangle = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }

    {
        let rect1: Rectangle = Rectangle {
            width: 30,
            height: 40,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.get_area()
        );

        let mut rect2: Rectangle = Rectangle {
            width: 10,
            height: 20,
        };

        rect2.set_rect(20, 30);
        println!(
            "The area of the rectangle is {} square pixels.",
            rect2.get_area()
        );

        let rect3: Rectangle = Rectangle::square(3);
        println!("rect3 is {rect3:#?}");
    }
}

pub fn chapter5()
{
    println!("\n=== Chapter 5 ===");
    create_user();
    test_struct();
    rectangles();
}
