# Lifetimes in Rust

In Rust, a lifetime is a way to specify the scope of a reference to a value. It's a way to tell the compiler how long a reference to a value is valid.

Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.

In Rust, references are not allowed to outlive the data they point to. This means that a reference to a value must be valid for at least as long as the value itself. Lifetimes help the compiler ensure that this rule is followed.

The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

Example:

```rust
    // Demo of lifetime in Rust
    {
        let s1: &String;
        {
            let s2 = String::from("test");
            s1 = &s2;
        } // s2 is valid only till this scope

        println!("{}", s1);
    }

```

And we will see the following error

```bash
error[E0597]: `s2` does not live long enough
  --> src/main.rs:25:18
   |
24 |             let s2 = String::from("test");
   |                 -- binding `s2` declared here
25 |             s1 = &s2;
   |                  ^^^ borrowed value does not live long enough
26 |         }
   |         - `s2` dropped here while still borrowed
27 |         println!("{}", s1);
   |                        -- borrow later used here
```

This is because s1 which holds the reference to s2, cannot outlive s2, and s2 dies once the scope in which it is defined, ends.

## Types of lifetimes

There are several types of lifetimes in Rust:

- *Inferred lifetime*: This is a lifetime that is inferred by the compiler based on the context.
- *Dynamic lifetime*: This is a lifetime that is determined at runtime.
- *Static lifetime*: This is the longest possible lifetime, which is the entire duration of the program. Use `'static` to indicate static lifetime.


## Lifetime annotations

A lifetime is a way to specify the scope of a reference to a value. It's a way to tell the compiler how long a reference to a value is valid. A lifetime is denoted by a symbol, such as 'a, 'b, etc.

Lifetime annotations are used to specify the lifetime of a reference. They are denoted by a symbol, such as 'a, 'b, etc., followed by a colon and the type of the reference.

Example 1: Infered lifetime

```rust
// Lifetimes of r is 'a and x is 'b
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```
Note: This code won't compile since r has a lifetime that outlives x

Example 2: Explicit lifetime annotation

```rust
let x: &'a i32 = &10;
```
In this example, the lifetime of the x, which stores the reference to an i32 value, is specified as `'a`.

---
## Lifetime Annotations in Function Signatures

Example 3: 

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This code will not work till we use an explicit lifetime annotation.

```bash
cargo build
   Compiling test2 v0.1.0 (/Users/bbc/tmp/test2)
error[E0106]: missing lifetime specifier
  --> src/main.rs:40:33
   |
40 | fn longest(x: &str, y: &str) -> &str {
   |               ----     ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
   |
40 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `test2` (bin "test2") due to 1 previous error
```

To fix this we add the lifetime annotations, and it now works fine.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```


To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as we did with generic type parameters.

---

## Lifetime Elation Rules

1. The compiler assigns a lifetime parameter to each parameter that is a reference.

2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameter.

3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

---
## References
- [The Rust Book | Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)