# Day 6: Ownership and Borrowing

### Ownership in Rust
* Ownership is one of Rust's unique features for managing memory safely without a garbage collector.
* Every value in Rust has a variable that's its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value is dropped.

### Borrowing
* Borrowing allows you to temporarily loan a reference to a value without taking ownership.
* Borrowing can be `immutable` (shared) or `mutable` (exclusive).

### Action
Open your text editor and create a new Rust file named ownership_borrowing.rs.

Write the following code to demonstrate ownership and borrowing in Rust:

```rust
fn main() {
    // Ownership example
    let s1 = String::from("hello");
    let s2 = s1; // Move ownership from s1 to s2
    // println!("s1: {}", s1); // This line will cause a compilation error because s1 is no longer valid

    // Borrowing example
    let s3 = String::from("world");
    let len = calculate_length(&s3); // Pass a reference to s3
    println!("Length of '{}': {}", s3, len);

    // Mutable borrowing example
    let mut s4 = String::from("hello");
    change_string(&mut s4); // Pass a mutable reference to s4
    println!("Changed string: {}", s4);
}

// Function to calculate the length of a string without taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function to change a string using a mutable reference
fn change_string(s: &mut String) {
    s.push_str(", world!");
}
```
Save the file.

Open your terminal, navigate to the directory containing ownership_borrowing.rs, and compile the program using rustc:

```bash
rustc ownership_borrowing.rs
```
* After successful compilation, run the executable:
```bash
./ownership_borrowing
```
You should see the output printed to the console, demonstrating ownership and borrowing concepts in Rust.

Congratulations! You've learned about ownership and borrowing in Rust. Day7, we'll explore structs and enums, which are essential for defining complex data structres.