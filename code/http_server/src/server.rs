// Every file is it's own module
// Everything inside a module is private by default

use std::net::{TcpListener, TcpStream};

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

        let server_addr = format!("{}:{}", self.addr, self.port);

        // we use unwrap to get the value from the Result<T> enum
        // In case of an Err, we Panic and terminate the program
        let listener = TcpListener::bind(server_addr).unwrap();

        println!("Press Ctrl-C to exit...");
        // start an infinite loop
        loop {
            listener.accept();
        }
    }

    fn handle_client(&self, stream: TcpStream) {
        // ...
    } // functions - These are like static functions, they do not need an instance of the struct
}
