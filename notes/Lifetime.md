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

---

## Non-Lexical Lifetimes (NLL) and the Borrow Checker

The borrow checker enforces ownership and borrowing rules at compile time. It tracks the "borrow lifetime" — how long a reference is live — and rejects code where a borrowed reference could outlive the data it points to.

Before NLL (pre-2018), the borrow checker used lexical scopes: a borrow was alive from declaration to the end of the block. With NLL, the borrow checker tracks the actual last USE of a reference, not its lexical scope.

### Example: Borrow checker catches mutable borrow while immutable borrow is live

```rust
fn main() {
    let mut words = String::from("hello world rust");
    let first = first_word(&words);
    words.clear();  // ERROR: mutable borrow while immutable borrow is live
    println!("{}", first);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```

Why this fails:
- `first_word` returns `&str` — a string slice that borrows from the `String` owned by `words`.
- `first` is an immutable borrow of `words` that is still alive when `words.clear()` is called.
- `clear()` takes `&mut self` — it needs a mutable borrow.
- You cannot have a mutable borrow while an immutable borrow exists. That's the violation.

### Fix 1: Use the borrow before the mutable access

```rust
fn main() {
    let mut words = String::from("hello world rust");
    let first = first_word(&words);
    println!("{}", first);  // last use of first — borrow ends here
    words.clear();          // now fine — no live borrows
    println!("{}", words);
}
```

With NLL, the borrow of `words` by `first` ends at the `println!` line because that's the last place `first` is used. By the time `clear()` runs, there are no live borrows.

### Fix 2: Return an owned String instead of a borrow

```rust
fn first_word(s: &String) -> String {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return s[..i].to_string();
        }
    }
    s[..].to_string()
}
```

This compiles because there is no borrow — `first` owns a new heap allocation. But the trade-off is a memory allocation and copy of the substring.

### The fundamental trade-off

The borrow checker forces you to choose:
- Borrow (`&str`, `&[T]`): zero-cost — just a pointer + length into existing data — but ties the return value's lifetime to the input.
- Own (`String`, `Vec<T>`): free of lifetime constraints, but pays with memory and heap allocation.


## Lifetime Elision Rules

1. The compiler assigns a lifetime parameter to each parameter that is a reference.

2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameter.

3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

---

### Example: Multiple input references — which one does the output borrow from?

Consider a function that searches for a word inside a string and returns the matching substring:

```rust
fn first_word_occurance(s: &str, word: &str) -> Option<&str> {
    // ...
}
```

This will not compile. The compiler gives:

```bash
error[E0106]: missing lifetime specifier
```

Why: there are TWO input references and ONE output reference. The borrow checker needs to know — does the output `&str` borrow from `s` or from `word`? It cannot guess.

The lifetime elision rules only auto-resolve when:
- There is exactly one input lifetime (rule 2), or
- One parameter is `&self`/`&mut self` (rule 3)

Neither applies here. So we must annotate explicitly.

### The correct annotation — only tie the lifetime to what the output actually borrows

The result is a substring of `s` — it points into `s`'s buffer, NOT into `word`. So only `s` gets the lifetime tied to the output:

```rust
fn first_word_occurance<'a>(s: &'a str, word: &str) -> Option<&'a str> {
    s.find(word).map(|start| &s[start..start + word.len()])
}
```

This is the pattern the standard library uses — `str::find` returns `Option<usize>`, and slicing with that result gives you an `Option<&str>`.

`word` gets its own anonymous lifetime — it only needs to live for the duration of the function call. The return value does not borrow from `word`, so `word`'s lifetime is irrelevant to the caller.

Key insight: not every input reference needs the same lifetime as the output. Annotate only the ones the output actually borrows from.

This has the same borrow checker constraint as `first_word` — the returned `&str` borrows from `s`, so you cannot mutate `s` while the returned slice is still live. The `Option` wrapper does not change the borrow — `Some(&str)` still holds the immutable borrow of `s` until it is dropped or consumed.

The Option<&'a str> holds an immutable borrow of s for as long as the Option is alive. Even though it's wrapped in Option, the borrow checker still tracks the inner &str — Some(&str) is still a live immutable borrow of s.

So this fails:

```rust
    fn main() {
        let mut s = String::from("hello world");
        let found = first_word_occurance(&s, "world");  // borrows s immutably
        s.clear();  // ERROR: cannot mutably borrow s while found is alive
        if let Some(matched) = found {
            println!("{}", matched);
        }
    }
```

And this works (same NLL principle as before — consume the borrow before mutating):

```rust
    fn main() {
        let mut s = String::from("hello world");
        let found = first_word_occurance(&s, "world");
        if let Some(matched) = found {
            println!("{}", matched);  // last use of found — borrow ends here
        }
        s.clear();  // fine — no live borrows
    }
```

The Option doesn't change anything about the borrow. It just adds the possibility that the result is None. The lifetime 'a still ties the inner &str to s.

---


## References
- [The Rust Book | Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)