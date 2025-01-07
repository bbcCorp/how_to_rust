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
4. **Explain the `Rule of Three` that defines owndership **

Scope is the range within a program for which an item is valid.
1. Every value is owned
2. One owner at a time
3. When the owner leaves scope, what is owned is dropped.

----

5. **Difference between an array and Vec type** 

Here are the key differences between arrays and vectors in Rust:
    - Size: Arrays have a fixed size, while vectors have a dynamic size.
    - Memory allocation: Arrays are stack-allocated, while vectors are heap-allocated.
    - Resizing: Arrays cannot be resized dynamically, while vectors can be resized dynamically.
    - Performance: Arrays are generally faster and more efficient than vectors, since they are stored on the stack and do not require dynamic memory allocation.

Arrays are suitable for small, fixed-size collections, while vectors are suitable for large, dynamic collections.

----

6. **When to Use Vec or LinkedList in Rust**

In Rust, `Vec` and `LinkedList` are two different data structures that can be used to store collections of values. While they share some similarities, they have distinct differences in terms of their characteristics, usage, and behavior.

**Vec**

`Vec` is a dynamic, growable array that is stored on the heap. It is a contiguous block of memory that can grow or shrink dynamically as elements are added or removed.

**LinkedList**

`LinkedList` is a doubly-linked list that is stored on the heap. It is a sequence of nodes that are linked together, where each node points to the next node in the sequence.

**When to Use Vec**

Here are some scenarios where you should use `Vec`:

* **Random access**: When you need to access elements at arbitrary indices, `Vec` is a better choice because it provides constant-time access to elements.
* **Cache-friendly**: When you need to iterate over a large collection of elements, `Vec` is a better choice because it is stored in contiguous memory, which makes it cache-friendly.
* **Insertion and deletion at the end**: When you need to frequently insert or delete elements at the end of the collection, `Vec` is a better choice because it can grow or shrink dynamically.
* **Memory efficiency**: When you need to store a large collection of elements and memory efficiency is a concern, `Vec` is a better choice because it uses less memory than `LinkedList`.

**When to Use LinkedList**

Here are some scenarios where you should use `LinkedList`:

* **Frequent insertion and deletion at arbitrary positions**: When you need to frequently insert or delete elements at arbitrary positions in the collection, `LinkedList` is a better choice because it can do so in constant time.
* **Preserving order**: When you need to preserve the order of elements in the collection, `LinkedList` is a better choice because it maintains the order of elements even after insertion or deletion.
* **Iterating over a large collection**: When you need to iterate over a large collection of elements and you don't need random access, `LinkedList` is a better choice because it is more memory-efficient than `Vec`.

**In General**

In general, if you need to store a collection of elements and you don't need to frequently insert or delete elements at arbitrary positions, `Vec` is a better choice. However, if you need to frequently insert or delete elements at arbitrary positions, `LinkedList` is a better choice.

Here's a rough guideline to help you decide between `Vec` and `LinkedList`:

* Use `Vec` when:
	+ You need to store a large collection of elements and you don't need to frequently insert or delete elements at arbitrary positions.
	+ You need to access elements at arbitrary indices.
	+ You need to iterate over a large collection of elements and you don't need to preserve the order of elements.
* Use `LinkedList` when:
	+ You need to frequently insert or delete elements at arbitrary positions in the collection.
	+ You need to preserve the order of elements in the collection.
	+ You need to iterate over a large collection of elements and you don't need random access.

Listen to [Bjarne Stroustrup on why you should avoid Linked Lists](https://www.youtube.com/watch?v=YQs6IC-vgmo). Always prefer Vec over LinkedList for performace reason. Compactness and predictability trumps random access in the real world.

---

6. **Crates vs Packages**

Crates are a grouping of module that produces either a library or an executable. There are two types of crates: 
1. Library crate - A crate that contains code meant to be consumed by other Rust projects
2. Binary crates - A crate that is compiled to an executable

Packages are used by Cargo, the Rust package manager, to help handle and share crates. Packages are defined by a manifest file called `Cargo.toml`

A crate is a single unit of compilation, while a package is a collection of one or more crates that are published together. A package can contain multiple crates, and each crate can have its own dependencies and versioning information.

Packages can contain any number of binary crates but only 1 library crate.

---

