mod area_calculation;
mod celsius_to_fahrenheit;
mod even_or_odd;
mod factorial;
mod grade_calculator;
mod make_username;
mod multiplication_table;
mod rectangle_border;
mod shopping_receipt;
mod sum_digits;
mod sum_numbers;
mod voting_age;

pub fn practices() {
    println!("\n--- [sum digits] ---\n");
    sum_digits::sum_digits();
    println!("\n--- [sum numbers] ---\n");
    sum_numbers::sum_numbers();
    println!("\n--- [celsius to fahrenheit] ---\n");
    celsius_to_fahrenheit::celsious_to_fahrenheit();
    println!("\n--- [even or odd] ---\n");
    even_or_odd::even_or_odd();
    println!("\n--- [make username] ---\n");
    make_username::make_username();
    println!("\n--- [voting age] ---\n");
    voting_age::voting_age();
    println!("\n--- [grade calculator] ---\n");
    grade_calculator::grade_calcaltor();
    println!("\n--- [multiplication table] ---\n");
    multiplication_table::multiplication_table();
    println!("\n--- [factorial] ---\n");
    factorial::factorial();
    println!("\n--- [area calculation] ---\n");
    area_calculation::area_calculator();
    println!("\n--- [shopping receipt] ---\n");
    shopping_receipt::shopping_receipt();
    println!("\n--- [rectangle border] ---\n");
    rectangle_border::rectangle_border();
}
