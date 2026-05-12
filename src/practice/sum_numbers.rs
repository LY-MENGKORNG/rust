// FIX: Sum of Numbers
//
// Read a list of numbers and calculate their sum.
// TODO: Instructions
//
// - The first line of input is a count (how many numbers will follow).
// - The next lines each have one number.
// - Add all the numbers together and print the sum:
//      - Sum: [total]

use std::io;

pub fn sum_numbers() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();
    let mut sum: i64 = 0;

    for _ in 0..num {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let n: i32 = input.trim().parse().unwrap();

        sum += n as i64;
    }

    println!("Sum: {}", sum);
}
