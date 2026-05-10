fn main() {
    let ayush = Man {
        name: String::from("Ayushmaan"),
        age: 23,
    };
    ayush.hello();
    Man::bye();
}

struct Man {
    name: String,
    age: i32,
}

impl Man {
    fn hello(&self) {
        println!("Hello from {} and i am {} years old", &self.name, &self.age);
    }
    fn bye() {
        println!("BYE EVERYONE !!");
    }
}
