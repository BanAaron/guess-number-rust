use std::io;

fn main() {
    println!("Guess the number!");
    // mut means mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    print!("hello");
}