#![allow(unused)]
// 最简单直接的智能指针是 box，其类型是 Box<T>
// - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {:#?}", b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));


    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 强制解引用
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 实际上需要的是 &str  这里原本应该写成 &(*m)[..] Rust强制解引用
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
