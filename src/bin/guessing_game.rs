//! 《Rust 程序设计语言》第 2 章的猜数游戏练习。
//!
//! 涉及的知识点：用 `rand` 生成随机数、`String::new()` 与可变绑定、
//! `io::stdin().read_line()` 返回的 `io::Result` 用 `expect` 处理失败、
//! `parse` 把字符串转成数字、`cmp` 配合 `match` 做分支比较。
//!
//! 运行：`cargo run --bin guessing_game`

use rand::RngExt; // 随机范围方法在 RngExt 扩展 trait 上
use std::cmp::Ordering;
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

    // read_line 读进来的是字符串（还带着结尾的换行），要跟数字比较就得先转换：
    // trim 去掉换行，parse 按标注的类型 u32 解析，失败时用 expect 给出提示。
    let guess: u32 = guess.trim().parse().expect("请输入一个数字！");

    println!("你猜测的数是：{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("To big!"),
        Ordering::Equal => println!("You win!"),
    }
}
