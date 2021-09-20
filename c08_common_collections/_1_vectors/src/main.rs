#![allow(unused)]
fn main() {
    let v: Vec<i32> = Vec::new();

    // 利用 vec 宏操作
    let v = vec![1, 2, 3];

    // 可变变量才能添加数据
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 读取 vector 里面的元素
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // 这种方式索引要不存在会 panic
    println!("The third element is {}", third);

    match v.get(2) { // 这种方式返回的是 Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 产生了不可变引用
    // v.push(6); // 这里就不能够对对象取出可变引用
    println!("The first element is: {}", first);

    // 遍历元素
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // 遍历修改元素
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * 解引用找到对应的地址
    }

    // 利用枚举在 vector 中存储不同的值
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
