fn main() {
    let mut v = vec![1, 2, 3, 4];
    println!("{}", v[3]);
    println!("{:?}", v);
    v.pop();
    println!("{:?}", v);
    v.push(12);
    println!("{:?}", v);
}
