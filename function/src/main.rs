fn main() {
    hello();
    println!("{}", give_five());
    my_age("Ayush", 25);
    println!("4/2={}", divide(4, 2));
}

// No return type
// No Parameters
fn hello() {
    println!("Hello from function");
}

// return type
// No Parameters
fn give_five() -> i32 {
    return 5;
}

// No return type
// Parameters
fn my_age(name: &str, age: i32) {
    println!("My Name is {} and my age is {}", name, age);
}

// return type
// Parameters
fn divide(num1: i32, num2: i32) -> i32 {
    let ans = num1 / num2;
    return ans;
}
