// NOTE: Create a `Struct`
// You define a struct using the struct keyword and place the fields (variables) inside:
struct Person {
    name: String,
    age: u32,
    can_vote: bool,
}

// WARN: Fields are similar to variables, but they belong to a struct.
// Since they are part of a larger structure (like Person or Car),
// they are called fields in Rust, not regular variables.

/**
* INFO: A `struct` (short for "structure") is a custom data structure that lets you group related values together.
* You can think of a struct like a mini-database for one thing, like a person with a name and age.
*/
pub fn structs() {
    // Create a Person object
    let mut user = Person {
        name: String::from("John"),
        age: 35,
        can_vote: true,
    };

    access_fields(&mut user);
    change_fields(&mut user);

    // NOTE: Why Use Structs?
    //
    // To group related data in a clean way
    // To make your code easier to read and maintain
    // To create real-world examples, like users, books, cars, etc.
}

/**
* INFO: you can access the fields of the struct using dot syntax (.):
*/
fn access_fields(user: &mut Person) {
    // Access and print the values
    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Can vote? {}\n", user.can_vote);
}

/**
* INFO: To change a value inside a struct, you must make the struct object mutable by using mut:
*/
fn change_fields(user: &mut Person) {
    user.name = String::from("Alex");
    user.age = 23;
    user.can_vote = false;

    access_fields(user);
}
