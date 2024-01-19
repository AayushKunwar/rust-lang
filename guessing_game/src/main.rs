use std::io;

fn main() {
    println!("Gess the number!");

    println!("Please input your gues.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
