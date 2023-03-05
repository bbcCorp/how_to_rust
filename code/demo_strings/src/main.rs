fn main() {
    
    read_commandline();

    string_ownership();
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

    // there is no copy, the value of s1 has been moved to s2
    let s2 = s1;

    println!("s2: {}", s2);

    // However now we will not be able to access the value using s1
    // this statement will throw an error
    // println!("s1: {}", s1);

}