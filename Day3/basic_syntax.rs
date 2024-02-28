// This is a single line comment

/*

    This is a multi-line comment
    We're demonstrating basic syntax features in rust

*/

fn main() {
    // Declaring variables
    let name = "Mudasir";   // Immutable string
    let mut age = 30;       // Mutable integer

    // Printing variables
    println!("Name: {}", name);
    println!("Age: {}", age);

    // Updating mutable variable
    age = 36;
    println!("Updated age: {}", age);
}