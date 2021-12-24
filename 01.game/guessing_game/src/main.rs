use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("猜数字");
    let _num = rand::thread_rng().gen_range(1..101);
    println!("数字为:{}", _num);
    let mut guess_num: i32 = 0;
    // if guess == _num {
    //     println!("猜测成功");
    // } else if guess < _num {
    //     println!("猜小了");
    // } else if guess > _num {
    //     println!("猜大了");
    // }
    loop {
        guess_num += 1;
        println!("请输入你的猜想, 输入exit像懦夫一样退出");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("输入有误");
        if guess.trim() == "exit" {
            println!("切, 懦夫!");
            break;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                println!("请不要瞎 J * 输入");
                continue;
            }
        };
        println!("猜想为: {}", guess);

        match guess.cmp(&_num) {
            Ordering::Equal => {
                println!("猜测成功");
                println!("共猜测{}次", guess_num);
                break;
            }
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
        }
    }
}
