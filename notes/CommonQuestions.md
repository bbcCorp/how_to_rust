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

7. **Explain `Result` type in Rust**

In Rust, the Result type is a built-in type that represents a value that may or may not be present. It is a way to handle errors and exceptions in a more explicit and safe way.

A Result is a type that can have one of two possible values:
- Ok(value): This represents a successful computation that produced a value.
- Err(error): This represents a failed computation that produced an error.

Example: In this example, the divide function returns a Result that is either Ok with the result of the division, or Err with an error message if the divisor is zero.

```rust
fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(x / y)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => panic!("Error: {}", error),
    }
}
```

In Rust, the ? operator is used to propagate errors up the call stack. It is a shorthand way to handle errors in a more concise way.

Example: In this example, the calculate function calls the divide function and uses the ? operator to propagate any errors that may occur. If the divide function returns an error, the ? operator will return that error from the calculate function.

```rust
fn calculate(x: i32, y: i32) -> Result<i32, &'static str> {
    let result = divide(x, y)?; // ? unwraps the value or propagates the error
    Ok(result * 2)
}
```

----
8. ***Result vs Option***

Result and Option are both used to handle cases where a value may or may not be present. However, they have different use cases:
- Option: Use Option when the absence of a value is a valid and expected outcome.
- Result: Use Result when the absence of a value is an error or an unexpected outcome.

---
9. ***How to write to a HashMap only if key doesn't exist***

```rust
// insert a key only if it doesn't already exist
player_stats.entry("my_key").or_insert(100);
```

---

10. ***Explain: "Do not communicate by sharing memory but share memory by communicating" ***

In many programming languages, threads often communicate with each other by sharing mutable state, such as variables or data structures, through shared memory. This can lead to synchronization issues, data races, and other concurrency-related problems.

Rust takes a different approach. Instead of sharing memory directly, Rust encourages you to use channels, message passing, and other forms of communication to exchange data between threads. This approach is often referred to as "shared memory by communicating" or "message passing."

In Rust, you can use libraries like std::sync::mpsc (multi-producer, single-consumer) or crossbeam to create channels that allow threads to communicate with each other. By using these channels, you can share data between threads without exposing shared mutable state. This is especially useful when data is created in one thread and consumed in another.

By using channels and message passing, Rust encourages us to write concurrent code that is safer, more efficient, and easier to reason about.

Example:
```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let message = "Hello, world!";
        tx.send(message).unwrap();
    });

    let received_message = rx.recv().unwrap();
    println!("{}", received_message);
}
```

---

11. *** Mutexes in Rust ***

Rust provides mutexes which can be used by multiple threads to access/modify shared data. 

The mutex can be created via a new constructor. Each mutex has a type parameter which represents the data that it is protecting. The data can only be accessed through the RAII guards returned from lock and try_lock, which guarantees that the data is only ever accessed when the mutex is locked.

Mutexes protect shared data from concurrent access and helps prevent data races. Mutex will block threads waiting for the lock to become available. 

To use a mutex, we must 
- Acquire a lock before using the data
- Release the lock when finished. 

Locks are automatically released when a MutexGuard does out of scope. To unlock a mutex guard sooner than the end of the enclosing scope, either create an inner scope or drop the guard manually.

Example:
```rust
fn simple_mutex_demo(){
    let simple_mutex = Mutex::new(10);

    // Display the mutex
    println!("{:?}", simple_mutex);

    // Display the data inside the mutex
    let num1 = simple_mutex.lock().unwrap();
    println!("Data inside mutex: {:?}", num1);

    // We drop the mutex guard to if release the lock explicitly 
    // Without this the program will get stuck next time we try to acquire a lock
    drop(num1);

    // we will create a new scope to represent a thread context
    {
        // acquire the lock 
        let mut num = simple_mutex.lock().unwrap(); 

        // modify the data 
        *num = 20;

    } // lock is automatically released here

    // Display the mutex
    println!("{:?}", simple_mutex);

    let num2 = simple_mutex.lock().unwrap();
    println!("Data inside mutex: {:?}", num2);
    drop(num2);
}
```

Management of mutexes can be tricky, and can result in deadlocks, so it is better to use channels wherever possible.

---

12. *** What is the role of Atomic Reference Count (ARC) while using Mutex ***

In Rust, Arc stands for "Atomically Reference Counted". It's a type of smart pointer that allows multiple owners to share the same value, while ensuring that the value is properly cleaned up when it's no longer needed.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();    // acquire the lock here
            *num += 1;
        }   // lock is automatically released here
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
```

This Arc struct, via the Clone implementation can create a reference pointer for the location of a value in the memory heap while increasing the reference counter. As it shares ownership between threads, when the last reference pointer to a value is out of scope, the variable is dropped.

Wrapping a Mutex with an Arc is a common pattern in Rust that is used to can share a Mutex instance with multiple threads or parts of our program. Mutex allows safe mutability where as Arc allows us to safely share a value between multiple threads.

Read more [about why we wrap Mutex using Arc in this article](https://itsallaboutthebit.com/arc-mutex/)
---

---

