#![allow(unused)]
fn main() {
    let mut s = String::new();


    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l'); // push 单个字符

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // + 运算符 相当于 String 有一个 fn add(self, s: &str) -> String 的方法
    // &String 也可以当做 &str 使用，因为 解引用强制多态 &s2 相当于 &s2[..]


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 连续 + 很笨重
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 可以使用 format 宏操作连接字符串
    let s = format!("{}-{}-{}", s1, s2, s3);    


    let s1 = String::from("hello");
    // let h = s1[0];// 无效 String 不能简单通过索引访问

    // 首先 String 底层是 u8 的链表 对于不同字符的长度不一样，直接索引取有问题
    // 另一个原因，链表通过索引访问遍历性能消耗太大
    let hello = String::from("Hola");    
    let len = String::from("Здравствуйте").len();

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 取出前 2 个字符 左边必须是字符起始，右边必须是字符边界

    for c in "नमस्ते".chars() {
        println!("{}", c); // 当做字符集遍历
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b); // 当做 u8 集遍历
    }

}
