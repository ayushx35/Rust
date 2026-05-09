fn main() {
    let s = String::from("I am owned by s");
    let s1 = &s;
    println!("{s1} but s1 has taken references from me");
    println!("{s}");
}
