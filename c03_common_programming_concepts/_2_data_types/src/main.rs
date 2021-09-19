fn main() {

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess: {}", guess);


    let x = 57u8;


    // 数据类型
    // - 标量类型
    //   1. 整型数  u8 u16 u32 u64 u128 i8 i16 i32 i64 i128  isize usize size是根据架构
    //      u8 类型可以用 b'A' 表示
    //      57u8 指定类型
    //      debug模式编译器会检测溢出，release模式不会，方法前缀 wrapping_ checked_ overflowing_ saturating_ 分别 
    //   2. 浮点数 f32 f64 IEEE-754
    //   3. 布尔 bool -> true false
    //   4. 字符 char 4字节
    // - 复合类型
    //   1. 元组 不同类型组合
    //   2. 数组 相同类型组合 索引越界会退出，很多底层语言会允许访问

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
