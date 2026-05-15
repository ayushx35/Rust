fn main() {
    let v = vec![1, 2, 3, 4];
    let i = 9;
    let ans = match v.get(i).ok_or("Index out of bounds") {
        Ok(w) => w,
        Err(e) => panic!("Error occurred: {}", e),
    };
    println!("The value is: {}", ans);
}
