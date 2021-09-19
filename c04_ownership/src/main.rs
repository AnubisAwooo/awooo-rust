// 所有权规则
// - 每一个 Rust 值都有一个成为所有者的变量
// - 同一时间所有者只能有一个
// - 当变量的所有者失去作用域，这个变量就会被丢弃

// 变量的作用域



fn main() {
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);


    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
