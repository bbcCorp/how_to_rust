use std::io;

fn main() {
    handle_input();
}

fn handle_input() {

    // allocate a new string to store the input
    // String is a struct that is growable, UTF-8 encoded, and heap allocated
    let mut input = String::new();
    println!("Enter your name: ");

    // We need to use &mut string to pass a mutable reference to the string
    io::stdin().read_line(&mut input).unwrap();
    
    println!("Hello {}", input);
}

