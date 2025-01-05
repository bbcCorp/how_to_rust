#![crate_name = "demo_package"]

/// A simple function to demonstrate operations on scalar types
pub fn demo_scalar() {
    println!("\n\n Demo scalar \n");

    let flag: bool = true;
    println!("flag = {}", flag);
    assert_eq!(flag, true);

    println!("\n\n Demo int operations");
    let i = 16;
    println!("i = {}", i);

    let j = 20;
    println!("j = {}", j);

    let sum = i + j;
    println!("sum = {}", sum);

    // floating point operations
    println!("\n\n Demo floating point operations");
    let pi: f32 = 3.14159;
    println!(
        "pi = {}, floor={} ceil={} round={}",
        pi,
        pi.floor(),
        pi.ceil(),
        pi.round()
    );

    // calculate area of circle
    let radius = 5.0;
    println!("area = {}", get_area(radius));

    // char operations
    println!("\n\n Demo char operations");
    let ch = 'a';
    println!("ch = {}", ch);
    println!("ch as i32 = {}", ch as i32);
    println!("is uppercase? {}", ch.is_uppercase());
    println!("is lowercase? {}", ch.is_lowercase());
    println!("to string: {}", ch.to_string());
}

/// A simple function to demonstrate operations on tuple
pub fn demo_tuple() {
    println!("\n\n Demo tuple");

    let tup = (1, 2, 3.14159, 'a', "Hello");
    println!("tup = {:?}", tup);
    // println!("Length of tup = {}", tup.len());
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);

    println!("\n\n Demo tuple destructuring");
    let (x, y, pi, a, b) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("pi = {}", pi);
    println!("a = {}", a);
    println!("b = {}", b);
}

/// A simple function to demonstrate operations on array
pub fn demo_array() {
    // Demo of array
    println!("\n\n Demo array");

    // crate a buffer of 3 bytes with all 0
    let buf = [0; 3];
    println!("buf = {:?}", buf);
    println!("buf len = {}", buf.len());

    // array with type def
    let buf2: [u8; 3] = [1, 2, 3];
    println!("buf2 = {:?}", buf2);

    // array with default value
    let buf3: [u8; 3] = [0; 3];
    println!("buf3 = {:?}", buf3);
}

/// A simple function to demonstrate operations on vectors
pub fn demo_vectors() {
    // beyond a size of 32 use a vector
    println!("\n\n Demo vector");

    let vector = vec![1, 2, 3, 4, 5, 6];
    println!("vector = {:?}", vector);
    println!(
        "vector len:{} first:{} last:{}",
        vector.len(),
        vector[0],
        vector[vector.len() - 1]
    );

    // iterating a vector
    for i in 0..vector.len() {
        println!("vector[{}] = {}", i, vector[i]);
    }
}

/// A function to calculate area of a circle
pub fn get_area(radius: f64) -> f64 {
    // floating point
    const PI: f64 = 3.14159;

    let area = PI * radius * radius;
    return area;
}

pub fn exercise3() {
    println!("\n\n Exercise 3");

    pub fn print_array(array: [f32; 2]) {
        for i in 0..array.len() {
            println!("array[{}] = {}", i, array[i]);
        }
    }

    pub fn print_difference(x: f32, y: f32) {
        println!("The difference is: {}", x - y);
    }

    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_array: [f32; 2] = [coords.0, coords.1];
    print_array(coords_array);
}

// Define a custom data type
// the derive debug macro is used to print the struct
#[derive(Debug)]
struct Person {
    id: u32,
    name: String,
    age: u8,
    email: String,
}

pub fn demo_struct() {
    // create an immutable struct object
    let p1 = Person {
        id: 1,
        name: String::from("BBC"),
        age: 25,
        email: String::from("bbc@gmail.com"),
    };

    println!("\n\n Demo struct");
    println!("p1 = {:#?}", p1);
    println!("Name: {}", p1.name);
    println!("Age: {}", p1.age);
    println!("Email: {}", p1.email);

    // create a mutable struct object
    // combine data from a base struct
    let mut p2 = Person { id: 2, ..p1 };
    println!("p2 = {:#?}", p2);
}
