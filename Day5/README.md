# Day 5: Functions

### Functions in Rust
Functions in Rust allow you to encapsulate a block of code that can be called multiple times with different inputs.
Functions can have parameters and return values.

### Defining Functions
Functions in Rust are defined using the fn keyword followed by the function name and parameters.
The function body contains the code to be executed when the function is called.

### Parameters and Return Values
Functions can have parameters, which are variables that receive values when the function is called.
Functions can also return values using the return keyword or implicitly returning the last expression in the function body.


### Action
Open your text editor and create a new Rust file named `functions.rs`.

Write the following code to define a function that calculates the sum of two numbers:

```rust
// Define a function to calculate the sum of two numbers
fn add(a: i32, b: i32) -> i32 {
    // Return the sum of the two numbers
    a + b
}

fn main() {
    // Call the add function and store the result in a variable
    let result = add(5, 3);

    // Print the result
    println!("Result of adding 5 and 3: {}", result);
}
```
Save the file.

Open your terminal, navigate to the directory containing `functions.rs`, and compile the program using rustc:

```bash
rustc functions.rs
```
* After successful compilation, run the executable:
```bash
./functions
```
You should see the output printed to the console, displaying the result of adding 5 and 3.

Congratulations! You've learned how to define and call functions in Rust. Day6, we'll explore ownership and borrowing in Rust, which are fundamental concepts for managing memory safely.