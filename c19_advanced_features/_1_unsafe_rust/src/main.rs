#![allow(unused)]
// 解引用裸指针
// - 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
// - 不保证指向有效的内存
// - 允许为空
// - 不能实现任何自动清理功能


fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);


    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 导入外部其他语言的函数 ABI
extern "C" {
    fn abs(input: i32) -> i32;
}

// 允许外部调用 rust 函数  no_mangle 告诉编译器不要修改这个函数的名称
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

