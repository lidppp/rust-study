use std::{io, ops::Add};
fn main() {
    // let mut arr = [1; 3];
    // println!("arr: {:?}", arr);
    // arr[1] = 2;
    // println!("arr: {:?}", arr);
    // arr[2] = 3;
    // println!("arr: {:?}", arr);
    // let mut c = 233;
    // let x = test_fn(&mut c);
    // println!("x: {:?}", x);
    // println!("x: {:?}", c);
    // let test_num = "-8888";
    // let test_parse_num: i32 = match test_num.parse::<i32>() {
    //     Ok(num) => num,
    //     Err(_) => todo!(),
    // };

    // println!("test_parse_num: {:?}", test_parse_num);

    // let mut str: String = String::new();
    // io::stdin().read_line(&mut str).expect("输入有误");
    // let num: i32 = match str.trim().parse() {
    //     Ok(res) => res,
    //     Err(_) => return,
    // };
    // println!("num: {:?}", num);
    let mut s = String::from("new String");
    let s1 = &s;
    println!("s: {:?} ", s1);
    println!("num: {:?}", s1);

    let s2 = find_first_word(&s);
    println!("s2: {:?}", s2)
}

fn my_print(print_str: String) -> (String, usize) {
    println!("{:?}", print_str);
    let len = print_str.len();
    (print_str, len)
}

fn test_fn(val: &mut i32) -> i32 {
    for number in (1..10).rev() {
        println!("{}!", number);
    }
    println!("this is testFn");
    *val += 10;
    *val
}

fn find_first_word(str: &str) -> &str {
    for (i, item) in str.chars().enumerate() {
        if item == ' ' {
            return &str[..i];
        }
    }
    &str[..]
}
