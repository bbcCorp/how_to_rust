# Datatypes in Rust

## Static Datatypes
Rust has 4 main datatypes:
- Boolean
- Integer
- Float
- Character

### Integer 

Represented as:
- u : Unsigned integer
- i : Signed integer

Basic Integer Types:
- u8, i8
- u16, i16
- u32, i32
- u64, i64
- u128, i128
- usize, isize : Architecture dependent types. 32 bit on 32 bit architecture and 64 bit on 64 bit architecture.

```rust
let i: i32 = 12345;
```

### Float (f)

Basic Float Types:
- f32
- f64


```rust
let pi: f64 = 3.14159;
```

### Boolean (bool)

Single byte in size. Can be either true or false.

```rust
let flag: bool = false;
```

### Character (char)

4 bytes in size. Unicode Scalar Value.

----

## Compound Types
Can be used to store multiple values together. Both of these have fixed lengths
- Tuple 
- Array

### Tuple

```rust
let t: [i32,i64,u8] = (100,2.5,3);
```


### Array
```rust
let a: [i32,5] = [1,2,3,4,5];
```
----

Note:
1. For 'primitive' types like numbers, since they are just values; they are allowed to be copyable because they are cheap to copy.