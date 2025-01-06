// main is the starting point of rust programs
fn main() {
    // On stack
    {
        let a = 1;
        let b = a;

        // a and _b can both be used since these are simple values stored on stack
        // b is a shallow copy of a
        println!("On Stack a={} b={}", a, b);
    }

    // On heap
    {
        // s1 and _s2 are stack variables that point to a location on the heap
        // it stores a pointer to the heap location and the size of the data

        let s1 = String::from("hello");

        let s2 = s1;

        // The value of s1 has been MOVED to s2. Beyond this point we cannot use s1
        // They both CANNOT be used at the same time
        // println!("On Heap s1={} s2={}", s1, s2); will result in error

        // If we want to use both the variables, we will need to clone the data
        // Clone trait creates a deep copy of the data,
        // so s2 and s3 refers to different memory locations with same data
        let s3 = s2.clone();

        println!("On Heap s2={} s3={}", s2, s3);

        let p1 = Person {
            id: 1,
            name: String::from("John"),
            age: 25,
        };
        let p2 = p1.clone();
        println!("p1 = {:#?}", p1);
        println!("p2 = {:#?}", p2);
    }

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

#[derive(Debug, Clone)]
struct Person {
    id: u8,
    name: String,
    age: u8,
}
