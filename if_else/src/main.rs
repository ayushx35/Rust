fn main() {
    println!("{}", is_even(0));
}

fn is_even(num: i32) -> String {
    if num == 0 {
        return "Really !!!... creative".to_string();
    } else if num % 2 == 0 {
        return "true".to_string();
    } else {
        return "false".to_string();
    }
}
