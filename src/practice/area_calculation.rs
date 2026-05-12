// FIX: Area Calculator
//
// Calculate the area of a rectangle, triangle, or circle.
//
// TODO: Instructions
//
// - Read a shape and its measurements from input.
// - The shape is one of these three words:
//      - `rectangle` - next two lines are width and height
//      - `triangle` - next two lines are base and height
//      - `circle` - next line is the radius
// - Print the area with two decimal places:
//      - Area: [result]
// - Triangle area = base × height / 2. Circle area = pi × radius × radius.

use std::{f32::consts::PI, io};

pub fn area_calculator() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let shape = input.trim();
    let mut area: f32 = 0.00;

    match shape {
        "circle" => {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let radius: i32 = input.trim().parse().unwrap();

            area = (radius * radius) as f32 * PI;
        }
        "triangle" => {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let base: i32 = input.trim().parse().unwrap();

            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let height: i32 = input.trim().parse().unwrap();

            area = (base * height / 2) as f32;
        }
        "rectangle" => {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let height: i32 = input.trim().parse().unwrap();

            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let width: i32 = input.trim().parse().unwrap();

            area = (width * height) as f32;
        }
        _ => {
            println!("Incorrect share!");

            #[warn(clippy::needless_return)]
            return;
        }
    }
    println!("Area: {:.2}", area);
}
