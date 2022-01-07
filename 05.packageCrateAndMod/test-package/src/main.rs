mod my_mod;
mod my_mod2;

use std::str::FromStr;

use my_mod::test_mod;
fn main() {
    println!("Hello, world!");
    my_mod::print(String::from("Hello, world!"));
    test_mod::t_main();
    my_mod2::m_print();
    let mut a = vec![1, 2, 3];
    println!("{:?}", a);
    a.push(4);
    println!("{:?}", a);
    println!("{:?}", &a[a.len() - 1]);
    for i in 0..a.len() {
        println!("{:?}", &a[i]);
    }
    let s = "abcdef";
    let mut s_v = s.to_string();
    s_v.push_str(&String::from("as"));
    println!("{:?}", s);
    println!("{:?}", &s_v);
    a[99];
}
