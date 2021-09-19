fn main() {
    // 条件判断

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }



    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // if 语句可以当成表达式，只要每个分支返回的结果类型一样即可
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);



    // 循环

    loop {
        println!("again!");
        break; // 不要死循环
    }


    // 循环也可以当成表达式
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break 语句真的秀，还带返回值的，不过如果不这样，那也不能写 return 返回值啊，只能这样了
        }
    };

    println!("The result is {}", result);

    // while 语句
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for in 语句
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}
