// FIX: Voting Age
//
// Determine if a person is old enough to vote.
//
// TODO: Instructions
//
// - Read a name and an age from input.
// - If the person is 18 or older, print:
//      - [name] can vote
// - If the person is younger than 18, print:
//      - [name] cannot vote

use std::io;

pub fn voting_age() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let name = input.trim().to_string();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let age: i32 = input.trim().parse().unwrap();

    println!(
        "{} {} vote\n",
        name,
        if age >= 18 { "can" } else { "cannot" }
    );
}
