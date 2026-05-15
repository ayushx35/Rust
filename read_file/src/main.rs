use std::fs::read_to_string;

fn main() {
    let result = read_to_string("home.txt");

    match result {
        Ok(content) => println!("File content:\n{}", content),
        Err(error) => panic!("Something went wrong: {}", error),
    };
}
