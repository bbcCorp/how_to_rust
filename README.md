# how_to_rust-
How to Rust?


## Hello World!


```bash

# Create the program
$ cargo new helloworld 
     Created binary (application) `helloworld` package

# build and run the cargo package
$ cargo run

   Compiling helloworld v0.1.0 (/home/bbc/progs/rust/how_to_rust/helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/helloworld`
Hello, world!

# The code would be created in target/debug folder
$ ./target/debug/helloworld 
Hello, world!



# To build the code in the release mode
$ cargo run --release      
   Compiling helloworld v0.1.0 (/home/bbc/progs/rust/how_to_rust/helloworld)
    Finished release [optimized] target(s) in 0.16s
     Running `target/release/helloworld`
Hello, world!


```