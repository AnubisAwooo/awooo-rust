fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; // immutable variable can not assign again
    println!("The value of x is: {}", x);

    let mut xx = 5;
    println!("The value of xx is: {}", xx);
    xx = 6;
    println!("The value of xx is: {}", xx);

    // 常量必须指定类型 大写
    // 常量结果必须唯一，也就不能用运行时才能确定的表达式
    // 常量必须编译时能确定，和运行时无关
    const XXX: u32 = 6; 
    println!("The value of xx is: {}", XXX);


    // 隐藏变量
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

}
