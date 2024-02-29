# Day 9: Vectors and Ownership

## Vectors in Rust
* Vectors are a dynamic array type in Rust that can grow or shrink in size.
* They allow you to store multiple values of the same type in a single data structure.

### Creating Vectors
* You can create a vector using the vec! macro followed by the initial values enclosed in square brackets.

```rust
// Create a vector of integers
let numbers = vec![1, 2, 3, 4, 5];
```

### Accessing Elements
* You can access elements of a vector using indexing, starting from zero.
```rust
// Access the first element of the vector
let first_number = numbers[0];
```

### Ownership and Vectors
* Vectors own their data. When a vector goes out of scope, its elements are dropped.

### Action
* Open your text editor and create a new Rust file named `vectors.rs`.
* Write the following code to demonstrate vectors in Rust:

```rust
/*

    how to use vectors (dynamic arrays) in rust
    with example

*/

fn main() {
    // definind a mutable vector (dynamic array)
    let mut numbers = vec![1, 2, 3, 4, 5];

    // lets add a new element in vector
    // this will add '6' as the last element in the vector
    numbers.push(6);

    // let's print 1st element of the vector
    let first_number = numbers[0];
    let len_numbers = numbers.len();
    println!("First element is, {:?} and length of vector is {:?}", first_number, len_numbers);
}
```
Save the file.

Open your terminal, navigate to the directory containing `vectors.rs`, and compile the program using `rustc`:

```bash
rustc vectors.rs
```

* After successful compilation, run the executable:

```bash
./vectors
```

* You should see the output printed to the console, displaying the vector of numbers and the first element.

Congratulations! You've learned about vectors and ownership in Rust. Day10, we'll explore more advanced topics, including slices and borrowing.