fn main() {
    let ayush = Color(1, 2, 3.22, String::from("hrjek"));
    println!("{}", ayush.0);
    println!("{}", ayush.1);
    println!("{}", ayush.2);
    println!("{}", ayush.3);
}
struct Color(i32, i32, f32, String);
