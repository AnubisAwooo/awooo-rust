#![allow(unused)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

// 引入函数，引入到函数的上级模块
// 引入结构体、枚举 引入到名字
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::fmt;
use std::io;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

fn function3() -> IoResult<()> {
    // --snip--
    Ok(())
}

mod front_of_house2 {
    pub mod hosting2 {
        pub fn add_to_waitlist() {}
    }
}
// pub use front_of_house2 没有对外公开引用，使用 pub use 才暴露出 hosting2 模块
pub use crate::front_of_house2::hosting2;

pub fn eat_at_restaurant3() {
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
}

// 嵌套路径引入
use std::{cmp::Ordering, io as io2};
use std::io::{self as io3, Write};
use std::collections::*;
