use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // guess_name();

    // let s1 = String::from("height");
    // let _len = first_world(&s1);
    // // println!("长度是 {}", _len);
    // // s1.clear();  这里会报错，有一个不可变引用
    // println!("第一个单词是 {}", _len);

    // get_area();   // 值传递，两个参数的形式
    // get_area1();
    get_area2();
}


/**  我们会实现一个经典的新手编程问题：猜猜看游戏。
    它是这么工作的：程序将会随机生成一个 1 到 100 之间的随机整数。
    接着它会请玩家猜一个数并输入，然后提示猜测是大了还是小了。
    如果猜对了，它会打印祝贺信息并退出
*/
fn guess_name () {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret_number is {}", secret_number);

    loop {
        println!("Please input your guess");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number");  //这样会导致程序崩溃，退出程序。下边的方式可以继续输入
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// fn first_world (s: &String) -> usize {
fn first_world (s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item1) in bytes.iter().enumerate() {
        println!("循环出来的内容 {} {} {}", item1, b' ', b'h');
        if item1 == b' ' {
            // return i
            return &s[0..i];
        }
    }

    // s.len()
    &s[..]
}

/*
    获取以像素为单位的长方形的宽度和高度，并计算出长方形的面积。
*/
fn get_area() {
    let width1 = 30;
    let height1 = 50;
    
    println!("The area of the rectangle is {} squire pixels. -- 普通的方式，传参不是引用",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

/* 求面积，以元组的方式 */
fn get_area1() {
    let rect1 = (30, 50);

    println!(
        "The area of this rectangle if {} squire pixels. -- 元组的方式, 传参不是引用",
        area1(rect1)
    );
}

fn area1(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

/* 求面积，以结构体的方式 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn get_area2() {
    let scale = 2;
    let rect1 = Rectangle { width: dbg!(32 * scale), height: dbg!(50 * scale) };

    // 如何打印结构体
    // println!("rect 1 is {}", rect1); //不行； {} 默认告诉 println! 使用被称为 Display 的格式：意在提供给直接终端用户查看的输出。
    // println!("rect1 is {:?}", rect1); //不行； 解决方案： 给Rectangle声明的地方加一个 #[derive(Debug)]
    // println!("rect1 is {:?}", rect1); //可以，但是格式不太完整
    // println!("rect1 is {:#?}", rect1); //可以，格式非常完整
    dbg!(&rect1);  //可以打印出行数来，一个里边不能同时有println!和dbg!

    println!(
        "The area of the rectangle is {} squire pixels - 结构的方式，传参是引用",
        area2(&rect1) //我们希望借用结构体而不是获取它的所有权，这样 main 函数就可以保持 rect1 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 &
    );
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}