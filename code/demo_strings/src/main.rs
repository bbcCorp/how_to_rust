fn main() {
    
    read_commandline();

    string_ownership();

    string_ownership2();
}

fn read_commandline() {

    println!("Test command line using multiple params and flags");
    println!("Reading command line arguments");

    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("args: {:?}", args);
}

fn string_ownership(){
    let s1 = String::from("abc");
    println!("s1: {}", s1);

    // there is no copy, the value of s1 has been MOVED to s2
    let s2 = s1;

    println!("s2: {}", s2);

    // However now we will not be able to access the value using s1
    // this statement will throw an error
    // println!("s1: {}", s1);

    // we can clone the value of s2 to s3
    // In rust, we use the term copy if only stack data is copied
    // if heap data is copied or pointer manipulation is involved,  we use the term clone
    let s3 = s2.clone();
    println!("s3: {}", s3);

}

fn string_ownership2(){
    let s1 = String::from("abc");
    let mut s2 = s1.clone();

    println!("string_ownership2:: s1: {}", s1);
    println!("string_ownership2:: s2: {}", s2);

    // calling a function with a string param results in a MOVE operation    
    do_stuff_with_string(s1);
    // So this statement will result in a compile time error
    //println!("s1: {}", s1);

    // here s2 is passed as a IMMUTABLE reference (a pointer to the string)
    // even though the value being passed is mutable. 
    do_stuff_with_string_reference(&s2);
    println!("string_ownership2:: s2 after function call : {}", s2);

    // to pass sting to a function as a mutable reference, we need to use the mut keyword
    update_stuff_with_string_reference(&mut s2);
    println!("string_ownership2:: s2 after mutable function call : {}", s2);

    replace_string_using_reference(&mut s2);
    println!("string_ownership2:: s2 after mutable replacement function call : {}", s2);

    // At any point in time, we can have only one mutable reference to a string and any number of immutable references 
    // across ALL threads.
}

// function takes a string as a param
fn do_stuff_with_string(s: String) {
    println!("do_stuff_with_string:: s: {}", s);
}


// function takes a string REFERENCE as a param
fn do_stuff_with_string_reference(s: &String) {
    println!("do_stuff_with_string_reference:: s: {}", s);
}

// function takes a string REFERENCE as a param
fn update_stuff_with_string_reference(s: &mut String) {
    println!("do_stuff_with_string_reference:: s: {}", s);

    // note: The dot operator auto dereferences down to the actual string value so we can use regular string methods    
    s.insert_str(0, "[Update in progress ...]  ")
}

// function takes a string REFERENCE as a param
fn replace_string_using_reference(s: &mut String) {

    // using dereference operator to change value of the string   
    (*s) = String::from("We have CHANGED!");    
}