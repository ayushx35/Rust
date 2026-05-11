fn main() {
    let a: Option<i32> = Some(5);

    let b: Option<i32> = None;

    let a1: Option<i32> = add_one(a);
    let b1: Option<i32> = add_one(b);

    println!("al is {:?}", al);
    println!("b1 is {:?}", b1);
}
fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
