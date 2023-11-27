# Macro

Macro is a set of instructions that is invoked by a name. It is a way to reduce the amount of code you have to write. It is similar to a function, but it is expanded inline when it is invoked. It is a preprocessor directive.

Example:

Heres a [sample code](./code/intro_to_macro/src/main.rs) that uses the println macro.

```rust
```rust
println!("Hello, world!");

println!("{} days in {}", 31, "December");
```

Macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. 

`cargo expand` can be used to see what all the macros in our code are expand into. Install with `cargo install cargo-expand`.

```bash
$ cargo expand
    Checking intro_to_macro v0.1.0 (/Users/bbc/progs/rust/how_to_rust/code/intro_to_macro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
    {
        ::std::io::_print(format_args!("{0} days in {1}\n", 31, "December"));
    };
}
```