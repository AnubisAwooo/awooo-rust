// 演示添加方法
// impl 块里面的方法 第一个参数
//   self 表明本方法要把实例的所有者获取，若不返回的话，那么调用这个方法的实例就没了
//   &self 表明本方法借用实例，只读取不修改，所有者不变
//   &mut self 表明本方法借用可变实例，可以修改，调用本方法的实例必须也是可变对象才行

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
    // fn rotate(self) -> Rectangle {
    //     Rectangle {
    //         width: self.height,
    //         height: self.width
    //     }
    // }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数 无需实例即可调用 不需要 self，关键点在于第一个参数是否指明类型，不写类型说明就是本实例类型，那么就是方法，不是关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );

//     let rect2 = rect1.rotate();

//     println!(
//         "The area of the rectangle2 is {} square pixels.",
//         rect2.area()
//     );

// }

// 多个参数
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}

