use std::io;
use rand::Rng;


fn main() {
    println!("----猜数字---");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("数字是：{}", secret_number);
    let mut guess = String::new();
    println!("请输入你猜的数字："); 
    io::stdin().read_line(&mut guess).expect("无法读取");
    println!("你猜的数字是：{}", guess.trim());
}