# Option and Result
-----------------------------

Rust has no `null`. Instead, functions that might not produce a value return `Option<T>`, and functions that might fail return `Result<T, E>`. Both are enums, and you must handle the "missing" case before the compiler lets you use the inner value.

## Option

`Option` is an enum with two variants:

```rust
enum Option<T> {
    Some(T),   // there is a value of type T
    None,      // there is no value
}
```

`Some` wraps a value; `None` is the explicit "no value" variant. Because `Option` is a different type from `T`, you cannot accidentally use a missing value as if it were present — the compiler forces you to unpack it first.

### Example: finding a substring

```rust
fn main() {
    let s = String::from("Hello, world");
    let w = String::from("world");

    let result = find_first_occurance(&s, &w);

    if let Some(matched) = result {
        println!("{}", matched);
    }
}

fn find_first_occurance<'a>(s: &'a str, w: &str) -> Option<&'a str> {
    s.find(w).map(|start| &s[start .. start + w.len()])
}
```

---

### Step-by-step breakdown

1. `str::find` returns `Option<usize>`.
   - If `w` is found inside `s`, it returns `Some(start_index)` where `start_index` is the byte offset.
   - If `w` is not found, it returns `None`.
   - There is no `-1` sentinel and no null — the type itself tells you the result might be absent.

2. `.map(|start| ...)` transforms the value inside the `Option`.
   - When `find` returned `Some(7)`, `map` runs the closure with `start = 7` and wraps the result back in `Some`, producing `Some("world")`.
   - When `find` returned `None`, `map` does nothing and passes `None` through untouched.
   - So `find_first_occurance` returns `Option<&str>`: `Some("world")` on success, `None` on failure.

3. The lifetime `<'a>` ties the returned `&str` to `s`.
   - `fn find_first_occurance<'a>(s: &'a str, w: &str) -> Option<&'a str>` says: the returned slice borrows from `s`, not from `w`. The borrow checker then ensures callers can't drop `s` while still holding the returned slice.
   - `w` does not need a lifetime because nothing borrowed from `w` escapes the function.

4. `if let Some(matched) = result` unpacks the `Option`.
   - `Some(matched)` is a pattern. If `result` is `Some("world")`, the pattern matches, the inner value is bound to `matched` (type `&str`), and the block runs.
   - If `result` is `None`, the pattern doesn't match, so `matched` is never created and the block is skipped.
   - This is shorthand for a `match` that only cares about one case:

     ```rust
     match result {
         Some(matched) => println!("{}", matched),
         None => {}
     }
     ```

5. `matched` only exists inside the `if` block and is guaranteed to be a real `&str`.
   - There is no "null pointer you forgot to check" bug possible here, because `None` is a real variant you had to account for (even if just by skipping the block).
   - If you tried to use `result` directly as a `&str`, the compiler would reject it — `Option<&str>` and `&str` are different types.

## Result

`Result` is the same idea, but for operations that can fail and where you care about *why* they failed:

```rust
enum Result<T, E> {
    Ok(T),    // success — holds the value of type T
    Err(E),   // failure — holds an error of type E
}
```

You unpack it the same way — `if let Ok(value) = result { ... }` or a `match` covering both `Ok` and `Err`.

```rust
use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()   // returns Ok(n) or Err(ParseIntError)
}

fn main() {
    let result = parse_number("42");

    match result {
        Ok(n)  => println!("Parsed: {}", n),
        Err(e) => println!("Failed: {}", e),
    }
}
```
---

### Option vs Result

| Type      | Meaning of "missing"    | Use when                                 |
|-----------|-------------------------|------------------------------------------|
| `Option`  | value absent / no result | search, lookup, optional field           |
| `Result`  | operation failed         | I/O, parsing, anything that can error   |

Both force you to handle the absent/failed case at compile time, which is Rust's replacement for null-pointer exceptions and unchecked error propagation.

---

## Common helpers

- `.unwrap()` — extract the value, panic if `None`/`Err`. Fine for quick scripts, risky in production code.
- `.expect("msg")` — like `unwrap` but with a custom panic message.
- `.unwrap_or(default)` — return the value, or `default` if absent.
- `.map(|x| ...)` — transform the inner value, pass `None`/`Err` through.
- `?` operator (in `Result`-returning functions) — early-return on `Err`, unwrap on `Ok`.

---
