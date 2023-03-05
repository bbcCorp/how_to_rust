pub fn demo_scalar() {
    println!("\n\n Demo scalar");

    let i = 16;
    println!("i = {}", i);

    let j = 20;
    println!("j = {}", j);

    let sum = i + j;
    println!("sum = {}", sum);

    // calculate area of circle
    let radius = 5.0;
    println!("area = {}", get_area(radius));
}

pub fn demo_tuple() {
    println!("\n\n Demo tuple");

    let tup = (1, 2, 3, 4, 5);
    println!("tup = {:?}", tup);
    // println!("Length of tup = {}", tup.len());
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);

    let (x, y, z, a, b) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("a = {}", a);
    println!("b = {}", b);

    // mixed type typle
    let tup2 = (1, 3.14159, "Hello world");
    let pi = tup2.1;
    println!("pi = {}", pi);
}

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
}

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
