use std::io;

fn main() {
    loop {
        // Ask the user which conversion they want to perform
        println!("Select a conversion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().parse::<i32>().unwrap();

        // Ask the user for the temperature to convert
        println!("Enter the temperature:");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read line");
        let temperature = temperature.trim().parse::<f64>().unwrap();

        // Perform the conversion
        let converted_temperature = match choice {
            1 => temperature * 1.8 + 32.0,
            2 => (temperature - 32.0) / 1.8,
            _ => continue,
        };

        // Display the result
        let from_unit = if choice == 1 { "C" } else { "F" };
        let to_unit = if choice == 1 { "F" } else { "C" };
        println!("{}{} = {}{}", temperature, from_unit, converted_temperature, to_unit);
    }
}
