// declarative macros
fn main() {
    hello!();
    hello!("Ayush");
}

#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello")
    };
    ($w:expr) => {
        println!("{}", $w)
    };
}
