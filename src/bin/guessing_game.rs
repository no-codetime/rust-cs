//! 《Rust 程序设计语言》第 2 章的猜数游戏练习。
//!
//! 涉及的知识点：用 `rand` 生成随机数、`String::new()` 与可变绑定、
//! `io::stdin().read_line()` 返回的 `io::Result` 用 `expect` 处理失败。
//!
//! 运行：`cargo run --bin guessing_game`

use rand::RngExt; // 随机范围方法在 RngExt 扩展 trait 上
use std::io; // prelude // trait

fn main() {
    println!("猜数游戏！！");

    // 等价于 rand 0.8 的 rand::thread_rng().gen_range(1, 101)
    let secret_number = rand::rng().random_range(1..101);

    println!("神秘数字是：{}", secret_number);

    println!("猜测一个数");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");
    // io::Result Ok, Err

    println!("你猜测的数是：{}", guess);
}
