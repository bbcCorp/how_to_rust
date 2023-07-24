# how_to_rust-
How to Rust?

Follow [this document](https://www.rust-lang.org/learn/get-started) to get started with rust. 


## Getting Started

Let's step into the world of Rust. 

### Validate Setup

First, let us validate the setup. We should have the rustc and cargo commands available.

```bash
$ rustc --version
rustc 1.67.1 (d5a82bbd2 2023-02-07)

$ cargo --version
cargo 1.67.1 (8ecd4f20a 2023-01-10)
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

Now, let's use Cargo generates a boilerplate project for us.

It consists of a new directory named hello-cargo with a src subdirectory and adds two files: Cargo.toml and main.rs.

```bash
# 
$ cargo new hello-cargo

$ cd hello-cargo

# Now run the program
$ cargo run
```

--------------------
## List of exercises

We have lots of things to cover so here is a step-wise exploration of the key topics.



| Sequence | Topic  | Description |
| ---------| -----  | ----------- |
| 1 | [HelloWorld!](./code/helloworld/src/main.rs) | Getting started with Rust |
| 2 | [Variables and DataTypes](./code/variables/src/main.rs) | Exploring datatypes|
| 3 | [Custom libraries](./code/demo_package/src/main.rs) | How to create and use libraries|
| 4 | [Control flow](./code/control_flow/src/main.rs) | Exploring control flows|
| 5 | [Strings](./code/demo_strings/src/main.rs) | Exploring Strings, Ownerships, References and multable references |

------------

## References

* [Ultimate-Rust-Crash-Course by Nathan Stocks](https://www.udemy.com/course/ultimate-rust-crash-course/)