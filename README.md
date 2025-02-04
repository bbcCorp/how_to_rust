# How to Rust?

Now, that is an interesting question about an interesting programming language.

We will explore the key concepts of this language through a series of examples. Once we have covered the basics, we will implement some common algorithms using Rust.

If you are new to Rust, you can follow the trainings and books mentioned in the reference section.


## Getting Started

Let's step into the world of Rust.

## Installation

```bash
# On Mac, use homebrew
brew install rustup

# On Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Once rustup is installed, use this command
rustup-init
```

`rustup-init`, which will install Rust and the Rust Toolchain, which includes the following:
- rustc - the Rust compiler
- cargo - the Rust package manager
- rustup - the Rust toolchain manager

### Validate Setup

First, let us validate the setup. We should have the rustc and cargo commands available.

```bash
$ rustc --version
rustc 1.83.0 (90b35a623 2024-11-26)

$ cargo --version
cargo 1.83.0
```

Rust has a six-week, rapid release process. To update the rust installation, use the `rustup update` utility.


### IDE Setup

If you are comfortable wih a vim like experience consider [Helix Editor](https://helix-editor.com)

```bash
# On a Mac
$ brew install helix rust-analyzer llvm marksman

# On Ubuntu 
## Add ppa
$ sudo add-apt-repository ppa:maveonair/helix-editor
$ sudo apt update

$ sudo apt install helix xclip rust-analyzer llvm
```

### VSCode
You can follow [this guide](https://code.visualstudio.com/docs/languages/rust)

---


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

---

### Topics:

- [Datatypes](./notes/Datatypes.md)
- [Macros](./notes/Macro.md)
- [Memory](./notes/Memory.md)
- [Ownership](./notes/Ownership.md)
- [Lifetime](./notes/Lifetime.md)

---

### Code:

| Sequence | Topic  | Description |
| ---------| -----  | ----------- |
| 1 | [HelloWorld!](./code/helloworld/src/main.rs) | Getting started with Rust |
| 2 | [Variables and DataTypes](./code/variables/src/main.rs) | Exploring datatypes|
| 3 | [Structs and Traits](./code/structs_and_traits/src/main.rs) | Exploring Structs and Traits |
| 4 | [Control flow](./code/control_flow/src/main.rs) | Exploring control flows|
| 5 | [Functions and Closures](./code/demo_closures_and_functions/src/main.rs) | Exploring functions and closures|
| 6 | [Collections](./code/demo_collections/src/main.rs) | Exploring sequences, maps and sets |
| 7 | [Custom libraries](./code/demo_package/src/main.rs) | How to create and use libraries|
| 8 | [Strings](./code/demo_strings/src/main.rs) | Exploring Strings, Ownerships, References and multable references |
| 8 | [Stack and Heap](./code/stack_and_heap/src/main.rs) | Exploring Stack and Heap |
| 10 | [References and Slices](./code/references_and_slices/src/main.rs) | Exploring Rust References and slices |
| 11 | [Generics](./code/demo_generics/src/main.rs) | Exploring Generics in Rust |
| 12 | [File I/O](./code/demo_fileio/src/main.rs) | Exploring File I/O in Rust |
| 13 | [Threads](./code/demo_thread/src/main.rs) | Exploring Threads in Rust |
| 13 | [Mutexes and Arc](./code/demo_mutex/src/main.rs) | Exploring Mutexes in Rust |
------------

You can find answers to common questions [here](./CommonQuestions.md)

Next we implemented some algorithms

| Sequence | Topic  | Description |
| ---------| -----  | ----------- |
| 1 | [LinkedList using Vector](./code/algorithms/src/linked_list/mod.rs) | Linked List implementation using Vec |

---

## References
* [Official | Getting started guide](https://www.rust-lang.org/learn/get-started) 
* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
* [Ultimate-Rust-Crash-Course by Nathan Stocks](https://www.udemy.com/course/ultimate-rust-crash-course/)
* [Rust 2021 Fundamentals by Zachary Bennett | Pluralsight](https://www.pluralsight.com/courses/rust-2021-fundamentals)
* [A Gentle Introduction To Rust | Bt Steve Donovan](https://stevedonovan.github.io/rust-gentle-intro/readme.html)
* [Implementing Concurrency in Rust | By Shaun Meyer](https://app.pluralsight.com/library/courses/rust-implementing-concurrency/)

---
