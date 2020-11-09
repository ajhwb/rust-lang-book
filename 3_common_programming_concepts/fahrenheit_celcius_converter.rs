use std::io;

fn main() {

    println!("Enter convertion mode");
    println!("F(ahrenheit) to Celcius OR C(elcius) to Fahrenheit: ");
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    let mode = mode.trim(); // Remove newline, spaces

    if mode == "F" {
        println!("Fahrenheit to Celcius mode");
    } else if mode == "C" {
        println!("Celcius to Fahrenheit mode");
    } else {
        println!("Invalid mode {}", mode);
        return;
    }

    let mut value = String::new();
    println!("Enter {} value: ", mode);
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: f32 = value.trim().parse().expect("Please type a valid value!");
    let converted: f32;

    if mode == "F" {
        converted = (value - 32.0) * 5.0 / 9.0;
        println!("{} F = {} C", value, converted);
    } else {
        converted = 9.0 / 5.0 * value + 32.0;
        println!("{} C = {} F", value, converted);
    }
}