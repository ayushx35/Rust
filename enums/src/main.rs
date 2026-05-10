fn main() {
    let color = Color::White;
    let color1 = Color::Black;
    println!("{:?}", color);
    println!("{:?}", color1);
}
#[derive(Debug)]
enum Color {
    Black,
    White,
}
