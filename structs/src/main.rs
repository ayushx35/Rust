fn main() {
    let ayush = Man {
        name: String::from("Ayushmaan"),
        age: 23,
        is_male: true,
    };

    println!("{}", ayush.name);
    println!("{}", ayush.age);
    println!("{}", ayush.is_male);
}

struct Man {
    name: String,
    age: i32,
    is_male: bool,
}
