/*

    Example of how to define ENUMS in rust

*/

enum Status {
    Ok,                   // Request successful
    NotFound,             // Resource not found
    BadRequest,           // Bad request
    InternalServerError, // Internal server error
}

fn main() {

    let status = Status::Ok;
    // Print a message based on the request reponse
    match status {
        Status::Ok => println!("Request Successful"),
        Status::NotFound => println!("Resource not found"),
        Status::BadRequest => println!("Bad request"),
        Status::InternalServerError => println!("Internal Server Error"),
    }

}