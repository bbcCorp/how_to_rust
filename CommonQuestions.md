# Common Rust Questions

1. **What is the difference between `const` and `mut`**
Constants(`const`) values are evaluated at compile-time, while immutable values(`mut`) are evaluated at runtime.

---

2. **What is the `Option` type**

In Rust, the `Option` type is a way to represent a value that may or may not be present. It's a type that can have one of two possible values:

* `Some(value)`: represents a value that is present.
* `None`: represents the absence of a value.

Think of `Option` as a container that may or may not hold a value. When you use `Option`, you're explicitly acknowledging that the value might not be there, and you need to handle that possibility.


`Option` is useful when:

* A function might not return a value (e.g., a search function that might not find a match).
* A value is not always available (e.g., a user's profile picture might not be set).
* You want to avoid null pointer dereferences ( Rust doesn't have null, but `Option` serves a similar purpose).

**Creating Options**

You can create an `Option` using the `Some` and `None` constructors:

```rust
let x = Some(5); // x is an Option<i32> with value 5
let y = None; // y is an Option<i32> with no value
```

You can use pattern matching to handle `Option` values:

```rust
match x {
    Some(value) => println!("Value is: {}", value),
    None => println!("No value"),
}
```
`Option` has several useful methods:

* `unwrap()`: returns the value inside `Some`, or panics if it's `None`.
* `expect()`: returns the value inside `Some`, or panics with a custom message if it's `None`.
* `is_some()`: returns `true` if the `Option` is `Some`, `false` otherwise.
* `is_none()`: returns `true` if the `Option` is `None`, `false` otherwise.
* `map()`: applies a function to the value inside `Some`, returning a new `Option`.
* `and_then()`: applies a function to the value inside `Some`, returning a new `Option`.

Example:
```rust
let x = Some(5);

// Using unwrap()
let value = x.unwrap(); // value is now 5

// Using expect()
let value = x.expect("Value not found"); // value is now 5

// Using pattern matching
match x {
    Some(value) => println!("Value is: {}", value),
    None => println!("No value"),
}

// Using is_some() and is_none()
if x.is_some() {
    println!("Value is present");
} else {
    println!("Value is not present");
}
```
In summary, `Option` is a powerful type in Rust that helps you handle values that may or may not be present. By using `Option`, you can write more robust and safe code that avoids null pointer dereferences and other common errors.

---

3. **Explain `Result` type**

In Rust, the `Result` type is a way to handle errors and exceptions in a explicit and safe way. It's a type that can have one of two possible values:

* `Ok(value)`: represents a successful operation with a value.
* `Err(error)`: represents an error or exception with an error value.

Think of `Result` as a container that can hold either a successful value or an error value. When you use `Result`, you're explicitly acknowledging that an operation might fail, and you need to handle that possibility.

**Why use Result?**

`Result` is useful when:

* A function might fail and return an error.
* You want to handle errors in a explicit and safe way.
* You want to avoid panicking or crashing the program when an error occurs.

**Creating Results**

You can create a `Result` using the `Ok` and `Err` constructors:

```rust
let x = Ok(5); // x is a Result<i32> with value 5
let y = Err("Error message"); // y is a Result<i32> with error message
```

**Pattern Matching with Results**

You can use pattern matching to handle `Result` values:

```rust
match x {
    Ok(value) => println!("Value is: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

Common Methods:

`Result` has several useful methods:

* `unwrap()`: returns the value inside `Ok`, or panics if it's `Err`.
* `expect()`: returns the value inside `Ok`, or panics with a custom message if it's `Err`.
* `is_ok()`: returns `true` if the `Result` is `Ok`, `false` otherwise.
* `is_err()`: returns `true` if the `Result` is `Err`, `false` otherwise.
* `map()`: applies a function to the value inside `Ok`, returning a new `Result`.
* `and_then()`: applies a function to the value inside `Ok`, returning a new `Result`.

Example:
```rust
let x = Ok(5);

// Using unwrap()
let value = x.unwrap(); // value is now 5

// Using expect()
let value = x.expect("Error message"); // value is now 5

// Using pattern matching
match x {
    Ok(value) => println!("Value is: {}", value),
    Err(error) => println!("Error: {}", error),
}

// Using is_ok() and is_err()
if x.is_ok() {
    println!("Value is present");
} else {
    println!("Error occurred");
}
```
In summary, `Result` is a powerful type in Rust that helps you handle errors and exceptions in a explicit and safe way. By using `Result`, you can write more robust and reliable code that avoids panicking or crashing the program when an error occurs.

---
