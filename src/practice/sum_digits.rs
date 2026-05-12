use std::io;

// FIX: Sum of Digits
//
// Calculate the sum of all digits in a given number.
//
// TODO: Instructions
//
// - Read a positive integer from input.
// - Calculate and print the sum of all its digits.

pub fn sum_digits() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut n: i32 = input.trim().parse().unwrap();
    let mut result: i32 = 0;

    loop {
        if n == 0 {
            break;
        };

        result += n % 10;
        n /= 10;
    }
    println!("{}", result);
}
