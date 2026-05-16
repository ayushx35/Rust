use clap::Parser;
fn main() {
    let args = Args::parse();
    println!("name is {} and age is {}", args.name, args.name);
}

#[derive(Parser)]
struct Args {
    name: String,
    age: i32,
}

// for universal -- cargo install --path .
// uninstall -- cargo uninstall <name of cli>
