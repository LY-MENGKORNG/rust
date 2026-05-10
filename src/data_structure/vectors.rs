/**
* INFO: A `vector` is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.
*/
pub fn vectors() {
    // INFO: To create a vector, use the `vec!` macro:
    let mut fruits = vec!["🍎", "🍊", "🍌"];

    access_vector_elements(&fruits);
    change_vector_values(&mut fruits);
    add_vector_elements(&mut fruits);
    remove_vector_elements(&mut fruits);
    remove_or_add_vector_specific_elements(&mut fruits);
    loop_vector(&mut fruits);
}

/**
* INFO: You can access values in a vector using index numbers (just like arrays):
*/
fn access_vector_elements(fruits: &[&str]) {
    println!("First fruit: {} \n", fruits[0]);
}

/**
* INFO: To change a value in the vector, refer to the index number and assign a new value.
* Remember to make the vector mutable (using the `mut` keyword):
*/
fn change_vector_values(fruits: &mut [&str]) {
    fruits[fruits.len() - 1] = "🍒";

    println!("Last fruit changed to 🍒 {:?} \n", fruits);
}

/**
* INFO: You can add a new element to the end of a vector using the `push()` method:
*/
fn add_vector_elements(fruits: &mut Vec<&str>) {
    fruits.push("🍇");

    println!("New last fruit 🍇 {:?} \n", fruits)
}

/**
* INFO: To remove the last element from a vector, use `pop()`:
*/
fn remove_vector_elements(fruits: &mut Vec<&str>) {
    fruits.pop();

    println!("Good bye 🍇 {:?} \n", fruits);
}

/**
* INFO: Rust vectors are designed to grow and shrink at the end, but you can also add or remove elements at the beginning or at a specified index.
*/
fn remove_or_add_vector_specific_elements(fruits: &mut Vec<&str>) {
    // NOTE: Use `insert()` to add an item at a specified index:
    fruits.insert(0, "🍇");
    println!("Welcome back 🍇 {:?} \n", fruits);

    // NOTE: Use `remove()` to remove an element from a specified index:
    fruits.remove(1);
    println!("Good bye 🍎 {:?} \n", fruits);
}

/**
* INFO: Just like arrays, you can use a for loop to go through all the values in a vector:
*/
fn loop_vector(fruits: &mut Vec<&str>) {
    for fruit in fruits {
        println!("I like {} \n", fruit);
    }
}
