// equation °C = (°F - 32) × 5/9;
use std::io;

const FAHRENHEIT_TO_CELSIUS_FACTOR: f32 = 32.0;
const FAHRENHEIT_TO_CELSIUS_CONSTANT: f32 = 5.0 / 9.0;

fn main() {
    println!("\n\n🌡️ fahrenheit to celsius 🌡");

    let mut fahrenheit = String::new();
    println!("\nEnter fahrenheit:");
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Error: can't get input");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("fahrenheit:{fahrenheit}\ncelsius:{celsius}");
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - FAHRENHEIT_TO_CELSIUS_FACTOR) * FAHRENHEIT_TO_CELSIUS_CONSTANT
}
