use std::io::{self, Write};

fn main() {
    print!("Enter a string: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let reversed = input.trim().chars().rev().collect::<String>();
    println!("Reversed string: {}", reversed);
}
