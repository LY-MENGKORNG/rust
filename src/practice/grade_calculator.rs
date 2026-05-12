// FIX: Grade Calculator
//
// Convert a score into a letter grade.
//
// TODO: Instructions
//
// - Read a score from input (0 to 100).
// - Print the letter grade:
//      - 90 or more: A
//      - 80 to 89: B
//      - 70 to 79: C
//      - 60 to 69: D
//      - Below 60: F

use std::io;

pub fn grade_calcaltor() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let score: i32 = input.trim().parse().unwrap();

    match score {
        ..60 => println!("F"),
        60..=69 => println!("D"),
        70..=79 => println!("C"),
        80..=89 => println!("B"),
        90.. => println!("A"),
    }
}
