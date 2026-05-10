/**
* INFO: A tuple is a group of values of different types, stored in a single variable.
* Tuples are useful when you want to return or work with multiple values together.
*/
pub fn tuples() {
    // NOTE: Tuples are written using parentheses `()`, with values separated by commas:
    let person = ("John", 30, true);

    access_tuple_values(person);
    unpack_tuple(person);
    returning_tuple_from_function();
}

/**
* INFO: You can access tuple values by using a dot . followed by the index:
*/
fn access_tuple_values(person: (&str, i32, bool)) {
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {} \n", person.2);
}

/**
* INFO: When we create a tuple, we normally assign values to it. This is called "packing" a tuple:
*/
fn unpack_tuple(person: (&str, i32, bool)) {
    let (name, age, is_active) = person;

    println!("name: {}", name);
    println!("age: {}", age);
    println!("is_active: {} \n", is_active);
}

/**
* INFO: Tuples are often used to return multiple values from a function:
*/
fn returning_tuple_from_function() {
    let user = get_user();

    println!(
        "{} is {} years old and he/she is {}",
        user.0,
        user.1,
        if user.2 { "active" } else { "inactive" }
    );

    fn get_user() -> (String, i32, bool) {
        (String::from("Alice"), 22, false)
    }
}
