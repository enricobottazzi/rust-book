use std::io;

fn main() {

    println!("Please input the type of temperature you want to convert (C or F):");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input = input.trim();

    if input == "C" {
        println!("Please input the temperature in Celsius:");
        let celsius = read_temperature();
        let celsius = parse_temperature(celsius);
        let fahrenheit = convert_to_fahrenheit(celsius);
        println!("{}°C is {}°F", celsius, fahrenheit);
    } else if input == "F" {
        println!("Please input the temperature in Fahrenheit:");
        let fahrenheit = read_temperature();
        let fahrenheit = parse_temperature(fahrenheit);
        let celsius = convert_to_celsius(fahrenheit);
        println!("{}°F is {}°C", fahrenheit, celsius);
    } else {
        println!("Please type C or F!");
    }

}

fn read_temperature() -> String {
    let mut fahrenheit = String::new();
    io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Failed to read line");
    fahrenheit
}

fn parse_temperature(temperature: String) -> f64 {
    temperature.trim().parse().expect("Please type a number!")
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
