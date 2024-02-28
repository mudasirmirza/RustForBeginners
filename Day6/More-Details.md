## Ownership in Rust

### Unique Ownership
* In Rust, every value has a single owner at any given time.
* When a variable goes out of scope, Rust automatically calls the drop function for the value, freeing its memory.
### Move Semantics
* Ownership in Rust follows move semantics. When a value is assigned to another variable or passed to a function, ownership of the value is transferred, and the original variable can no longer be used.
* This prevents issues like dangling pointers or double free errors, common in languages with manual memory management.

### Example of Ownership
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of s1 moves to s2
    // println!("s1: {}", s1); // Error: s1 is no longer valid
}
```

## Borrowing in Rust

### Borrowing
* Borrowing allows you to temporarily loan a reference to a value without transferring ownership.
* References in Rust are immutable by default, meaning you can't modify the value through a reference unless it's mutable.

### Types of References
* Immutable References `&T`: Allow reading the value but not modifying it.
* Mutable References `&mut T`: Allow both reading and modifying the value, but only one mutable reference is allowed at a time to prevent data races.

### Example of Borrowing
```rust
fn main() {
    let s = String::from("world");
    let len = calculate_length(&s); // Immutable reference to s
    println!("Length of '{}': {}", s, len);

    let mut s = String::from("hello");
    change_string(&mut s); // Mutable reference to s
    println!("Changed string: {}", s);
}
```
### Benefits of Ownership and Borrowing
* Prevents common bugs like null pointer dereferences, use-after-free errors, and data races.
* Enables concurrency and parallelism by enforcing strict rules at compile time.
* Eliminates the need for garbage collection, leading to predictable performance without runtime overhead.

---

*Understanding ownership and borrowing is fundamental in Rust programming and contributes to writing safe and efficient code. These concepts are critical when working with complex data structures and managing memory effectively.*