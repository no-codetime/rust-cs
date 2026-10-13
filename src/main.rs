use std::io;
use webbrowser;

fn main() {
    println!("杨锦江nb");
    println!("杨锦江是什么？：");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取");
    println!("杨锦江是：{}",guess);
    webbrowser::open("https://www.baidu.com");
    println!("\n按回车键退出...");
    let mut pause = String::new();
    io::stdin().read_line(&mut pause).unwrap();
}