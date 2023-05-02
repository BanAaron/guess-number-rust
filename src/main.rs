use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // generate a random number between 1 and up to 10.
    let random_number: u32 = rand::thread_rng().gen_range(1..=10);

    // request a number from the user
    println!("Guess a number:");

    // loop is an infinite loop. While loops need a condition. Loop does not require any condition
    loop {
        // we will store the number in guess
        let mut guess: String = String::new();
        // we use stdin() to capture the input from the command line
        io::stdin()
            // read_line() is what captures the line
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // convert guess to an int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a number");
                continue;
            }
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

    let mut exit = String::new();
    // capture an input so the program doesn't just exit immediately
    println!("Press enter to exit");
    io::stdin().read_line(&mut exit).expect("nothing");
}
