mod my_mod {
    #[derive(Debug)]
    pub struct TestStruct {
        num: i32,
    }
    impl TestStruct {
        pub fn new(num: i32) -> TestStruct {
            if num > 100 {
                panic!("value mast be low 100");
            }
            TestStruct { num }
        }
        pub fn get_num(&self) -> i32 {
            self.num
        }
    }
}
fn main() {
    println!("Hello, world!");
    let t_s = my_mod::TestStruct::new(100);
    println!("{:?}", t_s);
    let a = match use_test_err(200) {
        Ok(num) => num,
        Err(err) => {
            panic!("{}", err);
        }
    };
    println!("{}", a);
    let b = use_test_err(100).expect("输入有误");
    println!("{}", b);
}

fn use_test_err(num: i32) -> Result<i32, String> {
    let num = test_err(num)?;
    Ok(num)
}

fn test_err(num: i32) -> Result<i32, String> {
    if num > 100 {
        return Ok(num);
    } else {
        return Err(String::from("这是一个错误的值"));
    }
}
