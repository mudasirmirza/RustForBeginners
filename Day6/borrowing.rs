/*
    Example of borrowing
*/

fn main() {
    // immutable variable
    let s1 = String::from("Hello");

    let len = calculate_len(&s1); // immutable borrowing
    println!("String1: {} and Length of String1: {}", s1, len);

    // mutable variable
    let mut s2 = String::from("World");
    println!("String 2 is {}", s2);
    change_string(&mut s2); // mutable borrowing
    println!("Changes string 2 is {}", s2);
}

fn calculate_len(a: &String) -> usize {
    a.len()
}

fn change_string(a: &mut String) {
    a.push_str(" , Hello!");
}