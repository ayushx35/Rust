fn main() {
    let color = Color::Black;
    let color1 = Color::White;
    println!("color is {:?}", color);
    println!("color is {:?}", color1);
}
#[derive(Debug)]
enum Color {
    Black,
    White,
}
