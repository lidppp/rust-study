use std::fmt::Display;

#[derive(Debug)]
struct Point<'a,T> {
    x: T,
    y: T,
    str: &'a str
}
impl<'a,T> Point<'a,T> {
    fn new(x: T, y: T, str:&'a str) -> Self {
        Point { x, y ,str}
    }
}

trait PointMove<T> {
    fn move_point(&self, x: T, y: T) -> Point<T>;
}

impl<'a,T: Copy + std::ops::Add<Output = T>> PointMove<T> for Point<'a,T> {
    fn move_point(&self, x: T, y: T) -> Point<T> {
        Point {
            x: self.x + x,
            y: self.y + y,
            str: self.str
        }
    }
}
impl<'a,T: std::fmt::Debug> Display for Point<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point_My x:{:?},y:{:?},str:{:?}", self.x, self.y,self.str)
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> String {
    String::from("really long string")
}

fn main() {
    let point = Point::new(11, 1, "point1");
    let point_b = point.move_point(10, 10);
    println!("{:?}", point);
    println!("{:?}", point_b);
    let str = String::from("这是一个长度为13的字符串");
    println!("str-len: {}", str.len());
    println!("str-len: {}", str.chars().collect::<Vec<char>>().len());
    
    
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
