#![allow(unused)]

use std::sync::mpsc; // mpsc 多生产者 单消费者 multiple producer single consumer  感觉像 java 里面的阻塞队列
use std::thread;

fn main1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // 一旦发送到通道，就失去所有权了
    });

    let received = rx.recv().unwrap(); // 阻塞直到获取到值 try_recv 不阻塞
    println!("Got: {}", received);
}

use std::time::Duration;
fn main2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // 复制了新的生产者通道
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}
