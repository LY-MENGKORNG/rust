// FIX: Shopping Receipt
//
// Read item details from input and print a short receipt.
//
// TODO: Instructions
//
// Your program reads three lines:
// - Item name
// - Price (a number like 1.50)
// - Quantity (a whole number like 3)
//
// Print exactly these 4 lines:
// - Item: [item]
// - Price: $[price] (always two decimal places, like 1.50)
// - Quantity: [quantity]
// - Total: $[total] (price times quantity, two decimal places)

use std::io;

pub fn shopping_receipt() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let item = input.trim().to_string();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let price: f64 = input.trim().parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let quantity: i32 = input.trim().parse().unwrap();

    println!("Item: {}", item);
    println!("Price: ${:.2}", price);
    println!("Quantity: {}", quantity);
    println!("Total: ${:.2}\n", price * quantity as f64);
}
