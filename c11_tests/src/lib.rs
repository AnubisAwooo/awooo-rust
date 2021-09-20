// cargo test -- --test-threads=1 // 指定线程数
// cargo test -- --nocapture // 打印输出
// cargo test one_hundred // 单个测试
// cargo test add // 运行 add 前缀的测试函数
// #[ignore] // 有该注解的测试函数忽略
// cargo test -- --ignored // 运行带有忽略标记的测试函数


#[cfg(test)] // 测试模块 cfg 表示需要特定配置才启用
mod tests {

    // #[should_panic] //方法上的注解
    // #[should_panic(expected = "Guess value must be less than or equal to 100")] // 仔细判断 panic 内容
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        // assert_ne!
        // assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result); // 提示额外错误
        
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
