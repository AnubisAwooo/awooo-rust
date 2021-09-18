
// 引入随机包
use rand::Rng;
// 引入 io 包
use std::io;
// 引入比较包
use std::cmp::Ordering;

fn main() {
    // 输出标题
    println!("Guess number");

    // 用随机包生成一个随机数生成器
    // 用随机数生成器生成一个随机数
    let secret = rand::thread_rng().gen_range(1..=100);

    // 输出秘密数字
    println!("the secret number is {}.", secret);

    loop {
        // 输出提示输入数字
        println!("please input your guess number:");

        // 存入输入的数字变量 可变类型
        let mut guess = String::new();
        
        // io 输入流
        // 读取一行输入，并赋值到 guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        // 对输入的字符进行 trim
        // 对 trim 后的结果进行转换，对结果进行匹配
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue
            },
        };

        // 比较数字
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Gotcha!");
                break
            },
        }

    }

}
