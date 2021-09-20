#![allow(unused)]
use _2_trait::aggregator::*;
use std::fmt::Display;
use std::fmt::Debug;
use std::cmp::PartialOrd;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

pub fn notify(item: impl Summary) { // 传入特性
    println!("Breaking news! {}", item.summarize());
}
pub fn notify2<T: Summary>(item: T) { // 采用泛型的方式i
    println!("Breaking news! {}", item.summarize());
}
pub fn notify3(item: &(impl Summary + Display)) {} // + 表示组合特性
pub fn notify4<T: Summary + Display>(item: &T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    0
}
fn some_function2<T, U>(t: T, u: U) -> i32
    where T: Display + Clone, // where 语句在后面约束泛型
          U: Clone + Debug
{
    0
}


fn returns_summarizable() -> impl Summary { // 使用特性当做返回值
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
// 这里无法编译，我认为，因为 rust 会编译时确定每个变量内存大小，返回不同的对象，这个函数的返回值大小就不确定
// fn returns_summarizable2(switch: bool) -> impl Summary { 
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// 首先 传入的是借用
// 1. 取出第一个值，需要 copy 特性才能这样取出，和原数据无关了已经
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // 实现了 Copy 特性的类型才可以不通过引用来实现赋值

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 完全不明白写的啥，这也太复杂了
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0]; // 实现了 Copy 特性的类型才可以不通过引用来实现赋值

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}
