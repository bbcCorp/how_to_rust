// Every file is it's own module
// Everything inside a module is private by default

// define a struct for the server
pub struct Server {
    // struct definition
    addr: String,
    port: i32,
}

impl Server {
    // put in the functionality of a Server here

    // methods - defined on the context of the struct (self)
    pub fn new(addr: String, port: i32) -> Self {
        Self { addr, port }
    }

    pub fn run(&self) {
        // run will take the ownership of the self reference
        println!(
            "Server running at addr:{} and port:{}",
            self.addr, self.port
        );
    }

    // functions - These are like static functions, they do not need an instance of the struct
}
