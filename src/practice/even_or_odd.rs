// FIX: Even or Odd
//
// Check if a number is even or odd.
//
// TODO: Instructions
//
// - Read a number from input.
// - Print "Even" if the number is divisible by 2.
// - Print "Odd" if it is not.

use std::io;

pub fn even_or_odd() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();

    match num % 2 {
        0 => println!("Even"),
        _ => println!("Odd"),
    }
}
