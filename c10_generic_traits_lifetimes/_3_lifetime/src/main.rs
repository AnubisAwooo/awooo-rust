#![allow(unused)]
fn main() {
    {
        let r = 8;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            // r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+


    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+



    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 不同生命周期示例
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    let result = "123";
    {
        let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str()); // 生命周期不一致 结果值的生命周期必须是最小范围那个，这里不能在外面
    }
    println!("The longest string is {}", result);

}

// &i32       引用
//&'a i32     带有显式生命周期的引用
//&'a mut i32 带有显式生命周期的可变引用

// 就像函数签名一样，生命周期注解本身不影响任何变量的生命周期，仅是标识调用该函数的要求
// 'a 表明传入参数声生命周期的最小交集
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y 
    }
}

// 和第 2 个参数的生命周期无关了
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x // 结果可以判断与第 2 个参数无关
}

fn longest3<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("really long string");
    // result.as_str() // 会造成悬垂引用，原对象会在方法结束时清除，怎么能够吧原对象的引用传递出去呢？
    x
}

fn longest4<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result // 直接把所有权一起给出去，不要给引用
}

// 生命周期省略 
// 1. 每一个是引用的参数都有它自己的生命周期参数
// 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
// 3. 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self，那么 self 的生命周期被赋给所有输出生命周期参数

// 静态生命周期
// 这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间
// let s: &'static str = "I have a static lifetime.";

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
