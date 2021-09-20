#![allow(unused)]
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
use std::io;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("{:?}", f);

    let f = File::open("hello.txt").unwrap_or_else(|error| { // 这应该是 lambda 表达式
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);


    let f = File::open("hello.txt").unwrap();
    println!("{:?}", f);

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("{:?}", f);

}


// 错误抛出
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? 运算符可被用于返回值类型为 Result 的函数
// ？存在的位置必须是外部函数是 Result 作为返回值的 是遇到错误跳出本方法
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // 简化抛出错误
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 简化抛出错误
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // 链式调用简化抛出错误

    Ok(s)
}
