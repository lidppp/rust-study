use rand::Rng;
use std::io;
fn main() {
    println!("猜数字");
    println!("请输入你的猜想");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("输入有误");
    println!("猜想为: {}", guess);
    let _num = rand::thread_rng().gen_range(1..101);
    println!("数字为:{}", _num)
}
