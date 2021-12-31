// 以官网示例的ip作为例子
/*
// 普通使用
#[derive(Debug)]
enum IpAddr {
    V4,
    V6,
}
struct RIpAddr {
    ip_type: IpAddr,
    addr: String,
}

fn main() {
    let v4_test = RIpAddr {
        ip_type: IpAddr::V4,
        addr: String::from("127.0.0.1"),
    };

    println!("{:#?}", v4_test.ip_type);
    println!("{:?}", v4_test.addr);
}
*/
use rand::Rng;
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
impl IpAddr {
    fn print_debug(&self) {
        match self {
            IpAddr::V4(v1, v2, v3, v4) => {
                println!("{:?}.{:?}.{:?}.{:?}", v1, v2, v3, v4);
            }
            IpAddr::V6(val) => {
                println!("{:?}", val)
            }
        }
    }
}
fn main() {
    let r: u32 = rand::thread_rng().gen_range(0..100);
    println!("{:?}", r);
    let v4_test = IpAddr::V4(127, 0, 0, 1);
    let v6_test = IpAddr::V6(String::from("::1"));
    v4_test.print_debug();
    v6_test.print_debug();

    let smoe_number = Some(5);
    let empty_number: Option<i32> = None;
    let some_str = Some(String::from("123"));

    match some_str {
        Some(val) => {
            println!("{:?}", val);
        }
        None => {
            println!("this is None");
        }
    }
}
