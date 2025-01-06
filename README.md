# how_to_rust-
How to Rust?

Follow [this document](https://www.rust-lang.org/learn/get-started) to get started with rust. 


## Getting Started

Let's step into the world of Rust. 

### Validate Setup

First, let us validate the setup. We should have the rustc and cargo commands available.

```bash
$ rustc --version
rustc 1.83.0 (90b35a623 2024-11-26)

$ cargo --version
cargo 1.83.0
```

Rust has a six-week, rapid release process. To update the rust installation, use the `rustup update` utility.

### Hello World

Create a new file named main.rs and use your editor to write the following code into it:

```rust
fn main() {
	println!("Hello, world!");
}
```

Use `rustc` to compile your program into an executable file.

```bash
$ rustc main.rs
./main
```

### Hello Cargo

What is Cargo? Cargo is Rust’s build system and package manager. Most Rustaceans use it to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

Now, let's use Cargo to generates a boilerplate project for us. The command is `cargo new <program_name>`.

```bash
# 
$ cargo new hello-cargo

$ cd hello-cargo

# Now run the program
$ cargo run
```

It consists of a new directory named hello-cargo with a src subdirectory and adds two files: Cargo.toml and main.rs.

--------------------
## List of exercises

We have lots of things to cover so here is a step-wise exploration of the key topics.

### Topics:
- [Datatypes](./Datatypes.md)
- [Macros](./Macro.md)
- [Memory](./Memory.md)
- [Ownership](./Ownership.md)

### Code:

| Sequence | Topic  | Description |
| ---------| -----  | ----------- |
| 1 | [HelloWorld!](./code/helloworld/src/main.rs) | Getting started with Rust |
| 2 | [Variables and DataTypes](./code/variables/src/main.rs) | Exploring datatypes|
| 3 | [Custom libraries](./code/demo_package/src/main.rs) | How to create and use libraries|
| 4 | [Control flow](./code/control_flow/src/main.rs) | Exploring control flows|
| 5 | [Strings](./code/demo_strings/src/main.rs) | Exploring Strings, Ownerships, References and multable references |
| 6 | [Stack and Heap](./code/stack_and_heap/src/main.rs) | Exploring Stack and Heap |
| 7 | [References and Slices](./code/references_and_slices/src/main.rs) | Exploring Rust References and slices |
| 8 | [Structs and Traits](./code/structs_and_traits/src/main.rs) | Exploring Structs and Traits |
------------

You can find answers to common questions [here](./CommonQuestions.md)

## References

* [Ultimate-Rust-Crash-Course by Nathan Stocks](https://www.udemy.com/course/ultimate-rust-crash-course/)
* [Rust 2021 Fundamentals | Pluralsight](https://www.pluralsight.com/courses/rust-2021-fundamentals)
* [A Gentle Introduction To Rust | Bt Steve Donovan](https://stevedonovan.github.io/rust-gentle-intro/readme.html)
