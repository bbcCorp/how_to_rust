use std::io::stdin;

fn main() {
    // define an immutable string
    let msg = "Hello, world";

    // create a mutable string
    let mut name: String = String::new();
    println!("Enter your name: ");

    // read the input
    stdin().read_line(&mut name).unwrap();

    // trim whitespaces from the entered name
    name = name.trim().to_string();

    println!("{} {}!", msg, name);
}
