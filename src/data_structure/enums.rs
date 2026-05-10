/* INFO: Create an Enum
* To create an enum, use the `enum` keyword and add a set of named values (variants) separated by commas:
*/
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum LoginStatus {
    Success(String),
    Error(String),
}

/**
* INFO: An enum (short for "enumeration") is a way to define a type that can be one of a few different values.
* Each value in the enum is called a variant.
* Enums are useful when you want to represent a value that can only be one of a set of options - like days of the week, directions, or results like success and error.
*/
pub fn enums() {
    match_enum_values();
    enum_with_data();

    // WARN: Why Use Enums?
    //
    // To group related values into one type
    // To make your code more readable and safe
    // To handle different cases with `match`
}

/**
* INFO: Enums work great with the match statement. You can run different code depending on which variant is used:
*/
fn match_enum_values() {
    let direction = Direction::Left;

    fn print_direction(d: &str) {
        println!("{}\n", d);
    }

    match direction {
        Direction::Up => print_direction("Up"),
        Direction::Left => print_direction("Left"),
        Direction::Right => print_direction("Right"),
        Direction::Down => print_direction("Down"),
    }
}

/*
* INFO: Enum variants can also hold data. This is useful when each variant needs to store extra information:
* [LoginStatus](LoginStatus)
*/
fn enum_with_data() {
    let result1 = LoginStatus::Success(String::from("Welcome!"));
    let result2 = LoginStatus::Error(String::from("Invalid Password!"));

    fn check_login_status_helper(result: &LoginStatus) {
        match result {
            LoginStatus::Success(msg) => println!("Success: {}\n", msg),
            LoginStatus::Error(msg) => println!("Fail: {}\n", msg),
        }
    }

    check_login_status_helper(&result1);
    check_login_status_helper(&result2);
}
