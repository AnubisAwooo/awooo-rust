// 所有权规则
// - 每一个 Rust 值都有一个成为所有者的变量
// - 同一时间所有者只能有一个
// - 当变量的所有者失去作用域，这个变量就会被丢弃

// 变量的作用域

// 引用和借用
// & 符号会创建一个指向堆的引用，该引用不具有所有者，因此也不会因失去作用域而被清除其对应堆上的变量
// 获取引用 作为 函数参数 称为 借用
// 不可变引用可以多个
// 可变引用只有一个
// 不可变引用和可变引用冲突  读写锁？

// 切片也是没有所有者的数据类型

fn main() {
    // 栈上的直接复制，是 2 个独立的变量
    // 栈上根本放不下复杂变量，都是基本已知字节大小的数据，直接复制一份性能消耗和指针复制一样的
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    // 实际数据在堆上，又不想深拷贝耗费性能
    // 采用转移的方式，令前一个变量失效
    // 这样一个变量一份数据始终只有一份所有者，当离开作用范围时，只清理一次内存即可
    // 如果同一段内存有多个所有者，可能会导致重复清理，显然不能接受
    let s1 = String::from("hello");
    let s2 = s1; // s1 对象的所有权被转移   s1 就此失效
    println!("{}, world!", s2);

    main2();
    main3();


    test_string();

}

// 函数调用转移所有者
fn main2() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// 函数返回值转移所有者
fn main3() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn test_string() {
    let mut s = String::from("hello world");
    s.push_str("1231");

    // let s1 = String::from("hello world");
    // s1.push_str("1231"); // 不可变变量是不能够使用可变变量的方法的

    let word = first_word(&s);

    // 这里出错的原因是，前面已经把 s 当做不可变变量创建了索引，就是有读锁了，并且这个读锁通过切片的方式仍然保留着 word
    // 如果再要进行可变的操作，写锁是不能获取到的
    // 判断可变不可变，通过方法第一个参数的 mut 标识，需要可变引用
    // s.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
