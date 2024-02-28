/*

    Example of how ownership works in rust

*/

fn main() {
    // define string
    let s1 = String::from("Hello");
    let s2 = s1;

    // below line will work
    println!("String s2: {}", s2)

    /*
        below line will raise compilation error
        as s1 string is not there anymore as its
        ownership has been transferred to s2.
        Below is detailed error

                error[E0382]: borrow of moved value: `s1`
        --> ownership.rs:12:46
        |
        5  |     let s1 = String::from("Hello");
        |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        6  |     let s2 = s1;
        |              -- value moved here
        ...
        12 |     println!("String1: {} and String 2: {}", s1, s2);
        |                                              ^^ value borrowed here after move
        |
        = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
        |
        6  |     let s2 = s1.clone();
        |                ++++++++

        error: aborting due to 1 previous error

        For more information about this error, try `rustc --explain E0382`.
    */

    // println!("String1: {} and String 2: {}", s1, s2);
}