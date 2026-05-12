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
    // if width - 1 <= 0 {
    //     println!("[width] must be greater 1");
    //     return;
    // }

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let height: i32 = input.trim().parse().unwrap();
    // if height - 1 <= 0 {
    //     println!("[height] must be greater 1");
    //     return;
    // }

    let mut text_stars = String::from("*");
    for _ in 1..width {
        text_stars.push('*');
    }

    let space_width_len = width - 2;
    let space_height_len = height - 2;

    let mut row_between = String::from("*");
    for _ in 0..space_width_len {
        row_between.push(' ');
    }
    row_between.push('*');

    println!("{}", text_stars);
    for _ in 0..space_height_len {
        println!("{}", row_between)
    }
    println!("{}", text_stars);
}
