/**
* INFO: An `array` in Rust is a fixed-size list of values, all of the same type.
* You cannot grow or shrink an array after it's created.
* To access an array element, refer to its index number.
* Array indexes start with 0: [0] is the first element, [1] is the second element, etc.
*/
pub fn arrays() {
    create_array();
}

/**
* You can create an array using square brackets `[ ]`, and separate the values with commas.
*
* NOTE: Make sure all values are of the same data type (integers in the example below):
*/
fn create_array() {
    let mut numbers = [1, 2, 3, 4, 5];
    let fruits = ["🍇", "🍌", "🍊"];

    access_elements(&numbers);
    change_values(&mut numbers);
    loop_through_array(&fruits);
}

/**
* NOTE: To access an array element, refer to its index number.
* Array indexes start with 0: [0] is the first element. [1] is the second element, etc.
* This statement accesses the value of the first element [0] in numbers:
*/
fn access_elements(arr: &[i32; 5]) {
    println!("The last element is: {}", arr[arr.len() - 1])
}

/**
* NOTE: To change the value of a specified element, refer to the index number and assign a new value.
* Remember to make the array mutable (using the mut keyword):
*/
fn change_values(arr: &mut [i32; 5]) {
    arr[0] = 10;
    // WARN: When printing the whole array, you must use `{:?}` inside `println!`:
    println!("The new value is: {:?}", arr);
}

/**
* NOTE: You can loop through the array elements with the `for` loop.
*/
fn loop_through_array(fruits: &[&str; 3]) {
    for &fruit in fruits {
        println!("I like: {}", fruit);
    }
}
