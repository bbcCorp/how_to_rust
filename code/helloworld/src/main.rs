use std::io::stdin;

fn main() {
    // define an immutable string
    let msg: String = "Hello, world";

    println!("Enter your name: ");

    // create a mutable string to handle input
    let mut input: String = String::new();

    // read the input
    // Note: read_line() reads a line of input from the standard input and stores it in a String.
    // It returns a Result value, which indicates whether the operation was successful or not.

    stdin().read_line(&mut input).unwrap();

    // trim whitespaces from the entered name
    let name: String = input.trim().parse().unwrap();

    println!("{} {}!", msg, name);
}
