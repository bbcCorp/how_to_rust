# Channels and Concurrency in Rust

Rust's `std::sync::mpsc` provides multi-producer, single-consumer channels for thread communication. "mpsc" = Multiple Producer, Single Consumer.

## Basic Channel Pattern

```rust
use std::sync::mpsc::{channel, Sender, Receiver};

let (tx, rx): (Sender<T>, Receiver<T>) = channel();

// Clone the sender for each producer thread
let tx_clone = tx.clone();
thread::spawn(move || {
    tx_clone.send(data);
});

// Receive in the main thread
for received in rx {
    // process received
}
```

## How Sender::clone() Works Under the Hood

`Sender::clone()` does NOT copy the channel. It clones an internal reference-counted handle to the same shared channel. Under the hood, `mpsc` uses an `Arc` internally:

- `tx.clone()` — creates a new `Sender` pointing to the same channel, increments reference count.
- `Sender` dropped — decrements reference count.
- Channel closes when count reaches zero — i.e., ALL senders are gone.

This is why dropping one sender does not affect other clones. Each clone independently holds the channel open.

## The `move` Closure Pitfall with Channels

When spawning threads in a loop, `move` closures consume the variables they reference. On the first iteration, `tx` is moved into the closure and no longer exists for subsequent iterations.

### Error (E0382: use of moved value)

```rust
for line in reader.lines() {
    if buffer_full {
        thread::spawn(move || {
            let tx_clone = tx.clone();       // ERROR: tx already moved in previous iteration
            tx_clone.send(result);
        });
    }
}
```

### Fix: clone BEFORE the closure

```rust
for line in reader.lines() {
    if buffer_full {
        let tx_clone = tx.clone();          // clone before the move
        let buffer_snapshot = lines_buffer.clone();

        thread::spawn(move || {
            tx_clone.send(result);          // move the clone into the closure
        });
    }
}
```

Rule: if a variable is used inside a `move` closure AND needs to survive across loop iterations, clone it BEFORE the closure. The closure then moves the clone, leaving the original intact.

## The Deadlock Pattern: Forgetting to Drop the Original Sender

The receive loop `for received in rx` blocks until the channel closes. A channel closes when ALL senders are dropped. If the original `tx` stays alive on the main thread, the channel never closes and the receiver blocks forever.

### Deadlock (hangs forever)

```rust
let (tx, rx) = channel();

for line in reader.lines() {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        tx_clone.send(result);
    });
}

// tx is still alive here — channel never closes
for received in rx {
    // blocks forever waiting for more data
}
```

### Fix: drop the original sender before receiving

```rust
let (tx, rx) = channel();

for line in reader.lines() {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        tx_clone.send(result);
    });
}

drop(tx);  // close the last sender on the main thread

for received in rx {
    // now ends when all thread tx_clones are dropped
}
```

## Arc for Shared Read-Only Data Across Threads

When multiple threads need read access to the same data, use `Arc` (Atomically Reference Counted). `Arc::clone()` increments the reference count cheaply — it does NOT copy the underlying data.

```rust
use std::sync::Arc;

let common_words = Arc::new(vec![
    String::from("Gullivers"),
    String::from("Gutenberg"),
]);

// In each thread spawn:
let shared_common_words = Arc::clone(&common_words);  // cheap refcount bump
thread::spawn(move || {
    // shared_common_words is moved into the thread
    // use &*shared_common_words to dereference
});
```

Note: Use `Arc::clone(&common_words)` instead of `common_words.clone()`. The former makes it explicit you're cloning the Arc (cheap), not the data (expensive).

## Handling Result from send()

`Sender::send()` returns a `Result` because it can fail if the receiver has been dropped. Always handle it:

```rust
// Explicitly ignore (acceptable if you don't care about the error)
let _ = tx_clone.send(result);

// Or handle properly
if tx_clone.send(result).is_err() {
    eprintln!("receiver dropped, cannot send");
}
```

Never silently drop the return value — Rust will warn you with `unused_must_use`.

## Key Lessons

1. `move` closures take ownership permanently. Clone before the closure if the original is needed in subsequent iterations.
2. `mpsc` channels only close when ALL senders are dropped. Forgetting to drop the original sender causes the receiver to block forever.
3. `Sender::clone()` uses internal `Arc` — each clone independently keeps the channel open.
4. `Arc::clone()` is cheap (refcount bump), `Vec::clone()` is expensive (full data copy). Know the difference.
5. Every `Result` should be handled or explicitly ignored with `let _`. Never silently drop it.

## References

- [The Rust Book | Message Passing](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [std::sync::mpsc docs](https://doc.rust-lang.org/std/sync/mpsc/index.html)