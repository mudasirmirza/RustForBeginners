# Day 4: Control Flow

### Control Flow in Rust
Control flow allows you to change the execution path of your program based on conditions or loops.

Rust supports if expressions for conditional branching, loops (loop, while, for), and match expressions for pattern matching.

### if Statements
if statements in Rust are similar to those in other languages. They evaluate a condition and execute a block of code if the condition is true.

### Loops
Rust provides several loop constructs: loop, while, and for.

* loop: Executes a block of code repeatedly until explicitly stopped.
* while: Executes a block of code while a condition is true.
* for: Iterates over elements of a collection.

### Action
Open your text editor and create a new Rust file named `control_flow.rs`.

Write the following code to demonstrate if statements and loops in Rust:

```rust
fn main() {
    // If statement
    let number = 7;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // While loop
    let mut count = 0;
    while count < 5 {
        println!("While loop: {}", count);
        count += 1;
    }

    // For loop
    for i in 0..5 {
        println!("For loop: {}", i);
    }
}
```
Save the file.

Open your terminal, navigate to the directory containing control_flow.rs, and compile the program using rustc:

```bash
rustc control_flow.rs
```
* After successful compilation, run the executable:

```bash
./control_flow
```
You should see the output printed to the console, demonstrating the use of if statements, while loops, and for loops in Rust.

This concludes Day 4. You've learned about control flow constructs in Rust. Day5, we'll dive deeper into functions in Rust.