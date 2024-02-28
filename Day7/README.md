# Day 7: Structs and Enums


## Structs in Rust
Think of structs like containers that hold different pieces of information together.

```rust
// Define a struct representing a person
struct Person {
    name: String,  // A person's name
    age: u32,      // A person's age
}

fn main() {
    // Create a person named Alice who is 30 years old
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Access and print the person's information
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
}
```
#### In this example:

We define a struct named Person with two pieces of information: name and age.
We create a person named Alice with the specified name and age.
We access and print Alice's name and age.


## Enums in Rust
Enums are like a list of options you can choose from.

```rust
// Define an enum representing different states of a web request
enum Status {
    Ok,                   // Request successful
    NotFound,             // Resource not found
    BadRequest,           // Bad request
    InternalServerError, // Internal server error
}

fn main() {
    // Let's say the request was successful
    let status = Status::Ok;

    // Print a message based on the status
    match status {
        Status::Ok => println!("Request successful"),
        Status::NotFound => println!("Resource not found"),
        Status::BadRequest => println!("Bad request"),
        Status::InternalServerError => println!("Internal server error"),
    }
}
```

#### In this example:

We define an enum named Status with different options representing the state of a web request.
We set the status to Ok.
We print a message based on the status using a match statement.


### Action
Open your text editor and create a new Rust file named `easy_structs_enums.rs`.

Copy and paste the simplified examples of structs and enums provided above into `easy_structs_enums.rs`.

Save the file.

Open your terminal, navigate to the directory containing `easy_structs_enums.rs`, and compile the program using rustc:

```bash
rustc easy_structs_enums.rs
```
After successful compilation, run the executable:

```bash
./easy_structs_enums
```
You should see the output printed to the console, displaying the details of the person and the status of the web request.