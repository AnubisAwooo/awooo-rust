#![allow(unused)]

use std::thread;
use std::time::Duration;

fn main1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap(); // 主线程中调用 就是 等待子线程 handle 执行结束
}

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // move 闭包将变量 v 所有权移到线程内部，外部不再拥有 v 的所有权
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);  // 外部不再拥有 v 的所有权

    handle.join().unwrap();
}