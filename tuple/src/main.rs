fn main() {
    // define tuple
    let mut tup: (&str, i32, u32, f32) = ("", 0, 0, 0.0);
    let tup1 = ("String", -12, 10);
    let tup2: (&str, i32, u32, f32) = ("String", -12, 10, 1.2);
    // update tuple
    // Note: for updation it is necessary that the tuple is initialized with some default values
    tup.0 = "String";
    tup.1 = -100;
    tup.2 = 100;
    tup.3 = 12.3333;
    // Print tuple
    println!("{:?}", tup);
    println!("{:?}", tup1);
    println!("{:?}", tup2);
}
