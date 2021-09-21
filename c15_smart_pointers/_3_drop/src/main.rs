#![allow(unused)]

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c); //  丢弃回调函数会按照对象创建逆序方式再结束生命周期时调用  但是内建方法 drop 可以用代码方式控制丢弃后面就不能用该变量了
    println!("CustomSmartPointers created.");
}
