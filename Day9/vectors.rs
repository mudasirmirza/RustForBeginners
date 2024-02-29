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