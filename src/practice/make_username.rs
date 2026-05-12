use std::io;

// FIX: Make a Username
//
// Create a username and initials from a first and last name.
//
// TODO: Instructions
//
// - Read a first name and a last name from input.
// - Create a username by joining the two names together in lowercase (no space between them).
//
// TEST: Print these two lines:
//
// - Username: [username]
// - Initials: [first letter of first name][first letter of last name] (in uppercase)

pub fn make_username() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_name = input.trim().to_string();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let last_name = input.trim().to_string();

    fn first_upper(text: &str) -> char {
        text.chars().next().unwrap()
    }

    let username = format!("{}{}", first_name.to_lowercase(), last_name.to_lowercase());
    let initials = format!("{}{}", first_upper(&first_name), first_upper(&last_name),);

    println!("Username: {}", &username);
    println!("Initials: {}", &initials);
}
