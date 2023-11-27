# Memory

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. 

## Stack

Stack is a special region in the process memory that stores variables created by each function.

The memory for each function is called a `stack frame` and is allocated on top of the current one. This is what gives scope to our function. 

All data stored on the stack must have a known, fixed size. The size of every variable on the stack has to be known at compile time.

When a function exists, it's stack frame is released. That means we do not need to worry about deallocating the memory. It is managed for us.

Hence, stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out (LIFO).

Example:

```rust

fn main() {
    println!("Function call with parameters");
    let a = 2;
    let result = stack_only(a);
    dbg!(result);
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c;
}
```

This is how the stack looks

| Stack |
|-----  |
| stack_only b=2. c=3 |
| main a=2|

Note: Stack has a limited size that is dependent on the machine architecture. 
Once we run out of stack (through something like infinite recursion) we get the StackOverflow error.


A variable is a component of a stack frame, either a named function parameter, an anonymous temporary, or a named local variable.
A local variable (or stack-local allocation) holds a value directly, allocated within the stack's memory. The value is a part of the stack frame.
Local variables are not initialized when allocated. Instead, the entire frame worth of local variables are allocated, on frame-entry, in an uninitialized state.

---------

## The Heap

The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. 

The heap is a region of the process memory that is NOT automatically managed for us. It does not have a size restriction.

We have to manually allocate and then deallocate memory. Failure to do that will cause memory leaks.

It is accessible by any function, anywhere in the program.

Heap allocations are expensive and we should avoid them when possible. 
- Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 

```rust
fn stack_and_heap() -> i32 {
    // Box is a smart pointer that points to a value on the heap
    let e = Box::new(7);
    return 5 + *e;
}

```

----------
## References
-[Stack and Heap](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)
-[Memory allocation and lifetime](https://doc.rust-lang.org/reference/memory-allocation-and-lifetime.html)
