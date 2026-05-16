// declarative macros
fn main() {
    hello!();
    hello!("Ayush");
    hello!("Ayush", "Singh");
}

#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello")
    };
    ($w:expr) => {
        println!("{}", $w)
    };
    ($w:expr,$a:expr) => {
        println!("{} {}", $w, $a)
    };
}
