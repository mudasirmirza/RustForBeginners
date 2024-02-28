/*

    Example of how to define structs

*/

struct Person {
    name: String,   // A person's name
    age: i32,       // A person's age
}

fn main() {
    // Create a person named "Mudasir" with age "35"
    let person1 = Person {
        name: String::from("Mudasir"),
        age: 35,
    };
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
}