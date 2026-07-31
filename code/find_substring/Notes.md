# How it works 

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


