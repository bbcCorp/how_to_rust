# Strings

```rust
fn main() {
    let string = String::from("127.0.0.1:8080");
    
    // ss is a string slice (&str) by borrowing ptr reference within the string 
    let ss = &string[..9];

    /* A note on string: strings in rust are utf-8 so it uses 2 bytes. 
       However not all chars uses only 2 bytes, some use 3-4 bytes.
        If we use (byte) index to slice string, we might cut within string boundaries  
     */
        
    dbg!(&string);
    dbg!(ss);
}
```