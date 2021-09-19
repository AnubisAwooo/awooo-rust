fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{} {} {} {}", user.email, user.username, user.active, user.sign_in_count);
    // user.username.push_str("string: &str"); // 不可变变量不能借用为可变引用

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");

    let user_1 = build_user(String::from("email: String"), String::from("username: String"));
    let user_2 = build_user2(String::from("email: String"), String::from("username: String"));
    println!("{}", user_1.email);
    println!("{}", user_2.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    let user2_2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 解构另一个对象
    };
    println!("{}", user2.email);
    println!("{}", user2_2.email);

    // 通过元组方式构造结构体，是结构体，不是元组了已经  因为前面加了 struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let mut origin = Point(0, 0, 0);

    let r = black.1;
    origin.1 = r;

    // 元组解构
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {} {} {}", x, y, z);

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email, // 重复输入
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        email, // 同名省略, 有对应的变量才可以这里，但是这种方式应该很常见
        username,
        active: true,
        sign_in_count: 1,
    }
}
