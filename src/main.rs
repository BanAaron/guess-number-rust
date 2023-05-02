use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // generate a random number between 1 and up to 100.
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    // request a number from the user
    println!("Guess a number.");

    // loop is an infinite loop. While loops need a condition. Loop does not require any condition
    loop {
        // we will store the number in guess
        let mut guess: String = String::new();
        // we use stdin() to capture the input from the command line
        io::stdin()
            // read_line() is what captures the line
            .read_line(&mut guess)
            .expect("Failed to read line.");

        println!("You guessed {guess}");

        // convert guess to an int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
