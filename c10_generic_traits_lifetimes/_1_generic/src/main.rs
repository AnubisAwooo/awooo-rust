fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');



    struct Point<T> {
        x: T,
        y: T,
    }
    
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    
    impl<T> Point<T> { // 方法定义泛型 必须在 impl 关键字后面指出 T 是泛型
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    impl Point<f32> { // 具体类型
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }



}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest { // 泛型需要限定，并不是每个类型都有 > 方法实现
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T, U> { // 为啥 struct 不需要在后面加上 <T, U> 呢？
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> { // 方法上添加其他泛型,总得有地方声明这是泛型啊，
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
