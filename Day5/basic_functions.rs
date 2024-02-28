/*

    Basic functions

*/

fn add(a: i32, b: i32) -> i32 {
    // Return  the sum of two integers
    a + b
}

fn main() {
    // call the add function and store the returned value in a variable
    let sum = add(5, 4);

    // print the sum variable
    println!("Result of adding 5 and 4 is: {}", sum);
}