struct User {
    username: String,
    password: String,
    age: i32,
    is_sigin: bool,
}
struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("lidppp"),
        password: String::from("123456"),
        age: 18,
        is_sigin: false,
    };
    let user2 = User {
        username: String::from("1512282028@qq.com"),
        ..user1
    };
    let color = Color(233, 233, 155);
    print_color(&color);
}

fn print_color(color: &Color) {
    println!("R:{:?}  G:{:?}  B:{:?}", color.0, color.0, color.0)
}
