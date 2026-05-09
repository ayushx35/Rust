fn main() {
    let s1 = String::from("For a value there can only be one owner");
    // ownership changed to s2, s1 is out of scope now
    let s2 = s1;
    // print does not work for s1 because s1 is out of scope
    // println!("{s1}");
    println!("{s2}");
}
