use std::io;

fn main() {
    let mut option = String::new();

    println!("Select an option: ");
    println!("1. Fahrenheit to Celsius (default)");
    println!("2. Celsius to Fahrenheit");

    io::stdin().read_line(&mut option).expect("Failed to read");

    let option: i32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    println!("\nIntroduce the degrees (default: 32): ");
    let mut degrees = String::new();

    io::stdin().read_line(&mut degrees).expect("Failed to read");

    let degrees: f32 = match degrees.trim().parse() {
        Ok(num) => num,
        Err(_) => 32.0,
    };

    if option == 1 {
        println!("{}F = {}C", degrees, fahren_to_cels(degrees));
    } else {
        println!("{}C = {}F", degrees, cels_to_fahren(degrees));
    }
}

// Functions

fn fahren_to_cels(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

fn cels_to_fahren(x: f32) -> f32 {
    (x * 9.0) / 5.0 + 32.0
}
