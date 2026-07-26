# Cargo - The Rust Package Manager

Cargo is the Rust package manager. It is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build.

To accomplish this goal, Cargo does four things:

1. Introduces two metadata files with various bits of package information.
2. Fetches and builds your package’s dependencies.
3. Invokes rustc or another build tool with the correct parameters to build your package.
4. Introduces conventions to make working with Rust packages easier.

---
## Use cargo instead of rustc

To compile the classic “hello world” program, we use the following command:

```rust
$ rustc hello.rs
$ ./hello
Hello, world!
```
Note that the above command required that you specify the file name explicitly. We also need to manage dependencies ourselves. 

Rather than invoke `rustc` directly, you can instead invoke something generic such as `cargo build` and let cargo worry about constructing the correct `rustc` invocation.

```rust
# Compile the program. Binary crate created in ./target/debug folder
$ cargo build

# Now we can run the code
$ ./target/debug/hello_world

# Compile the program with Production optimization. Binary crate created in ./target/release folder
$ cargo build --release

```

For dev builds, we can use the `cargo run` command that compiles and runs the program in one step.

Furthermore, Cargo will automatically fetch any dependencies you have defined for your artifact from a registry, and arrange for them to be added into your build as needed.

---

## Cargo.toml

`Cargo.toml` is a manifest file that contains all of the metadata that Cargo needs to compile your package. This file is written in the TOML format.

```Cargo.toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]

```
---

## Cargo.toml vs Cargo.lock

`Cargo.toml` and `Cargo.lock` serve two different purposes. Before we talk about them, here’s a summary:

1. Cargo.toml is about describing your dependencies in a broad sense, and is written by you.
2. Cargo.lock contains exact information about your dependencies. It is maintained by Cargo and should not be manually edited.
3. When in doubt, check Cargo.lock into the version control system (e.g. Git).

---

 
