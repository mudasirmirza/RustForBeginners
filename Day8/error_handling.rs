/*

    How to do error handling with examples in Rust

*/

// Let's import rust module for file management
use std::fs::File;

fn main() {
    // lets try to open / load a non-existant file

    let my_file = File::open("non-existant-file.txt");

    match my_file {
        Ok(file)    => println!("File openend successfully, {:?}", file),
        Err(error)  => println!("Failed to open file, {:?}", error),
    }
}