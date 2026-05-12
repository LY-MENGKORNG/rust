use std::io;

// FIX: Celsius to Fahrenheit
//
// Convert a temperature from Celsius to Fahrenheit.
//
// TODO: Instructions
//
// - Read a temperature in Celsius from input.
// - Use this formula: fahrenheit = celsius * 9/5 + 32
// - Print the result like this: [celsius] Celsius = [fahrenheit] Fahrenheit

pub fn celsious_to_fahrenheit() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let celsius: i32 = input.trim().parse().unwrap();

    let fahrenheit: i32 = celsius * 9 / 5 + 32;

    println!("{} Celsius = {:.1} Fahrenheit", celsius, fahrenheit as f64);
}
