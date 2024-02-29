/*

    how to use match statements in rust with example

*/

fn main() {
    // defining a varible named day
    let day = 3;

    match day {
        1   =>  println!("Monday"),
        2   =>  println!("Tuesday"),
        3   =>  println!("Wednesday"),
        4   =>  println!("Thursday"),
        5   =>  println!("Friday"),
        6   =>  println!("Saturday"),
        7   =>  println!("Sunday"),
        _   =>  println!("Invalid Day"),   // this will happen when there is no match
    }
}