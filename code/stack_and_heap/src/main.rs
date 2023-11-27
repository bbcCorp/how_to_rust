// main is the starting point of rust programs
fn main() {
    println!("Function call with parameters");
    let a = 2;
    let result = stack_only(a);
    dbg!(result);
}

// Functions are defined with fn keyword
// Parameters are defined with type after the variable name
// Return type is defined with -> after the parameters
// Return type is optional

fn stack_only(b: i32) -> i32 {
    // Variables are immutable by default
    // To make them mutable, use mut keyword
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    // Box is a smart pointer that points to a value on the heap
    let e = Box::new(7);
    return get_default() + *e;
}

// function with implicit return
fn get_default() -> i32 {
    5
}
