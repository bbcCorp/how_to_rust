# Ownership

Ownership is a set of rules that govern how a Rust program manages memory. In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks. 

## Ownership Rules
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

If any of the rules are violated, the program won’t compile. 

Example:
```rust
fn main() {

    let a = 5;
    let b = a;  
    // b is a copy of a. Both a and b are on the stack and are independent of each other.
    // Hence, we can use both a and b in the program.

    let s1 = String::from("hello");
    let s2 = s1;
    // s2 is a copy of s1. But s1 is not valid anymore.
    // ownership of "hello" has now moved from s1 to s2.
    
    // Unlike Java/Python where s2 becomes yet another reference to the string object referenced by s1, 
    // in Rust string referencing/copy does not happen automatically. 
    // Copying involves memory allocation and copying the characters. 
    // So we either do explicitly cloning or pass a reference which is the preferred option in most cases


    // allocate a new string to store the input
    // String is a struct that is growable, UTF-8 encoded, and heap allocated
    // string is owned by the "input" variable.
    let mut input = String::new();
    println!("Enter your name: ");

    // We need to use &mut string to pass a mutable reference to the string
    io::stdin().read_line(&mut input).unwrap();
    
    println!("Hello {}", input);

    // the input variable goes out of scope here and the memory used by input is freed
    // much like a smart pointer
    // The rust compiler will automatically call the drop function on the "input" variable
    // to free the memory once it's owner goes out of scope.
}
```

RAII (Resource Acquisition Is Initialization) is a pattern in Rust that's used to manage resources. The basic idea is that a resource is tied to a variable, so that when the variable goes out of scope, the resource is freed. For example, when a String goes out of scope, its destructor is called and its memory is freed. This is similar to how smart pointers work in C++.

```rust
fn print_str(a: String){
    println!("String: {}",a);
}

fn main(){
    let x: String = String::from("Test");

    print_str(x);   // this will transfer the ownership of x to the function's a
    // once the function get executed and a goes out of scope, the value represented by x
    // will be dropped

    // Hence another call like this will result in error
    print_str(x);
}
```

The last line will result in an error: `use of moved value: x` which illustrates how borrow checker keeps track of ownership and lifetime of non-primitive data.

For primitive types like int/chat/float/array/tuple we will not see this behaviour. FOr these types since size is know at compile time, theit values are stored on the stack. They all implement the `Copy triat`, so everytime there is an assignment like `y=x` there is an implicit copy. 

Rust does not do that for non-primitive types like String, which are stored on a heap.

## References

What if we want to have two pointers to the same variable? This is called aliasing, and it can cause problems if we try to modify the variable through both pointers. Rust prevents this by default, but we can use references to allow aliasing.

We can avoid `double free` error by using `borrowing` and `references`.

References allow you to refer to some value without taking ownership of it. We call having references as function parameters borrowing. 

NOTE: 
1. Reference is just a pointer to the data that we already have.
2. References are immutable by default. We're not allowed to modify something we have a reference to.
3. In the same scope we can have as many immutable references as we want or ONE mutable reference. We cannot use immutable references after we have created a mutable references in the same scope.

Example:
```rust
fn main(){
    let s1 = String::from("hello");

    // pass a reference to the string "hello" to the function.
    // s1 is still the owner of the string "hello"
    let len = calculate_length(&s1);

    // Since no ownership was transferred we continue to use s1. 
    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) ->i32 {
    // s is a reference to a String
    // References are immutable by default
    s.len() as i32
} // Here, s goes out of scope. 
// But because it does not have ownership of what it refers to, nothing happens.
```

## Mutable references

In the same scope we can ONLY have ONE mutable reference.

```rust
use std::io;

fn main(){
    let mut input = String::new();

    // read_line takes a mutable reference to the string
    io::stdin().read_line(&mut input).unwrap();

    // input is still the owner of the string and regains control after the function call
    println!("Hello {}", input);
}
```

---------------

## References

- [Double Free Error](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#double-free-error)
