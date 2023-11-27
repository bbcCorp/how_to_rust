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
    // The read_line returns a result, so we need to unwrap it
    // in case there was an error, unwrap will panic and terminate the program
    // in case result is successful, it will return the number of bytes read
    io::stdin().read_line(&mut input).unwrap();
    
    println!("Hello {}", input);
}

