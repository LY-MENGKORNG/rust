pub fn function() {
    regular_fn();
    with_params(18);

    let result = with_return(3, 6);
    println!("result is: {}", result);
}

fn regular_fn() {
    println!("This is regular function!");
}

fn with_params(age: i32) {
    println!("I am {} years old", age)
}

fn with_return(a: i32, b: i32) -> i32 {
    // return a + b;

    // omit the `return` keyword. just write the value without `semicolon`, it'll  automatically return
    a + b
}
