use hello1::hello1 as hq;
use hello1::hi1 as tt;
mod fruit;
mod hi;
fn main() {
    hq();
    tt::hiq();
    hello::hello();
    hello::hi::hi();
    fruit::fruits();
    crate::hi::ayush::ayu();
    crate::hi::singh::sin();
}

mod hello {
    pub fn hello() {
        println!("Hello!");
    }
    pub mod hi {
        pub fn hi() {
            println!("hi");
        }
    }
}

mod hello1 {
    pub fn hello1() {
        println!("Hello! there");
    }
    pub mod hi1 {
        pub fn hiq() {
            println!("hi there");
        }
    }
}
