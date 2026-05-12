// FIX: Factorial
//
// Create a function that calculates the factorial of a number.
//
// TODO: Instructions
//
// - Read a number from input.
// - Create a function that calculates the factorial of that number.
// - The factorial of N means: multiply all numbers from 1 to N together.
// - Example: factorial of 5 = 1 × 2 × 3 × 4 × 5 = 120
// - The factorial of 0 is 1.
// - Print the result like this:
//      - [n]! = [result]

use std::io;

pub fn factorial() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i64 = input.trim().parse().unwrap();

    println!("{}! = {}\n", num, calculate_factorial(num, num));
}

fn calculate_factorial(mut n: i64, mut cache: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return cache;
    }

    n -= 1;
    cache *= n;

    calculate_factorial(n, cache)
}
