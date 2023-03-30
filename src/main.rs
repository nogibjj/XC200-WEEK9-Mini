use rand::Rng;
use std::io;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number between 1 and 100!");

    loop {
        // Get a guess from the user
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Parse the guess as an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Check if the guess is correct
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
