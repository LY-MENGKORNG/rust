// FIX: Multiplication Table
//
// Print the multiplication table for a given number.
//
// TODO: Instructions
//
// - Read a number from input.
// - Print its multiplication table from 1 to 10.
// - Each line should look like this:
//      - [number] x [i] = [result]

use std::io;

pub fn multiplication_table() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read of std input!");
    let num: i32 = input.trim().parse().expect("Unable to parse from input!");
    let mut count = 1;

    loop {
        if count > 10 {
            break;
        }
        println!("{} X {} = {}", num, count, num * count);

        count += 1;
    }
}
