// FIX: Rectangle Border
//
// Print a rectangle border made of stars.
//
// TODO: Instructions
//
// - Read a width and a height from input.
// - Print a rectangle border using `*` characters.
// - The first and last rows are full rows of stars. The rows in between have a star at the start and end, with spaces in the middle.

use std::io;

pub fn rectangle_border() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let width: i32 = input.trim().parse().unwrap();

    input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let height: i32 = input.trim().parse().unwrap();

    let border_x = "*".repeat(width as usize);
    let row_between = format!("*{}*", " ".repeat((width - 2) as usize));

    println!("{}", border_x);
    for _ in 0..height - 2 {
        println!("{}", row_between)
    }
    println!("{}", border_x);
}
