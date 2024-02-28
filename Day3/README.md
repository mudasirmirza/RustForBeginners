# Day 3: Basic Syntax and Hello World

## Basic Syntax
Rust syntax is similar to other programming languages but comes with some unique features like ownership, borrowing, and lifetimes.

### Statements and Code Blocks
* In Rust, statements end with a semicolon `;`, and blocks of code are enclosed in curly braces `{}`.

### Comments
* Comments in Rust start with `//` for single-line comments and `/* */` for multi-line comments.

### Variables and Data Types
* Rust is statically typed, which means that variables must have a declared type.
* Variables are immutable by default. You can use the mut keyword to make them mutable.

### Action
Open your text editor and create a new Rust file named basic_syntax.rs.

Write the following code to declare a variable, print its value, and demonstrate basic syntax features:

```rust
// This is a single-line comment

/* 
   This is a multi-line comment
   We're demonstrating basic syntax features in Rust
*/

fn main() {
    // Declaring variables
    let name = "Alice"; // Immutable string
    let mut age = 30;   // Mutable integer
    
    // Printing variables
    println!("Name: {}", name);
    println!("Age: {}", age);
    
    // Updating mutable variable
    age = 31;
    println!("Updated age: {}", age);
}
```
Save the file.

Open your terminal, navigate to the directory containing basic_syntax.rs, and compile the program using rustc:

```bash
rustc basic_syntax.rs
```
* After successful compilation, run the executable:
```bash
./basic_syntax
```
* You should see the output printed to the console, displaying the name and age.

This concludes Day 3. You've learned about basic syntax, variables, comments, and data types in Rust. Day4, we'll explore control flow in Rust, including if statements and loops.